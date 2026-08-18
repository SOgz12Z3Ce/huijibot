use crate::action;
use crate::action::params::EditParams;
use crate::action::params::LoginParams;
use crate::cli::metadata::{self, HuijibotConfig};
use crate::wiki::Title;
use crate::wiki_client::WikiClient;
use futures::future;
use reqwest::cookie::Jar;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use walkdir::DirEntry;
use walkdir::WalkDir;

pub(crate) fn init(dry: bool, path: PathBuf) -> Result<(), std::io::Error> {
    let huijibot_path = path.join(".huijibot");
    println!("Creating config directory: {}", huijibot_path.display());
    if !dry {
        fs::create_dir(huijibot_path)?;
    }
    Ok(())
}

pub(crate) fn config(dry: bool, patch: HuijibotConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("Finding config");
    let config_path = metadata::config_path();
    println!("Config found at: {}", config_path.display());
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
    println!("Writing config:");
    println!("{content}");
    if !dry {
        fs::write(config_path, content)?;
    }
    Ok(())
}

pub(crate) async fn push(
    dry: bool,
    paths: Vec<PathBuf>,
    worker: usize,
    duration: Duration,
    summary: String,
) -> Result<(), std::io::Error> {
    let config = metadata::config();
    let jar = Arc::new(Jar::default());
    let wiki_client = WikiClient::new(Arc::clone(&jar), config.site.unwrap());

    println!(
        "Logging in {}.huijiwiki.com with account:",
        wiki_client.site()
    );
    println!("Username: {}", config.username.as_ref().unwrap());
    println!("Password: {}", config.password.as_ref().unwrap());
    if !dry {
        let login_token = action::get_login_token(&wiki_client).await;
        action::login(
            &wiki_client,
            LoginParams::builder()
                .login_token(login_token)
                .username(config.username.unwrap())
                .password(config.password.unwrap())
                .build(),
        )
        .await;
    }

    println!("Collecting files:");
    let root_path = metadata::root_path();
    let files = collect_files(paths);
    for file in files.iter() {
        println!("{}", file.display());
    }

    for chuck in files.chunks(worker) {
        let tasks = chuck.iter().map(|file| {
            let file_path = fs::canonicalize(&file).unwrap();
            let relative_path = pathdiff::diff_paths(file_path, &root_path).unwrap();
            let title = Title::new(relative_path);
            let text = fs::read_to_string(file).unwrap();

            let wiki_client = wiki_client.clone();
            let summary = summary.clone();
            async move {
                let debug_title = title.clone();
                println!("Editing: {debug_title}");
                if !dry {
                    edit(wiki_client, title, text, summary).await;
                }
                println!("Done: {debug_title}");
            }
        });
        future::join_all(tasks).await;
        println!("Wait for {}s now...", duration.as_secs());
        if !dry {
            time::sleep(duration).await;
        }
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

async fn edit(wiki_client: WikiClient, title: Title, text: String, summary: String) {
    let csrf_token = action::get_csrf_token(&wiki_client).await;
    action::edit(
        &wiki_client,
        EditParams::builder()
            .csrf_token(csrf_token)
            .title(title.to_string())
            .text(text)
            .summary(summary)
            .bot(true)
            .build(),
    )
    .await;
}
