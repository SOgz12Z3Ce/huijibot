mod body;
mod error;
pub mod params;
mod respond;
use crate::{
    action::params::{EditParams, LoginParams},
    wiki_client::WikiClient,
};
use respond::GetTokenResponse;

pub async fn get_csrf_token(wiki_client: &WikiClient) -> String {
    let client = wiki_client.client();
    let site = wiki_client.site();
    let url = format!("https://{site}.huijiwiki.com/api.php?action=query&meta=tokens&format=json");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<GetTokenResponse>()
        .await
        .unwrap();
    response.query.tokens.csrftoken.unwrap()
}

pub async fn get_login_token(wiki_client: &WikiClient) -> String {
    let client = wiki_client.client();
    let site = wiki_client.site();
    let url = format!(
        "https://{site}.huijiwiki.com/api.php?action=query&meta=tokens&type=login&format=json"
    );
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<GetTokenResponse>()
        .await
        .unwrap();
    response.query.tokens.logintoken.unwrap()
}

pub async fn login(wiki_client: &WikiClient, params: LoginParams) {
    let client = wiki_client.client();
    let site = wiki_client.site();
    let body = body::login_body(params, site);
    let url = format!("https://{site}.huijiwiki.com/api.php");
    let _response = client
        .post(url)
        .multipart(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}

pub async fn edit(wiki_client: &WikiClient, params: EditParams) {
    let client = wiki_client.client();
    let site = wiki_client.site();
    let body = body::edit_body(params);
    let url = format!("https://{site}.huijiwiki.com/api.php");
    let _response = client
        .post(url)
        .multipart(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}
