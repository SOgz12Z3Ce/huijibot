use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct GetTokenResponse {
    pub(super) query: GetTokenQuery,
}

#[derive(Debug, Deserialize)]
pub(super) struct GetTokenQuery {
    pub(super) tokens: GetTokenTokens,
}

#[derive(Debug, Deserialize)]
pub(super) struct GetTokenTokens {
    pub(super) csrftoken: Option<String>,
    pub(super) logintoken: Option<String>,
}
