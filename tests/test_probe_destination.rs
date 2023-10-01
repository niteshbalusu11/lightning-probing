#[cfg(test)]
mod tests {
    use anyhow::Context;
    use dotenv::dotenv;
    use lightning_probing::probe_destination;
    use lightning_probing::ProbeDestination;
    use lnd_grpc_rust::lnrpc::GetInfoRequest;
    use log::error;
    use std::env;

    #[tokio::test]
    async fn test_probe_destination() -> anyhow::Result<()> {
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
            probe_amount: Some(10000),
            destination_pubkey: Some(
                "02d96eadea3d780104449aca5c93461ce67c1564e2e1d73225fa67dd3b997a6018".to_string(),
            ),
            timeout_seconds: Some(300),
            fee_limit_sat: 1000,
            payment_request: None,
        };

        // Call probe_destination and check the result
        let result = probe_destination(data).await;

        if let Err(e) = &result {
            error!("An error occurred: {:?}", e);
        }

        assert!(result.is_ok());

        Ok(())
    }
}
