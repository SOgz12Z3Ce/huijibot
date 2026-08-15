mod body;
mod error;
mod respond;
use reqwest::Client;
use respond::GetTokenResponse;

pub async fn get_csrf_token(client: &Client) -> String {
    let response = client
        .get("https://sixthhistory.huijiwiki.com/api.php?action=query&meta=tokens&format=json")
        .send()
        .await
        .unwrap()
        .json::<GetTokenResponse>()
        .await
        .unwrap();
    response.query.tokens.csrftoken.unwrap()
}

pub async fn get_login_token(client: &Client) -> String {
    let response = client
        .get("https://sixthhistory.huijiwiki.com/api.php?action=query&meta=tokens&type=login&format=json")
        .send()
        .await
        .unwrap()
        .json::<GetTokenResponse>()
        .await
        .unwrap();
    response.query.tokens.logintoken.unwrap()
}

pub async fn login(
    client: &Client,
    login_token: &str,
    username: &str,
    password: &str,
    login_return_url: &str,
) {
    let body = body::login_body(login_token, username, password, login_return_url);
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

pub async fn edit(client: &Client, csrf_token: &str, title: &str, text: &str, summary: &str) {
    let body = body::edit_body(csrf_token, title, text, summary);
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
