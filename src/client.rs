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

use std::{collections::HashMap, fmt::Debug, str::FromStr};

use log::*;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION},
    Body, Client, Method, Request,
};
use serde::de::DeserializeOwned;

use crate::{
    auth::AuthStrategy,
    containers::IndexesContainer,
    models::{APIResponse, FeaturesResponse, InfoResponse, MainResponse},
    APIClientBuilder, Result,
};

/// Represents the client for creating API requests for [charted-server](https://charts.noelware.org/docs/server/latest). This
/// is the entrypoint to faciliate API calls to charted-server.
///
/// ### Examples
/// ```no_run
/// # use charted::APIClient;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = APIClient::default();
/// client.repositories("charted/server").get().await?;
/// // => Vec<charted::models::Repository>
/// # }
/// ```
///
/// ### Authentication Strategies
/// #### Basic Authentication
/// **charted-server** supports basic authentication, which means you can pass in a user
/// and password and the server will check if the user is valid or not to make the request. You can use
/// the `BasicAuthStrategy` struct to use a user and password combination. This is not the recommended
/// strategy to use in a real-world application since it will expose every permission that the user
/// has on any repository or organization, use the `ApiKeyStrategy` to use an API key to limit permissions
/// on destructive actions.
///
/// ```no_run
/// # use charted::APIClient;
/// use charted::auth::BasicAuthStrategy;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = APIClient::default_with_auth(("username", "password").into());
/// client.users("@me").await?;
/// // => Ok(ApiResponse { success: true, data: Some(charted::models::User { ... }), errors: None })
/// # }
/// ```
///
/// #### Session Token Authentication
/// This will use a JWT session token that is used with the web UI to authenticate a user. This is by far
/// not the recommended strategy to use since it'll leak all permissions a user has on a repository or
/// organization, please use the `ApiKeyStrategy` to use an API key to limit permission on destructive
/// actions.
///
/// ```no_run
/// # use charted::APIClient;
/// use charted::auth::SessionTokenStrategy;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = APIClient::default_with_auth(SessionTokenStrategy::new("access token"));
/// client.users("@me").await?;
/// // => Ok(ApiResponse { success: true, data: Some(charted::models::User { ... }), errors: None })
/// # }
/// ```
///
/// ### API Key Authentication
/// This is the most recommended strategy to use in a real-world application that is making requests to **charted-server**. This
/// will use the API key created by a user to interact with the API server.
///
/// ```no_run
/// # use charted::APIClient;
/// use charted::auth::ApiKeyStrategy;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = APIClient::default_with_auth(ApiKeyStrategy::new("access token"));
/// client.users("@me").await?;
/// // => [if the `users.view` permission is enabled]: Ok(ApiResponse { success: true, data: Some(charted::models::User { ... }), errors: None })
/// # }
/// ```
#[derive(Debug)]
pub struct APIClient {
    auth_strategy: Option<Box<dyn AuthStrategy>>,
    http_client: Client,
    base_url: String,
    headers: Option<HashMap<String, String>>,
}

