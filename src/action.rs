mod body;
mod error;
mod respond;
use reqwest::Client;
use respond::GetLoginTokenResponse;

use crate::action::body::login_body;

pub async fn get_login_token(client: &Client) -> String {
    let response = client
        .get("https://sixthhistory.huijiwiki.com/api.php?action=query&meta=tokens&type=login&format=json")
        .send()
        .await
        .unwrap()
        .json::<GetLoginTokenResponse>()
        .await
        .unwrap();
    response.query.tokens.logintoken
}

pub async fn login(
    client: &Client,
    login_token: &str,
    username: &str,
    password: &str,
    login_return_url: &str,
) {
    let body = login_body(login_token, username, password, login_return_url);
    let _response = client
        .post("https://sixthhistory.huijiwiki.com/api.php")
        .multipart(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}
