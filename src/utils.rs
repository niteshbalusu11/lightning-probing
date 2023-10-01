use rand::Rng;
use sha2::{Digest, Sha256};

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
