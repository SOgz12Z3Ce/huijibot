use reqwest::multipart::Form;

use crate::action::params::{EditParams, LoginParams};

pub(crate) fn login_body(params: LoginParams, site: &str) -> Form {
    Form::new()
        .text("action", "clientlogin")
        .text("logintoken", params.login_token)
        .text("username", params.username)
        .text("password", params.password)
        .text("loginreturnurl", format!("https://{site}.huijiwiki.com/"))
        .text("format", "json")
}

pub(crate) fn edit_body(params: EditParams) -> Form {
    let body = Form::new()
        .text("action", "edit")
        .text("title", params.title)
        .text("summary", params.summary)
        .text("text", params.text)
        .text("token", params.csrf_token)
        .text("format", "json");
    if params.bot {
        body.text("bot", "1")
    } else {
        body
    }
}
