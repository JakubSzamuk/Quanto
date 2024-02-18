mod generator;
mod config;
mod error;
mod adminweb;

use std::fmt::Debug;
use std::thread;
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use crate::adminweb::run_web_server;
use crate::config::Config;


// Define the command line arguments, -d --dev for development mode, -p --prod for production mode, -g --generate for generating the files
#[derive(Parser, Debug)]
#[clap(name = "basic")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[clap(short, long, value_enum, default_value = "sqlite")]
    db: Database
}

#[derive(Debug, Clone, Subcommand)]
#[clap(rename_all = "kebab_case")]
enum Commands {
    Dev,
    Deploy,
    Generate,
}


#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
#[clap(rename_all = "kebab_case")]
pub enum Database {
    Mysql,
    Postgresql,
    Sqlite,
}


#[tokio::main]
async fn main() {
    let opts = Cli::parse();

    if !generator::check_if_config_exists() {
        match generator::generate(opts.db).await {
            Ok(_) => println!("Generated files"),
            Err(e) => println!("Error: {}", e),
        }
    }
    let config = Config::get_config().await;
    match opts.command {
        Commands::Dev => {
            thread::spawn(|| {
                let _ = run_web_server(config, Commands::Dev);
            }).join().expect("Web Server Thread failed")
        }
        Commands::Deploy => {
            println!("Production mode");
        }
        Commands::Generate => {}
    }
}
