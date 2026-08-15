mod body;
mod error;
mod respond;
use reqwest::Client;
use respond::GetTokenResponse;

pub async fn get_csrf_token(client: &Client, site: &str) -> String {
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

pub async fn get_login_token(client: &Client, site: &str) -> String {
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

pub async fn login(
    client: &Client,
    site: &str,
    login_token: &str,
    username: &str,
    password: &str,
    login_return_url: &str,
) {
    let body = body::login_body(login_token, username, password, login_return_url);
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

pub async fn edit(
    client: &Client,
    site: &str,
    csrf_token: &str,
    title: &str,
    text: &str,
    summary: &str,
) {
    let url = format!("https://{site}.huijiwiki.com/api.php");
    let body = body::edit_body(csrf_token, title, text, summary);
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
