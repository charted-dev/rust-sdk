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

mod api_key;
mod basic;
mod session_token;

use std::fmt::{Debug, Formatter, Result};

pub use api_key::*;
pub use basic::*;
pub use session_token::*;

/// The trait for implementing an authentication strategy. It is not recommended to build
/// your own strategy unless you have forked [charted-server](https://github.com/charted-dev/charted)
/// and you wish to use your own authentication strategy, then that's completely ok.
pub trait AuthStrategy {
    /// Represents the prefix to use when building the `Authorization` header.
    fn prefix(&self) -> String;

    /// The header value to use.
    fn value(&self) -> String;
}

impl Debug for dyn AuthStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "dyn AuthStrategy")?;
        Ok(())
    }
}
