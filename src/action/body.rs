use reqwest::multipart::Form;

pub(super) fn login_body(
    login_token: &str,
    username: &str,
    password: &str,
    login_return_url: &str,
) -> Form {
    Form::new()
        .text("action", "clientlogin")
        .text("logintoken", login_token.to_owned())
        .text("username", username.to_owned())
        .text("password", password.to_owned())
        .text("loginreturnurl", login_return_url.to_owned())
        .text("format", "json")
}

pub(super) fn edit_body(csrf_token: &str, title: &str, text: &str, summary: &str) -> Form {
    Form::new()
        .text("action", "edit")
        .text("title", title.to_owned())
        .text("summary", summary.to_owned())
        .text("text", text.to_owned())
        .text("token", csrf_token.to_owned())
        .text("format", "json")
}
