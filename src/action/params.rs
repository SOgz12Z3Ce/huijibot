pub(crate) struct LoginParams {
    pub(crate) login_token: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl LoginParams {
    pub(crate) fn builder() -> LoginParamsBuilder {
        LoginParamsBuilder::new()
    }
}

pub(crate) struct LoginParamsBuilder {
    login_token: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl LoginParamsBuilder {
    fn new() -> Self {
        Self {
            login_token: None,
            username: None,
            password: None,
        }
    }

    pub(crate) fn build(self) -> LoginParams {
        LoginParams {
            login_token: self.login_token.unwrap(),
            username: self.username.unwrap(),
            password: self.password.unwrap(),
        }
    }

    pub(crate) fn login_token(mut self, login_token: String) -> Self {
        self.login_token = Some(login_token);
        self
    }

    pub(crate) fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub(crate) fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }
}

pub(crate) struct EditParams {
    pub(crate) csrf_token: String,
    pub(crate) title: String,
    pub(crate) text: String,
    pub(crate) summary: String,
    pub(crate) bot: bool,
}

impl EditParams {
    pub(crate) fn builder() -> EditParamsBuilder {
        EditParamsBuilder::new()
    }
}

pub(crate) struct EditParamsBuilder {
    csrf_token: Option<String>,
    title: Option<String>,
    text: Option<String>,
    summary: Option<String>,
    bot: Option<bool>,
}

impl EditParamsBuilder {
    fn new() -> Self {
        Self {
            csrf_token: None,
            title: None,
            text: None,
            summary: None,
            bot: None,
        }
    }

    pub(crate) fn build(self) -> EditParams {
        EditParams {
            csrf_token: self.csrf_token.unwrap(),
            title: self.title.unwrap(),
            text: self.text.unwrap(),
            summary: self.summary.unwrap_or(String::new()),
            bot: self.bot.unwrap_or(false),
        }
    }

    pub(crate) fn csrf_token(mut self, csrf_token: String) -> Self {
        self.csrf_token = Some(csrf_token);
        self
    }

    pub(crate) fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub(crate) fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub(crate) fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    pub(crate) fn bot(mut self, bot: bool) -> Self {
        self.bot = Some(bot);
        self
    }
}
