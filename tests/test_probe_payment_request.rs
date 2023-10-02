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
            payment_request: Some("lnbc37300n1pj3n5whpp5u2gw3k7426dshkyd4rg5vvq5f6lqzgvfk27l3n4pujsqjtyd32hsdqqcqzpgxqzpesp5df5m797vrel5r4pkufhnk99fdvzpz6xsllgfwycd0g2fgk6v3gpq9qyyssqc07t2925lqw38hz7t9zfzk0jmw79alnxywn0wu74mwxee30dvw9pwll2xprk8g3l6402wdlw059mqm42z9lqu0m76m9dzq6peuyjmfcp2974l8".to_string()),
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
