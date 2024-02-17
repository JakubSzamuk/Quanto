use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub db: crate::Database,
    pub gen_db: bool,
    pub db_address: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            db: crate::Database::Sqlite,
            gen_db: true,
            db_address: "localhost".to_string(),
        }
    }
}