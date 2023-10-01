use anyhow::{bail, Context};
use lnd_grpc_rust::{
    lnrpc::{self, payment::PaymentStatus, FeatureBit, PaymentFailureReason},
    routerrpc, LndClient,
};
use log::{error, info, warn};
use std::{env, result::Result::Ok};
mod utils;
use dotenv::dotenv;
use utils::generate_secret_for_probes;

use crate::utils::{get_node_features, get_node_info};

const TLV_ONION_REQ: i32 = FeatureBit::TlvOnionReq as i32;
const DEFAULT_TIMEOUT_SECONDS: i32 = 300;
const ZERO_AMOUNT: i64 = 0;

pub struct ProbeDestination {
    pub client: LndClient,
    pub probe_amount: Option<i64>,
    pub destination_pubkey: Option<String>,
    pub timeout_seconds: Option<i32>,
    pub fee_limit_sat: i64,
    pub payment_request: Option<String>,
}

pub async fn probe_destination(mut args: ProbeDestination) -> anyhow::Result<()> {
    dotenv().ok();

    let log_level = env::var("PROBE_LOG_LEVEL").unwrap_or("info".to_string());

    env::set_var("RUST_LOG", log_level);

    pretty_env_logger::init();

    if args.payment_request.is_none() && args.destination_pubkey.is_none() {
        bail!("ExpectedEitherPaymentRequestOrDestinationPubkey");
    }

    if args.payment_request.is_some() && args.destination_pubkey.is_some() {
        bail!("ExpectedPaymentRequestOrDestinationPubkeyAndNotBoth");
    }

    let mut destination: String = "".to_string();
    let mut features: Vec<i32> = vec![TLV_ONION_REQ];
    let mut amount: i64 = args.probe_amount.unwrap_or_default();

    if args.destination_pubkey.is_some() {
        destination = args.destination_pubkey.clone().unwrap();

        let node_info = get_node_info(&mut args.client, args.destination_pubkey.unwrap())
            .await
            .context("failed to get nodeinfo")?;

        let node_features = node_info.node.context("failed to get node info")?.features;

        features = get_node_features(node_features);
    }

    if let Some(payment_request_string) = args.payment_request {
        let request = lnrpc::PayReqString {
            pay_req: payment_request_string,
        };

        let decoded_payment_request = args
            .client
            .lightning()
            .decode_pay_req(request)
            .await
            .context("FailedToDecodePaymentRequest")?;

        let inner = decoded_payment_request.into_inner();

        if inner.num_satoshis == ZERO_AMOUNT || inner.num_msat == ZERO_AMOUNT {
            bail!("Can't probe with 0 amount invoices")
        }

        amount = inner.num_satoshis;
        features = get_node_features(inner.features);

        destination = inner.destination;
    }

    let hash = generate_secret_for_probes();

    let request = routerrpc::SendPaymentRequest {
        amt: amount,
        dest: hex::decode(destination).context("Failed to decode hex pubkey")?,
        dest_features: features,
        payment_hash: hash,
        timeout_seconds: args.timeout_seconds.unwrap_or(DEFAULT_TIMEOUT_SECONDS),
        fee_limit_sat: args.fee_limit_sat,

        ..Default::default()
    };

    let res = args.client.router().send_payment_v2(request).await;

    let mut response = match res {
        Ok(response) => response.into_inner(),
        Err(e) => bail!(
            "Failed to get streaming response from send payment: {:?}",
            e
        ),
    };

    while let Some(payment) = response.message().await.context("Failed to get payment")? {
        if let Some(status) = PaymentStatus::from_i32(payment.status) {
            // info!("probing: {:?}", payment.htlcs);

            if status == PaymentStatus::Succeeded {
                info!("payment succeeded {:?}", payment.failure_reason());

                return Ok(());
            }

            if status == PaymentStatus::Failed
                && payment.failure_reason()
                    == PaymentFailureReason::FailureReasonIncorrectPaymentDetails
            {
                warn!("Payment failed: {:?}", payment.failure_reason());
                return Ok(());
            }
        } else {
            error!("Unknown payment status {:?}", payment.failure_reason());
        }
    }

    Ok(())
}
