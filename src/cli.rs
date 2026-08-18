mod metadata;
mod subcommand;

use crate::cli::metadata::HuijibotConfig;
use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser)]
#[command(version = "0.1")]
struct Cli {
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
        worker: usize,

        #[arg(short, long, default_value_t = 5)]
        duration: usize,

        #[arg(short, long, default_value = "")]
        summary: String,
    },
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(feature = "peek") {
        return Ok(());
    }
    let cli = Cli::parse();
    let dry = cli.dry;
    match cli.command {
        Commands::Init { path } => {
            subcommand::init(dry, path)?;
        }
        Commands::Config { config } => subcommand::config(dry, config)?,
        Commands::Push {
            paths,
            worker,
            duration,
            summary,
        } => {
            subcommand::push(
                dry,
                paths,
                worker,
                Duration::from_secs(duration.try_into()?),
                summary,
            )
            .await?;
        }
    }
    if dry {
        println!("Aborting due to dry run");
    }
    Ok(())
}
