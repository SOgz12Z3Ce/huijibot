use reqwest::multipart::Form;
pub(super) fn login_body(login_token: &str, username: &str, password: &str, login_return_url: &str) -> Form {
    Form::new()
        .text("action", "clientlogin")
        .text("logintoken", login_token.to_owned())
        .text("username", username.to_owned())
        .text("password", password.to_owned())
        .text("loginreturnurl", login_return_url.to_owned())
        .text("format", "json")
}
