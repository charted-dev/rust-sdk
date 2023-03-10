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

//! ## charted-server for Rust
//!
//!> *Rust SDK library for Noelware's Charts Platform*
//!
//! The **charted** crate is used to faciliate API calls to [charted-server](https://charts.noelware.org/docs/server/current),
//! this library was mainly created for the [Helm plugin](https://github.com/charted-dev/helm-plugin), but made public for everyone
//! to use when consuming the API.
//!
//! Read the [`APIClient`] struct for more information on how to use this struct to make requests
//! to the API server.
//!
//! [`APIClient`]: struct.APIClient.html

pub mod auth;
pub mod models;

mod builder;
mod client;
mod containers;
mod error;

pub use builder::*;
pub use client::*;
pub use error::*;
