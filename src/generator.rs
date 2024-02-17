use crate::error::GeneratorError;
use crate::config::Config;

pub fn Generate() -> Result<bool, GeneratorError> {

    if std::path::Path::new("data").exists() {
        return Err(GeneratorError::new("data folder already exists"));
    }

    // create a folder
    std::fs::create_dir("data").unwrap();
    // populate folder
    std::fs::write("data/quanto-config.toml", toml::to_string_pretty(&Config::default())).unwrap();


    Ok(true)
}