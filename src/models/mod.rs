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

mod main;
mod response;

pub use main::*;
pub use response::*;

/// Represents the charted-server distribution type that it was built from.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DistributionType {
    /// Distribution type that is running on a Kubernetes cluster, most likely using the charted-server
    /// Helm chart or Noel on Cloud Kubernetes Operator
    #[serde(rename = "kubernetes")]
    Kubernetes,

    /// Distribution type that is unknown, might not recommend touching this one!
    #[serde(rename = "unknown")]
    Unknown,

    /// Distribution type that is using the **charted-server** Docker image from Noelware or GitHub's
    /// container registry.
    #[serde(rename = "docker")]
    Docker,

    /// Distribution type that represents the server being pulled from systems that use
    /// the RPM package distribution.
    #[serde(rename = "rpm")]
    RPM,

    /// Distribution type that represents the server being pulled from systems that use
    /// the Debian package distribution (Ubuntu/Debian-based systems).
    #[serde(rename = "deb")]
    Deb,

    /// Distribution type that represents the server being ran from the [GitHub repository](https://github.com/charted-dev/charted),
    /// or built locally.
    #[serde(rename = "git")]
    Git,
}

#[allow(clippy::derivable_impls)]
impl Default for DistributionType {
    fn default() -> Self {
        DistributionType::Unknown
    }
}
