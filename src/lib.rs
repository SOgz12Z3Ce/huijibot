mod action;
mod wiki_client;

pub use action::edit;
pub use action::get_csrf_token;
pub use action::get_login_token;
pub use action::login;
pub use action::params::EditParams;
pub use action::params::EditParamsBuilder;
pub use action::params::LoginParams;
pub use action::params::LoginParamsBuilder;
pub use wiki_client::WikiClient;
