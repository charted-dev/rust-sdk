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

use std::sync::Arc;

use reqwest::{Body, Method};

use crate::{
    models::{APIResponse, ChartIndexYaml, Empty},
    APIClient, Error, Result,
};

/// Represents a container for requesting to the `/indexes` REST handler.
#[derive(Debug, Clone)]
pub struct IndexesContainer {
    client: Arc<APIClient>,
}

impl IndexesContainer {
    /// Creates a new [`IndexesContainer`] with the specified [`APIClient`]
    ///
    /// [`APIClient`]: struct.APIClient.html
    pub(crate) fn new(client: APIClient) -> IndexesContainer {
        IndexesContainer {
            client: Arc::new(client),
        }
    }

    /// Gets a organization or user's `index.yaml` with the specified snowflake. This can return
    /// the `Option::None` variant if the server doesn't use the Docker Registry feature.
    ///
    /// ## Example
    /// ```no_run
    /// # use charted::APIClient;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #   let client = APIClient::default();
    /// client.indexes().get_by_id(123).await?;
    /// // => Ok(charted::models::ChartIndexYaml { ... })
    /// # }
    /// ```
    pub async fn get_by_id(&self, id: u64) -> Result<Option<ChartIndexYaml>> {
        match self
            .client
            .request_yaml::<ChartIndexYaml, Body, String>(format!("/indexes/{id}"), Method::GET, None, None)
            .await
        {
            Ok(res) => Ok(Some(res)),
            Err(e) => match e {
                // This occurs if there is a database error or if the chart index was not found
                Error::YamlSerialization { payload, .. } => {
                    let serialized: APIResponse<Empty> =
                        serde_json::from_str(&payload).map_err(|e| Error::JsonSerialization { error: e, payload })?;

                    assert!(!serialized.success);
                    let errors = serialized.errors.unwrap();
                    if let Some(err) = errors.first() {
                        if err.code.as_str() == "REST_HANDLER_NOT_FOUND" {
                            return Ok(None);
                        }
                    }

                    Err(Error::APIServer { errors })
                }
                _ => Err(e),
            },
        }
    }

    /// Gets a organization or user's `index.yaml` with the user or organization's name. This can return
    /// the `Option::None` variant if the server doesn't use the Docker Registry feature.
    ///
    /// ## Example
    /// ```no_run
    /// # use charted::APIClient;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #   let client = APIClient::default();
    /// client.indexes().get("noel").await?;
    /// // => Ok(charted::models::ChartIndexYaml { ... })
    /// # }
    /// ```
    pub async fn get<S: Into<String>>(&self, name: S) -> Result<Option<ChartIndexYaml>> {
        match self
            .client
            .request_yaml::<ChartIndexYaml, Body, String>(format!("/indexes/{}", name.into()), Method::GET, None, None)
            .await
        {
            Ok(res) => Ok(Some(res)),
            Err(e) => match e {
                // This occurs if there is a database error or if the chart index was not found
                Error::YamlSerialization { payload, .. } => {
                    let serialized: APIResponse<Empty> =
                        serde_json::from_str(&payload).map_err(|e| Error::JsonSerialization { error: e, payload })?;

                    assert!(!serialized.success);
                    let errors = serialized.errors.unwrap();
                    if let Some(err) = errors.first() {
                        if err.code.as_str() == "REST_HANDLER_NOT_FOUND" {
                            return Ok(None);
                        }
                    }

                    Err(Error::APIServer { errors })
                }
                _ => Err(e),
            },
        }
    }
}
