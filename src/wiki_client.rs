use reqwest::{
    Client,
    cookie::Jar,
    header::{HeaderMap, HeaderValue},
};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct WikiClient {
    client: Client,
    site: String,
}

impl WikiClient {
    pub(crate) fn new(cookie_provider: Arc<Jar>, site: String, auth_key: &str) -> Self {
        let auth_key = {
            let mut auth_key = HeaderValue::from_str(auth_key).unwrap();
            auth_key.set_sensitive(true);
            auth_key
        };
        let headers = {
            let mut headers = HeaderMap::new();
            headers.insert("X-authkey", auth_key);
            headers
        };

        Self {
            client: Client::builder()
                .tls_sslkeylogfile(cfg!(debug_assertions))
                .cookie_provider(cookie_provider)
                .cookie_store(true)
                .user_agent("huijibot/0.1")
                .default_headers(headers)
                .build()
                .unwrap(),
            site: site.to_owned(),
        }
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }

    pub(crate) fn site(&self) -> &str {
        &self.site
    }
}
