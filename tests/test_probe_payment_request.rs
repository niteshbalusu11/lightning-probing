#[cfg(test)]
mod tests {
    use anyhow::Context;
    use dotenv::dotenv;
    use lightning_probing::probe_destination;
    use lightning_probing::ProbeDestination;
    use lnd_grpc_rust::lnrpc::GetInfoRequest;
    use log::error;
    use log::info;
    use std::env;

    #[tokio::test]
    async fn test_probe_payment_request() -> anyhow::Result<()> {
        dotenv().ok();

        let log_level = env::var("PROBE_LOG_LEVEL").unwrap_or("info".to_string());

        env::set_var("RUST_LOG", log_level);

        pretty_env_logger::init();

        let cert = env::var("CERT").context("failed to get cert")?;
        let macaroon = env::var("MACAROON").context("failed to get macaroon")?;
        let socket = env::var("SOCKET").context("failed to get socket")?;

        // Create a client (assuming LndClient has a default implementation)
        let mut client = lnd_grpc_rust::connect(cert, macaroon, socket)
            .await
            .expect("failed to get lnd client");

        // Make sure you are able to connect to lnd on start up
        client
            .lightning()
            .get_info(GetInfoRequest {})
            .await
            .context("Failed to connect to Lnd")?;

        // Create a ProbeDestination struct
        let data = ProbeDestination {
            client,
            probe_amount_sat: None,
            destination_pubkey: None,
            timeout_seconds: Some(30),
            fee_limit_sat: 1000,
            payment_request: Some("lnbcrt500u1pjjkhkhpp5cg6ussfmcycsjfhrdk44ap57whr8s0xckzcpqpdqxm8m59je2yaqdqqcqzzsxqyz5vqsp5ah8zfz8kvuea3xwnpch0rsmg6ky5dln4ex7f5fef4x6zz5ycvj0q9qyyssqntg770uaw0f8etnk06nmfvxwpgsh24ts2z4mlaphznmt9h90nkqn6z2dxl3hzjdwhyq4czrldl3tv698yw43j7wjxscevktaxyf243qqegcx54".to_string()),
            // outgoing_pubkeys: Some(vec![
            //     "03037dc08e9ac63b82581f79b662a4d0ceca8a8ca162b1af3551595b8f2d97b70a".to_string(),
            // ]),
            outgoing_pubkeys: None,
            last_hop_pubkey: None,
            max_paths: None,
        };

        // Call probe_destination and check the result
        let result = probe_destination(data).await;

        match &result {
            Ok(e) => info!("Result is: {:?}", e),
            Err(e) => error!("Error is: {:?}", e),
        }

        assert!(result.is_ok());

        Ok(())
    }
}
