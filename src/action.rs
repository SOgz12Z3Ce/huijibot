use reqwest::Client;

pub async fn get_csrf_token(client: &Client) -> String {
    let response = client
        .get("https://sixthhistory.huijiwiki.com/api.php?action=query&meta=tokens")
        .send()
        .await
        .unwrap();
    response.text().await.unwrap()
}
