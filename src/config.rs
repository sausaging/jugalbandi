use crate::storage::CURRENT_PORT;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::{MidenProof, Risc0Proof, Sp1Proof, VerifyProof};

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

pub async fn process_verification_queue(
    queue: Arc<Mutex<VecDeque<VerifyProof>>>,
    _sp1_hashmap: Arc<Mutex<HashMap<String, Sp1Proof>>>,
    _risc0_hashmap: Arc<Mutex<HashMap<String, Risc0Proof>>>,
    _miden_hashmap: Arc<Mutex<HashMap<String, MidenProof>>>,
) {
    loop {
        let mut queue = queue.lock().await;

        if queue.is_empty() {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            continue;
        }

        let verification_proof = queue.pop_front().unwrap();

        // implement later
        match verification_proof.proof_type {
            1 => {}
            2 => {}
            3 => {}
            _ => {}
        }

        let verification_successful = false;
        if verification_successful {
            // Send POST request to the other server on successful verification
            let port = CURRENT_PORT.lock().await;
            let url_str = format!("http://127.0.0.1:{}/submit-result", port.to_string());
            let url = reqwest::Url::from_str(&url_str).expect("Failed to parse URL");
            let client = reqwest::Client::new();
            let mut map = HashMap::new();
            map.insert("tx_id", verification_proof.tx_id);
            map.insert("verification_status", "true".to_string());
            let response = client
                .post(url)
                .json(&map)
                .send()
                .await
                .expect("Failed to send POST request");

            if response.status().is_success() {
                println!("Verification proof sent successfully!");
            } else {
                println!("Failed to send verification proof: {}", response.status());
            }
        } else {
            println!("Verification failed for proof: {:?}", verification_proof);
        }
    }
}
