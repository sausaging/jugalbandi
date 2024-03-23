use std::env;

use log::warn;
pub struct Config {
    pub port: u16,
    pub workers: usize,
    pub delete_files: bool,
}

impl Config {
    pub fn init() -> Self {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("PORT must be a number");
        let workers = env::var("WORKERS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .expect("WORKERS must be a number");
        let delete_files = env::var("DELETE_FILES")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .expect("DELETE_FILES must be a boolean");
        Config {
            port,
            workers,
            delete_files,
        }
    }
}

pub fn handle_delete_files(files: &Vec<String>) {
    let config = Config::init();
    if config.delete_files {
        for file in files {
            let _ =
                std::fs::remove_file(file).map_err(|err| warn!("Error deleting file: {:?}", err));
        }
    }
}
