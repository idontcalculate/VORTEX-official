use std::env;

pub struct Config {
    pub model_name: String,
    pub model_path: String,
    pub output_dir: String,
    pub vortex_version: i32,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            model_name: env::var("MODEL_NAME").unwrap_or_else(|_| "bert-base-uncased".to_string()),
            model_path: env::var("MODEL_PATH").unwrap_or_else(|_| "models".to_string()),
            output_dir: env::var("OUTPUT_DIR").unwrap_or_else(|_| "output".to_string()),
            vortex_version: env::var("VORTEX_VERSION").unwrap_or_else(|_| "1".to_string()).parse().unwrap(),
        }
    }
}
