// 🐻‍❄️📦 charted_sdk: Rust SDK library for Noelware's Charts Platform
// Copyright (c) 2022-2023 Noelware Team <team@noelware.org>
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

use std::{collections::HashMap, str::FromStr, fmt::Debug};

use reqwest::{Client as RClient, Result, Body, Method, header::{HeaderMap, HeaderName, HeaderValue}};
use serde::de::DeserializeOwned;

/// Represents the API client that is used to send requests to the API server.
#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    inner: RClient
}

impl Default for Client {
    fn default() -> Self {
        Client::new("https://charts.noelware.org/api").expect("Unable to build API client")
    }
}

impl Client {
    pub fn new<S: AsRef<str>>(base_url: S) -> Result<Client> {
        Ok(Client {
            base_url: base_url.as_ref().to_string(),
            inner: RClient::builder()
                .http1_only() // charted-server doesn't support HTTP/2 requests
                .user_agent(
                    format!("Noelware/charted-sdk-rust (v{}; https://github.com/charted-dev/rust-sdk)", env!("CARGO_PKG_VERSION"))
                )
                .build()?
        })
    }

    pub(crate) async fn request<U: DeserializeOwned + Debug, T: Into<Body>, E: AsRef<str>>(
        &self,
        endpoint: E,
        method: Method,
        body: Option<T>,
        headers: Option<HashMap<String, String>>
    ) -> Result<()> {
        let endpoint_uri = format!("{}{}", self.base_url, endpoint.as_ref());
        trace!("<- {} {}", method, format!("{}{}", self.base_url, endpoint.as_ref()));

        let request = self.inner.request(method, endpoint_uri);
        if let Some(map) = headers {
            let mut headers_map = HeaderMap::new();
            for (key, value) in map {
                let name = HeaderName::from_str(key.clone().as_str()).unwrap();
                let value = HeaderValue::from_str(value.clone().as_str()).unwrap();

                headers_map.insert(name, value);
            }


            let _ = request.try_clone().map(|f| f.headers(headers_map));
        }

        if let Some(b) = body {
            let _ = request.try_clone().map(|f| Some(f.body::<T>(b)));
        }

        let _req = request.build().expect("unable to build request obj");
        Ok(())
    }
}
