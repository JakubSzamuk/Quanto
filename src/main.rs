mod generator;
mod config;
mod error;

use clap::{Parser, ValueEnum};



// Define the command line arguments, -d --dev for development mode, -p --prod for production mode, -g --generate for generating the files
#[derive(Parser, Debug)]
#[clap(name = "basic")]
struct Args {
    #[clap(short, long, value_enum)]
    mode: Mode,
    db: Database
}

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab_case")]
enum Mode {
    Development,
    Production,
    Generate,
}
#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub enum Database {
    Mysql,
    Postgres,
    Sqlite,
}



fn main() {
    let opts = Args::parse();

    match opts.mode {
        Mode::Development => {
            println!("Development mode");
        }
        Mode::Production => {
            println!("Production mode");
        }
        Mode::Generate => {

        }
    }
}
