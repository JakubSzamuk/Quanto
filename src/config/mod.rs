use serde::{Deserialize, Serialize};
use crate::Database;

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



        macro_rules! verify_db {
            ($db_type: ident) => {
                use sqlx::$db_type;
                if let Err(_) = $db_type::connect(&config.db_address).await {
                    panic!("Error, Could not connect to database");
                }
            };
        }
        match &config.db {
            Database::Sqlite => {
                verify_db!(SqlitePool);
            },
            Database::Mysql => {
                verify_db!(MySqlPool);
            },
            Database::Postgresql => {
                verify_db!(PgPool);
            }
        };

        config
    }
}