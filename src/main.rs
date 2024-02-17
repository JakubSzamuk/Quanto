mod generator;
mod config;
mod error;

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};


// Define the command line arguments, -d --dev for development mode, -p --prod for production mode, -g --generate for generating the files
#[derive(Parser, Debug)]
#[clap(name = "basic")]
struct Args {
    #[clap(short, long, value_enum)]
    mode: Mode,
    #[clap(short, long, value_enum, default_value = "sqlite")]
    db: Database
}

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab_case")]
enum Mode {
    Development,
    Production,
    Generate,
}
#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
#[clap(rename_all = "kebab_case")]
pub enum Database {
    Mysql,
    Postgres,
    Sqlite,
}



fn main() {
    let opts = Args::parse();
    if !generator::check_if_config_exists() {
        match generator::generate(opts.db) {
            Ok(_) => println!("Generated files"),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("Using existing config file: {}", std::env::current_dir().unwrap().join("data/quanto-config.toml").to_str().unwrap());
    }

    match opts.mode {
        Mode::Development => {
            println!("Development mode");
        }
        Mode::Production => {
            println!("Production mode");
        }
        Mode::Generate => {}
    }
}
