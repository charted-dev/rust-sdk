# 🐻‍❄️📦 Rust SDK for Noelware's Charts Platform
> *Rust SDK library for Noelware's Charts Platform*

This repository holds the official SDK bindings for Noelware's Charts Platform. This was made to make API requests easier with the [Helm Plugin](https://charts.noelware.org/docs/helm-plugin/current) that is made in Rust.

## Usage
```rs
use charted_sdk::AuthStrategy;
use charted_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let auth = AuthStrategy::Basic("username".into(), "password".into());
  let client = Client::builder()
    .base_url("https://charts.noelware.org/api".into())
    .auth_strategy(auth)
    .http_builder(move |builder| {
      builder.user_agent("some user agent here");
    }).build()?;

  let noel = client.users("noel").get().await?;
  // => charted_sdk::UserBindings

  let noel_repos = noel.repositories().all().await?;
  // => Vec<charted_sdk::RepositoryBindings>
}
```

## License
**charted_sdk** is released under the MIT License with love by Noelware. <3
