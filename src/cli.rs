mod metadata;
mod subcommand;

use crate::cli::metadata::HuijibotConfig;
use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version = "0.1")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    dry: bool,
}

#[derive(Clone, Subcommand)]
enum Commands {
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    Config {
        #[command(flatten)]
        config: HuijibotConfig,
    },
    Push {
        paths: Vec<PathBuf>,

        #[arg(short, long, default_value_t = 1)]
        worker: u8,

        #[arg(short, long, default_value_t = 5)]
        gap: u8,
    },
}

pub async fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { path } => {
            subcommand::init(path)?;
        }
        Commands::Config { config } => subcommand::config(config)?,
        Commands::Push { paths, worker, gap } => {
            subcommand::push(paths, worker, gap).await?;
        }
    }
    Ok(())
}
