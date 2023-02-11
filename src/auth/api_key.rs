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

use super::AuthStrategy;

/// Represents the strategy to use an [API key](https://charts.noelware.org/docs/server/current/api/reference#api-keys) when
/// faciliating calls to **charted-server**. This is the most recommended way to use authenticated endpoints as yourself or
/// for automated bots.
#[derive(Debug, Clone)]
pub struct ApiKeyStrategy(String);

impl AuthStrategy for ApiKeyStrategy {
    fn prefix(&self) -> String {
        "ApiKey".to_owned()
    }

    fn value(&self) -> String {
        self.0.clone()
    }
}
