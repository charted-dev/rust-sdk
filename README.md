# 🐻‍❄️📦 Rust SDK for charted-server
> *Upcoming, and experimental Rust SDK for charted-server*

This repository holds the official SDK bindings for [charted-server](https://github.com/charted-dev/charted). This was made to make API requests easier with the [Helm Plugin](https://charts.noelware.org/docs/helm-plugin/current) that is made in Rust.

## Usage
```rs
use charted::{auth::BasicAuthStategy, APIClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth: BasicAuthStrategy = ("username", "password").into();
    let client = APIClient::builder()
        .auth_strategy(auth)
        .base_url("http://localhost:3651")
        .build();

    client.health().await?;
    Ok(())
}
```

## License
**charted_sdk** is released under the MIT License with love by Noelware. <3
