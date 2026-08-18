use reqwest::{Client, cookie::Jar};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct WikiClient {
    client: Client,
    site: String,
}

impl WikiClient {
    pub(crate) fn new(cookie_provider: Arc<Jar>, site: String) -> Self {
        Self {
            client: Client::builder()
                .tls_sslkeylogfile(cfg!(debug_assertions))
                .cookie_provider(cookie_provider)
                .cookie_store(true)
                .user_agent("huijibot/0.1")
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
