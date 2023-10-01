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
            probe_amount: None,
            destination_pubkey: None,
            timeout_seconds: Some(300),
            fee_limit_sat: 1000,
            payment_request: Some("lnbc10u1pj3ngcdpp5e7898f30qnx793n8vmzsm7aep5nlwfqg6tkkvkdvjymwpugf848sdpv2phhwetjv4jzqcneypqyc6t8dp6xu6twva2xjuzzda6qcqzzsxqrrsssp5xpqjfm2jufpcxk0zgqelcptufurqqjse3rz5dtdega3z7qtch79s9qyyssq6qllysm4sqry9plmvk9ugxyq598y3m7u4dyx38n77ethnm30knfs5xmwufk8knm04xwvr67wp20huz4872800ufx6zy6fqkg22wpatsqzml6rg".to_string()),
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
