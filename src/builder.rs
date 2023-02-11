// 🐻‍❄️📦 charted_sdk: Rust SDK library for Noelware's Charts Platform
// Copyright (c) 2022-2023 Noelware, LLC. <team@noelware.org>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::collections::HashMap;

use reqwest::Client;

use crate::{auth::AuthStrategy, APIClient};

/// Represents the builder for creating [api clients].
///
/// ```no_run
/// # use charted::APIClientBuilder;
/// #
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = APIClientBuilder::default()
///   .base_url("http://localhost:3651")
///   .build()?;
///
/// client.health().await?;
/// // => Ok(())
/// # }
/// ```
///
/// [api clients]: struct.APIClient.html
#[derive(Debug)]
pub struct APIClientBuilder {
    pub(crate) auth_strategy: Option<Box<dyn AuthStrategy>>,
    pub(crate) http_client: Client,
    pub(crate) base_url: Option<String>,
    pub(crate) headers: HashMap<String, String>,
}

impl Default for APIClientBuilder {
    fn default() -> Self {
        APIClientBuilder {
            auth_strategy: None,
            http_client: Client::new(),
            base_url: None,
            headers: HashMap::new(),
        }
    }
}

impl APIClientBuilder {
    /// Sets an authentication strategy to use when faciliating API calls to charted-server.
    pub fn auth_strategy<S: AuthStrategy + 'static>(&mut self, strategy: S) -> &mut Self {
        self.auth_strategy = Some(Box::new(strategy));
        self
    }

    /// Sets the inner [`Client`] to use when faciliating API calls to charted-server.
    pub fn http_client(&mut self, client: Client) -> &mut Self {
        self.http_client = client;
        self
    }

    /// Extra list of HTTP headers to append when creating requests.
    pub fn headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        self.headers = self.headers.clone().into_iter().chain(headers).collect();
        self
    }

    pub fn base_url<S: Into<String>>(&mut self, base_url: S) -> &mut Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn build(self) -> APIClient {
        APIClient::new_with_builder(self)
    }
}
