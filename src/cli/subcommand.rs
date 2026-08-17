use crate::action;
use crate::action::params::EditParams;
use crate::action::params::LoginParams;
use crate::cli::metadata::{self, HuijibotConfig};
use crate::wiki::Title;
use crate::wiki_client::WikiClient;
use std::fs;
use std::path::PathBuf;
use walkdir::DirEntry;
use walkdir::WalkDir;

pub(crate) fn init(path: PathBuf) -> Result<(), std::io::Error> {
    let huijibot_path = path.join(".huijibot");
    fs::create_dir(huijibot_path)
}

pub(crate) async fn push(paths: Vec<PathBuf>, _worker: u8, _gap: u8) -> Result<(), std::io::Error> {
    let root_path = metadata::root_path();
    let files = collect_files(paths);
    for file in files {
        let file_path = fs::canonicalize(&file).unwrap();
        let relative_path = pathdiff::diff_paths(file_path, &root_path).unwrap();
        let title = Title::new(relative_path);
        let text = fs::read_to_string(file)?;
        edit(title.to_string(), text).await;
    }
    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .path()
        .file_name()
        .and_then(|n| n.to_str().map(|s| s.starts_with(".")))
        .unwrap_or(false)
}

fn collect_files(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut files = vec![];
    for path in paths {
        let walker = WalkDir::new(path).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                files.push(entry.into_path());
            }
        }
    }
    files
}

pub(crate) fn config(patch: HuijibotConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = metadata::config_path();
    let mut config = metadata::config();
    if let Some(site) = patch.site {
        config.site = Some(site);
    }
    if let Some(username) = patch.username {
        config.username = Some(username);
    }
    if let Some(password) = patch.password {
        config.password = Some(password);
    }

    let content = toml::to_string_pretty(&config)?;
    fs::write(config_path, content)?;
    Ok(())
}

async fn edit(title: String, text: String) {
    let config = metadata::config();
    let site = config.site.unwrap();
    let username = config.username.unwrap();
    let password = config.password.unwrap();

    let wiki_client = WikiClient::new(site);
    let login_token = action::get_login_token(&wiki_client).await;
    action::login(
        &wiki_client,
        LoginParams::builder()
            .login_token(login_token)
            .username(username)
            .password(password)
            .build(),
    )
    .await;
    let csrf_token = action::get_csrf_token(&wiki_client).await;
    action::edit(
        &wiki_client,
        EditParams::builder()
            .csrf_token(csrf_token)
            .title(title)
            .text(text)
            .bot(true)
            .build(),
    )
    .await;
}
