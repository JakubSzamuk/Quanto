use serde::{Deserialize, Serialize};
use sqlx::migrate::MigrateDatabase;
use sqlx::{MySqlPool, PgPool};
use crate::Database;
use crate::error::BaseError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub db: crate::Database,
    pub db_address: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            db: crate::Database::Sqlite,
            db_address: "sqlite://data/db/data.db".to_string(),
        }
    }
}

impl Config {
    pub async fn get_config() -> Self {
        let config_string = std::fs::read_to_string("data/quanto-config.toml").expect("Error, Config is missing");
        let config: Config = toml::from_str(config_string.as_str()).expect("Error, failed to parse config file");

        match &config.db {
            Database::Sqlite => {
                use sqlx::Sqlite;
                if !Sqlite::database_exists(&config.db_address).await.unwrap() {
                    panic!("Error, Could not connect to database");
                }
            },
            Database::Mysql => {
                use sqlx::MySqlPool;
                if let Err(_) = MySqlPool::connect(&config.db_address).await {
                    panic!("Error, Could not connect to database");
                }
            },
            Database::Postgresql => {
                use sqlx::PgPool;
                if let Err(_) = PgPool::connect(&config.db_address).await {
                    panic!("Error, Could not connect to database");
                }
            }
        };

        config
    }
}