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
    async fn test_probe_destination() -> anyhow::Result<()> {
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
            probe_amount_sat: Some(3000),
            destination_pubkey: Some(
                "033a8f92bb6ed087e13270ffbbfe65dcd9c4531d9f63f01ae481ec6257df97f4cf".to_string(),
            ),
            timeout_seconds: Some(20),
            fee_limit_sat: 1000,
            payment_request: None,
            // outgoing_pubkeys: Some(vec![
            //     "035e4ff418fc8b5554c5d9eea66396c227bd429a3251c8cbc711002ba215bfc226".to_string(),
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
