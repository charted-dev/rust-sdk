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

use base64::{engine::general_purpose, Engine};

use super::AuthStrategy;

pub struct BasicAuthStrategy {
    username: String,
    password: String,
}

impl BasicAuthStrategy {
    /// Creates a new [`BasicAuthStrategy`] with a username and password.
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> BasicAuthStrategy {
        BasicAuthStrategy {
            username: username.into(),
            password: password.into(),
        }
    }
}

impl AuthStrategy for BasicAuthStrategy {
    fn prefix(&self) -> String {
        "Basic".to_owned()
    }

    fn value(&self) -> String {
        general_purpose::STANDARD.encode(format!("{}:{}", self.username, self.password))
    }
}

impl From<(String, String)> for BasicAuthStrategy {
    fn from((username, password): (String, String)) -> Self {
        BasicAuthStrategy { username, password }
    }
}

impl From<(&str, &str)> for BasicAuthStrategy {
    fn from((username, password): (&str, &str)) -> Self {
        BasicAuthStrategy {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
}
