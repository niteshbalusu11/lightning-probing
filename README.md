# lightning-probing

A package to probe the lightning network.

## Installation

- Install Rust https://www.rust-lang.org/

- Set env variables for running tests
```
# Update as necessary

cp .env.example .env
```

- Build
```
cargo build
```

- Test
```
# To test with log printing
cargo test -- --nocapture
```

- For using the repo, here's what it takes as inputs and outputs.

```rust
// For examples check the tests folder

// @Input

// For LndClient use https://github.com/yzernik/tonic_openssl_lnd 
// OR my fork https://github.com/niteshbalusu11/lnd-grpc-rust 

pub struct ProbeDestination {
    pub client: LndClient,
    pub probe_amount_sat: Option<i64>,
    pub destination_pubkey: Option<String>,
    pub timeout_seconds: Option<i32>,
    pub fee_limit_sat: i64,
    pub payment_request: Option<String>,
    pub outgoing_pubkeys: Option<Vec<String>>,
    pub last_hop_pubkey: Option<String>,
    pub max_paths: Option<u32>,
}


// @Returns

#[derive(Debug)]
pub struct ReturnValue {
    pub payment: lnrpc::Payment,
    pub is_probe_success: bool,
    pub failure_reason: FailureReason,
}
```

# License
MIT

