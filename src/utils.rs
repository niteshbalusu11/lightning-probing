use std::collections::HashMap;

use anyhow::Context;
use lnd_grpc_rust::{
    lnrpc::{self, Feature},
    LndClient,
};
use rand::Rng;
use sha2::{Digest, Sha256};

const FEATURE_TYPE_CHANNEL_TYPE: u32 = 45;
const FEATURE_TYPE_TRUSTED_FUNDING: u32 = 51;

pub fn generate_secret_for_probes() -> Vec<u8> {
    // Generate a random 32-byte secret
    let mut secret = [0u8; 32];
    rand::thread_rng().fill(&mut secret);

    // Hash the secret using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&secret);
    let hash = hasher.finalize().to_vec();

    hash
}

pub async fn get_node_info(
    client: &mut LndClient,
    destination: String,
) -> anyhow::Result<lnrpc::NodeInfo> {
    let request = lnrpc::NodeInfoRequest {
        pub_key: destination.clone(),
        include_channels: false,
        ..Default::default()
    };

    let node_info = client
        .lightning()
        .get_node_info(request)
        .await
        .context("Failed to get node info")?
        .into_inner();

    Ok(node_info)
}

pub fn get_node_features(features: HashMap<u32, Feature>) -> Vec<i32> {
    let features: Vec<i32> = features
        .into_iter()
        .filter(|(k, _)| *k != FEATURE_TYPE_CHANNEL_TYPE && *k != FEATURE_TYPE_TRUSTED_FUNDING)
        .map(|(k, _)| k as i32)
        .collect();

    features
}
