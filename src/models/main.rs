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

//! All the responses for the main endpoints (i.e, `/features`) that aren't categorized.

use std::collections::HashMap;

use super::DistributionType;

/// Represents the response object for the `GET /info` REST handler.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct InfoResponse {
    /// The distribution the server is running off from.
    pub distribution: DistributionType,

    /// The commit hash from the Git repository.
    pub commit_hash: String,

    /// The build date, in ISO-8601 format.
    pub build_date: String,

    /// The product name, which will always be `charted-server`.
    pub product: String,

    /// The version of the server
    pub version: String,

    /// The vendor that maintains this project, will always be Noelware if
    /// pulled from official sources.
    pub vendor: String,
}

/// Represents the response object for the `GET /` REST handler.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MainResponse {
    /// The message, which will always be "Hello, world!"
    pub message: String,

    /// You know, for Helm charts?
    pub tagline: String,

    /// Documentation URI.
    pub docs: String,
}

/// Represents the response object for the `GET /features` REST handler.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct FeaturesResponse {
    /// Whether if the server is using a local Docker registry.
    pub docker_registry: bool,

    /// Whether if the server is invite-only.
    pub is_invite_only: bool,
    pub registrations: bool,
    pub integrations: HashMap<String, bool>,
    pub audit_logs: bool,
    pub webhooks: bool,
    pub search: bool,
}
