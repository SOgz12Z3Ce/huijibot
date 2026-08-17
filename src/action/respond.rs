use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GetTokenResponse {
    pub(crate) query: GetTokenQuery,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GetTokenQuery {
    pub(crate) tokens: GetTokenTokens,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GetTokenTokens {
    pub(crate) csrftoken: Option<String>,
    pub(crate) logintoken: Option<String>,
}
