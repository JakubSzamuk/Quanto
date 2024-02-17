use crate::error::GeneratorError;
use crate::config::Config;
use crate::Database;

pub fn generate(db_arg: Database) -> Result<bool, GeneratorError> {
    // create a folder
    std::fs::create_dir("data").unwrap();


    // populate folder
    if matches!(db_arg, Database::Sqlite) {
        // Pregenerate if its sqlite
        std::fs::write("./data/quanto-config.toml", toml::to_string_pretty(&Config::default()).unwrap()).unwrap();
        std::fs::create_dir("./data/migrations").unwrap();
        std::fs::create_dir("./data/db").unwrap();
    } else {
        // manual config if not sqlite
        std::fs::write("./data/quanto-config.toml", toml::to_string_pretty(&Config {
            db: db_arg,
            gen_db: false,
            db_address: "localhost".to_string(),
        }).unwrap()).unwrap();
        println!("Please edit data/quanto-config.toml to match your database configuration.")
    }

    Ok(true)
}

pub fn check_if_config_exists() -> bool {
    std::path::Path::new("./data").exists()
}