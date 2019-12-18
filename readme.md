# Pegnetd Client

[![Crates.io](https://img.shields.io/crates/v/pegnetd.svg)](https://crates.io/crates/pegnetd)
[![Released API docs](https://docs.rs/pegnetd/badge.svg)](https://docs.rs/pegnetd)
[![Discord](https://img.shields.io/discord/550312670528798755.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/V6T7mCW)

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

## Contributing
PR's welcome. Fork the library and submit to dev branch. 
By contributing to this library you agree to it being Apache 2.0 licensed 

#### Donations
FCT: `FA2dJL4qbQimfkXjP7jREdm48AjPzdS8rcosfJisG2L465bs1ean`

BTC: `bc1qt0hcaf3w7mgms37zj4jdtaas42vpg9uhdwvm5e4tf2maj6yk6etqjdqqpg`
