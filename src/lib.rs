use anyhow::{bail, Context};
use lnd_grpc_rust::{
    lnrpc::{payment::PaymentStatus, FeatureBit, PaymentFailureReason},
    routerrpc, LndClient,
};
use log::{error, info, warn};
use std::{env, result::Result::Ok};
mod utils;
use utils::generate_secret_for_probes;

const TLV_ONION_REQ: i32 = FeatureBit::TlvOnionReq as i32;
const DEFAULT_TIMEOUT_SECONDS: i32 = 300;
const DEFAULT_PROBE_AMOUNT: i64 = 1;

pub struct ProbeDestination {
    pub client: LndClient,
    pub probe_amount: Option<i64>,
    pub destination_pubkey: Option<String>,
    pub timeout_seconds: Option<i32>,
    pub fee_limit_sat: i64,
    pub payment_request: Option<String>,
}

pub async fn probe_destination(mut args: ProbeDestination) -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "info");

    pretty_env_logger::init();

    if args.payment_request.is_none() && args.destination_pubkey.is_none() {
        bail!("ExpectedEitherPaymentRequestOrDestinationPubkey");
    }

    let hash = generate_secret_for_probes();

    let request = routerrpc::SendPaymentRequest {
        amt: args.probe_amount.unwrap_or(DEFAULT_PROBE_AMOUNT),
        dest: hex::decode(args.destination_pubkey.unwrap_or_default()).unwrap_or_default(),
        dest_features: vec![TLV_ONION_REQ],
        payment_hash: hash,
        timeout_seconds: args.timeout_seconds.unwrap_or(DEFAULT_TIMEOUT_SECONDS),
        fee_limit_sat: args.fee_limit_sat,
        payment_request: args.payment_request.unwrap_or_default(),

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
            info!("probing: {:?}", payment.htlcs);

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
