use anyhow::{Context, Ok};
use lnd_grpc_rust::{
    lnrpc::{self, Feature},
    LndClient,
};
use rand::Rng;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use crate::constants::FailureCode;
extern crate serde_json;

const FEATURE_TYPE_CHANNEL_TYPE: u32 = 45;
const FEATURE_TYPE_TRUSTED_FUNDING: u32 = 51;

pub(crate) fn generate_secret_for_probes() -> Vec<u8> {
    // Generate a random 32-byte secret
    let mut secret = [0u8; 32];
    rand::thread_rng().fill(&mut secret);

    // Hash the secret using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&secret);
    let hash = hasher.finalize().to_vec();

    hash
}

pub(crate) async fn get_node_info(
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

pub(crate) fn get_node_features(features: HashMap<u32, Feature>) -> Vec<i32> {
    let features: Vec<i32> = features
        .into_iter()
        .filter(|(k, _)| *k != FEATURE_TYPE_CHANNEL_TYPE && *k != FEATURE_TYPE_TRUSTED_FUNDING)
        .map(|(k, _)| k as i32)
        .collect();

    features
}

pub(crate) async fn filter_channels_from_pubkeys(
    client: &mut LndClient,
    pubkeys: Vec<String>,
) -> anyhow::Result<lnrpc::ListChannelsResponse> {
    let request = lnrpc::ListChannelsRequest {
        active_only: true,
        ..Default::default()
    };

    let response = client
        .lightning()
        .list_channels(request)
        .await
        .context("Failed to list channels")?;

    let channels = response
        .into_inner()
        .channels
        .into_iter()
        .filter(|n| pubkeys.contains(&n.remote_pubkey))
        .collect();

    let filtered_response = lnrpc::ListChannelsResponse {
        channels,
        ..Default::default()
    };

    Ok(filtered_response)
}

#[derive(Serialize, Debug)]
pub(crate) struct Hop {
    pub id: u64, // Assuming the type is u64, adjust as necessary
    pub pubkey: String,
}

#[derive(Serialize, Debug)]
pub(crate) struct FailureDetail {
    pub code: FailureCode,
    pub hops: Vec<Hop>,
}
