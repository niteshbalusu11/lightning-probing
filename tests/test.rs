#[cfg(test)]
mod tests {
    use anyhow::Context;
    use dotenv::dotenv;
    use lightning_probing::probe_destination;
    use lightning_probing::ProbeDestination;
    use lnd_grpc_rust::lnrpc::GetInfoRequest;
    use std::env;

    #[tokio::test]
    async fn test_probe_destination() -> anyhow::Result<()> {
        dotenv().ok();
        // Set the log level to "debug"

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
            probe_amount: Some(100000),
            destination_pubkey: Some(
                "0364913d18a19c671bb36dd04d6ad5be0fe8f2894314c36a9db3f03c2d414907e1".to_string(),
            ),
            timeout_seconds: Some(300),
            fee_limit_sat: 1000,
            payment_request: None,
        };

        // Call probe_destination and check the result
        let result = probe_destination(data);
        assert!(result.await.is_ok());

        Ok(())
    }
}
