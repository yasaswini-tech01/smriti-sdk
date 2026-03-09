# Smriti SDK

A Rust SDK for interacting with Smriti services.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
smriti-sdk = "0.1.0"
```

## Usage

```rust
use smriti_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your-api-key")?;
    // Use the client...
    Ok(())
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
