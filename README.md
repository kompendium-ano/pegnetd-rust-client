# Pegnetd Client

A JSON-RPC Rust client for the PegNet.

## Usage
In cargo.toml:
```toml
pegnetd = "0.1.2"
```

## [Documentation](https://docs.rs/pegnetd)

## Quickstart
```rust
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  let api = Pegnetd::open_node();
  let response = sync_status(&api).await;
  assert!(response.result.syncheight > 0);
  Ok(())
}
```

## Configuration
```rust
// Local Node Configuration
// http://localhost:8070/v1
let api = Pegnetd::new();

// Public Node Configuration
// https://api.pegnetd.com
let api = Pegnetd::open_node();

// Custom Node Configuration
let api = Pegnetd::custom_node("http://192.168.1.42:8070/v1");
```

## Learn
- [Accessing the Factom blockchain from different programming languages](https://medium.com/kompendium-developments/accessing-factom-blockchain-from-different-programming-languages-7f09030efe16)
- [Building simple blockchain game with Factom](https://medium.com/kompendium-developments/accessing-factom-blockchain-from-different-programming-languages-7f09030efe16)

## Contributions

The Library developed by Kompendium, LLC in partnership with [Mitchell Berry](https://github.com/MitchellBerry), [Sergey Bushnyak](https://sigrlami.eu), [Kelecorix, Inc](https://kelecorix.com) and  for the good of the Factom community. While we see the usage of the libraries over the months, the Factom community decided not to support this work in the form of a grant either upfront or as backpay on multiple occasions ([1](https://factomize.com/forums/threads/kompendium-12-back-pay-two-factom-community-sdks-client-libraries-php-ruby.4802/), [2](https://factomize.com/forums/threads/kompendium-12-back-pay-ruby-haskell-client-libraries-for-the-factom-blockchain.2740/), [3](https://factomize.com/forums/threads/back-pay-development-of-4-json-rpc-client-libraries-to-the-factom-community.2513/))

If you're an active user or find it useful we strongly encourage you to support our efforts and ensure long maintenance by contributing a small donation to one of the following cryptocurrency addresses:

BTC: 39oVXpsgsyW8ZgzsnX3sV7HLdtXWfT96qN

ETH: 0x9cDBA6bb44772259B3A3fb89cf233A147a720f34

FCT: FA38cwer93mmPw1HxjScLmK1yF9iJTu5P87T2vdkbuLovm2YXyss
