use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct GetLoginTokenResponse {
    pub(super) query: GetLoginTokenQuery,
}

#[derive(Debug, Deserialize)]
pub(super) struct GetLoginTokenQuery {
    pub(super) tokens: GetLoginTokenTokens,
}

#[derive(Debug, Deserialize)]
pub(super) struct GetLoginTokenTokens {
    pub(super) logintoken: String,
}