impl Default for APIClient {
    fn default() -> Self {
        let client = Client::builder()
            .http1_only() // charted-server doesn't support HTTP/2
            .user_agent(format!(
                "Noelware/charted-rust-sdk (+https://github.com/charted-dev/rust-sdk; v{}",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .unwrap();

        APIClient {
            auth_strategy: None,
            http_client: client,
            base_url: "https://charts.noelware.org/api".into(),
            headers: None,
        }
    }
}

impl APIClient {
    pub(crate) fn new_with_builder(builder: APIClientBuilder) -> APIClient {
        let base_url = if let Some(url) = builder.base_url {
            url
        } else {
            "https://charts.noelware.org/api".into()
        };

        APIClient {
            auth_strategy: builder.auth_strategy,
            http_client: builder.http_client,
            base_url,
            headers: Some(builder.headers),
        }
    }

    /// Creates a new [`APIClient`] with a base URL to use. If you plan to use the official
    /// instance hosted by Noelware, please use [`APIClient::default`] instead.
    ///
    /// ```no_run
    /// # use charted::APIClient;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = APIClient::default_with_url("http://localhost:3651");
    /// client.health().await?;
    /// // => Ok(())
    /// # }
    /// ```
    ///
    /// [`APIClient::default`]: https://doc.rust-lang.org/std/default/trait.Default.html#tymethod.default
    pub fn default_with_url<S: Into<String>>(base_url: S) -> APIClient {
        APIClient {
            base_url: base_url.into(),
            ..Default::default()
        }
    }

    /// Creates a new [`APIClient`] with an authentication strategy but uses the default
    /// base URL. To customize every input of the [`APIClient`] struct, use the [`APIClient::builder()`] method.
    ///
    /// ```no_run
    /// # use charted::APIClient;
    /// # use charted::auth::BasicAuthStrategy;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let auth: BasicAuthStrategy = ("someuser", "somepassword").into();
    /// let client = APIClient::default_with_auth(auth);
    ///
    /// client.me().get().await?;
    /// // => Ok(ApiResponse { success: true, data: Some(charted::models::User { ... }), errors: None })
    /// # }
    /// ```
    ///
    /// [`APIClient::builder()`]: struct.APIClient.html#tymethod.builder
    pub fn default_with_auth<S: AuthStrategy + 'static>(auth: S) -> APIClient {
        APIClient {
            auth_strategy: Some(Box::new(auth)),
            ..Default::default()
        }
    }

    /// Sends the `GET /health` request to the API server. This method returns a Result<()>, where
    /// the `Result::Ok` variant is returned if the request was a success AND the data was "Ok", otherwise,
    /// an `Result:Err` variant will occur.
    pub async fn health(&self) -> Result<()> {
        let resp = self
            .request_text::<Body, &str>("/health", Method::GET, None, None)
            .await?;

        if resp == "OK" {
            Ok(())
        } else {
            Err(crate::Error::String(format!("Expected: OK | Received: {resp}")))
        }
    }

    /// Sends the `GET /` request to the API server. Returns the [`MainResponse`] if the request was successful,
    /// which will always will be unless a server error occurs.
    pub async fn main(&self) -> Result<MainResponse> {
        let resp = self
            .request_json::<APIResponse<MainResponse>, Body, &str>("/", Method::GET, None, None)
            .await?;

        if !resp.success {
            return Err(crate::Error::APIServer {
                errors: resp.errors.unwrap(),
            });
        }

        Ok(resp.data.unwrap())
    }

    /// Sends the `GET /info` request to the API server. Returns the [`InfoResponse`] if the request was successful,
    /// which will always will be unless a server error occurs.
    pub async fn info(&self) -> Result<InfoResponse> {
        let resp = self
            .request_json::<APIResponse<InfoResponse>, Body, &str>("/info", Method::GET, None, None)
            .await?;

        if !resp.success {
            return Err(crate::Error::APIServer {
                errors: resp.errors.unwrap(),
            });
        }

        Ok(resp.data.unwrap())
    }

    /// Sends the `GET /features` request to the API server. Returns the [`FeaturesResponse`] if the request was successful,
    /// which will always will be unless a server error occurs.
    pub async fn features(&self) -> Result<FeaturesResponse> {
        let resp = self
            .request_json::<APIResponse<FeaturesResponse>, Body, &str>("/features", Method::GET, None, None)
            .await?;

        if !resp.success {
            return Err(crate::Error::APIServer {
                errors: resp.errors.unwrap(),
            });
        }

        Ok(resp.data.unwrap())
    }

    /// Performs a REST request where the response type returns just a String of the payload.
    /// This method is only used for the Health API or the Prometheus Metrics API.
    pub(crate) async fn request_text<B: Into<Body>, E: AsRef<str>>(
        &self,
        endpoint: E,
        method: Method,
        body: Option<B>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<String> {
        let endpoint_to_use = format!("{}{}", self.base_url, endpoint.as_ref());
        let req = self.create_request(endpoint, method.clone(), body, headers)?;

        match self.http_client.execute(req).await {
            Ok(res) => {
                let status = res.status();
                let bytes: &[u8] = &res.bytes().await.map_err(|e| crate::Error::Unknown(Box::new(e)))?;
                let slice = String::from_utf8_lossy(bytes).to_string();

                trace!("[{} {}] -> {}", method, endpoint_to_use, status);
                trace!("{}", slice);
                Ok(slice)
            }

            Err(e) => Err(crate::Error::Reqwest(e)),
        }
    }

    /// Performs a request where the response type will always be JSON, unless a [`Error::JsonSerialization`] error occurs. This
    /// method is used in almost all requests except the Health API, Prometheus Metrics API, and YAML-specific endpoints (like
    /// user/organization Helm indexes)
    pub(crate) async fn request_json<U: DeserializeOwned + Debug, B: Into<Body>, E: AsRef<str>>(
        &self,
        endpoint: E,
        method: Method,
        body: Option<B>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<U> {
        self.request_text(endpoint, method, body, headers).await.map(|data| {
            serde_json::from_str(data.as_str()).map_err(|e| crate::Error::JsonSerialization {
                error: e,
                payload: data,
            })
        })?
    }

    /// Performs a request where the response type will be deserialized from a YAML-body encoding, unless a [`Error::YamlSerialization`] error
    /// occurs. This method is used in user and organization Helm indexes and any specific repository templates, `Chart.yaml`, or `index.yaml`
    /// files from the release.
    pub(crate) async fn request_yaml<U: DeserializeOwned + Debug, B: Into<Body>, E: AsRef<str>>(
        &self,
        endpoint: E,
        method: Method,
        body: Option<B>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<U> {
        self.request_text(endpoint, method, body, headers).await.map(|data| {
            serde_yaml::from_str(data.as_str()).map_err(|e| crate::Error::YamlSerialization {
                error: e,
                payload: data,
            })
        })?
    }

    // Internal method to create a Request to not repeat code
    fn create_request<B: Into<Body>, E: AsRef<str>>(
        &self,
        endpoint: E,
        method: Method,
        body: Option<B>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Request> {
        let endpoint_to_use = format!("{}{}", self.base_url, endpoint.as_ref());
        trace!("creating request [{} {}]", method, endpoint_to_use);

        let request = self.http_client.request(method, endpoint_to_use);
        if let Some(auth) = &self.auth_strategy {
            let (prefix, value) = (auth.prefix(), auth.value());
            let _ = request
                .try_clone()
                .map(|r| r.header(AUTHORIZATION, format!("{prefix} {value}")));
        }

        let mut headers_to_use = HeaderMap::new();
        if let Some(h) = headers {
            for (key, value) in h {
                headers_to_use.insert(
                    HeaderName::from_str(key.clone().as_str()).map_err(|e| crate::Error::Unknown(Box::new(e)))?,
                    HeaderValue::from_str(value.clone().as_str()).map_err(|e| crate::Error::Unknown(Box::new(e)))?,
                );
            }
        }

        if let Some(h) = &self.headers {
            for (key, value) in h {
                headers_to_use.insert(
                    HeaderName::from_str(key.clone().as_str()).map_err(|e| crate::Error::Unknown(Box::new(e)))?,
                    HeaderValue::from_str(value.clone().as_str()).map_err(|e| crate::Error::Unknown(Box::new(e)))?,
                );
            }
        }

        let _ = request.try_clone().map(|f| f.headers(headers_to_use));
        if let Some(b) = body {
            let _ = request.try_clone().map(|f| f.body::<B>(b));
        }

        request.build().map_err(crate::Error::Reqwest)
    }
}

impl APIClient {
    /// Creates a container to request to the Indexes API.
    pub fn indexes(self) -> IndexesContainer {
        IndexesContainer::new(self)
    }
}
