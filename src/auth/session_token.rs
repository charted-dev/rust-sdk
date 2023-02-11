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

/// Represents the session token strategy. This is mainly used if you ***only*** have an access token
/// with a *optional* refresh token.
///
/// ## Examples
/// ```no_run
/// # use charted::APIClient;
/// use charted::auth::SessionTokenStrategy;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// /// This uses the `SessionTokenStrategy` with no refresh token used. If the access token
/// /// expires, then all authenticated calls will return the `Result:Err` variant. If the refresh
/// /// token was passed, then it will actually refresh it but only on the first time, since the
/// /// refresh token used is exhausted.
/// let client = APIClient::default_with_auth(SessionTokenStrategy::new("access token"));
/// client.users("@me").await?;
/// // => Ok(ApiResponse { success: true, data: Some(charted::models::User { ... }), errors: None })
/// # }
/// ```
///
#[derive(Debug, Clone)]
pub struct SessionTokenStrategy {
    access_token: String,

    // this is only `pub(crate)` so we can actually refresh it once
    // and never use it again
    #[allow(dead_code)]
    pub(crate) refresh_token: Option<String>,
}

impl SessionTokenStrategy {
    /// Creates a new [`SessionTokenStrategy`] with an access token and no refresh token.
    pub fn new<S: AsRef<str>>(access_token: S) -> SessionTokenStrategy {
        SessionTokenStrategy {
            access_token: access_token.as_ref().to_owned(),
            refresh_token: None,
        }
    }

    /// Creates a new [`SessionTokenStrategy`] with an access and refresh token.
    pub fn new_with_refresh(access_token: impl AsRef<str>, refresh_token: impl AsRef<str>) -> SessionTokenStrategy {
        SessionTokenStrategy {
            access_token: access_token.as_ref().to_owned(),
            refresh_token: Some(refresh_token.as_ref().to_owned()),
        }
    }
}

impl AuthStrategy for SessionTokenStrategy {
    fn prefix(&self) -> String {
        "Bearer".to_owned()
    }

    fn value(&self) -> String {
        self.access_token.clone()
    }
}
