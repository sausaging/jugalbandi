use log::{info, warn};
use std::collections::{HashMap, VecDeque};
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::errors::VerificationError;
use crate::models::{
    JoltProof, MidenProof, PostVerificationResult, Risc0Proof, Sp1Proof, VerificationResult,
    VerifyProof,
};
use crate::services::{jolt_verifier, miden_verifier, risc0_verifier, sp1_verifier};

pub struct Config {
    pub port: u16,
    pub workers: usize,
    pub delete_files: bool,
    pub u_port: u16,
}

impl Config {
    pub fn init() -> Self {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("PORT must be a number");
        let workers = env::var("WORKERS")
            .unwrap_or_else(|_| "1".to_string())
            .parse()
            .expect("WORKERS must be a number");
        let delete_files = env::var("DELETE_FILES")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .expect("DELETE_FILES must be a boolean");
        let u_port: u16 = env::var("UPORT")
            .unwrap_or_else(|_| "0".to_string())
            .parse()
            .expect("UPort must be a number");
        Config {
            port,
            workers,
            delete_files,
            u_port,
        }
    }
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Config {
            port: self.port,
            workers: self.workers,
            delete_files: self.delete_files,
            u_port: self.u_port,
        }
    }
}

pub fn handle_verification_result(
    verification_result: Result<VerificationResult, VerificationError>,
) -> bool {
    match verification_result {
        Ok(result) => {
            info!("Proof Verification Successfull {:?}", result);
            result.is_valid
        }
        Err(err) => {
            warn!("Verification Error: {:?}", err);
            false
        }
    }
}

pub async fn process_verification_queue(
    queue: Arc<Mutex<VecDeque<VerifyProof>>>,
    _sp1_hashmap: Arc<Mutex<HashMap<String, Sp1Proof>>>,
    _risc0_hashmap: Arc<Mutex<HashMap<String, Risc0Proof>>>,
    _miden_hashmap: Arc<Mutex<HashMap<String, MidenProof>>>,
    _jolt_hashmap: Arc<Mutex<HashMap<String, JoltProof>>>,
) {
    loop {
        let mut queue = queue.lock().await;

        if queue.is_empty() {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            continue;
        }

        let verification_proof = queue.pop_front().unwrap();
        info!("Processing verification proof: {:?}", verification_proof);
        let is_valid;
        match verification_proof.verify_type {
            1 => {
                let sp1_hashmap = _sp1_hashmap.lock().await;
                let sp1_proof = sp1_hashmap.get(&verification_proof.tx_id).unwrap();
                let verification_result = sp1_verifier::verify(sp1_proof).await;
                is_valid = handle_verification_result(verification_result);
            }
            2 => {
                let miden_hashmap = _miden_hashmap.lock().await;
                let miden_proof = miden_hashmap.get(&verification_proof.tx_id).unwrap();
                let verification_result = miden_verifier::verify(miden_proof).await;
                is_valid = handle_verification_result(verification_result);
            }
            3 => {
                let risc0_hashmap = _risc0_hashmap.lock().await;
                let risc0_proof = risc0_hashmap.get(&verification_proof.tx_id).unwrap();
                let verification_result = risc0_verifier::verify(risc0_proof).await;
                is_valid = handle_verification_result(verification_result);
            }
            4 => {
                let jolt_hashmap = _jolt_hashmap.lock().await;
                let jolt_proof = jolt_hashmap.get(&verification_proof.tx_id).unwrap();
                let verification_result = jolt_verifier::verify(jolt_proof).await;
                is_valid = handle_verification_result(verification_result);
            }
            _ => {
                warn!("Invalid proof type");
                is_valid = false;
            }
        }
        // Send POST request to the other server on successful verification
        let config = Config::init();
        let port = config.u_port;
        let url_str = format!("http://127.0.0.1:{}/submit-result", port.to_string());
        info!("Sending verification proof to: {}", url_str);
        let url = reqwest::Url::from_str(&url_str).expect("Failed to parse URL");
        let client = reqwest::Client::new();
        let map = PostVerificationResult {
            tx_id: verification_proof.tx_id,
            is_valid,
        };
        let response = client
            .post(url)
            .json(&map)
            .send()
            .await
            .expect("Failed to send POST request");
        info!("Response: {:?}", response);
        if response.status().is_success() {
            info!("Verification proof sent successfully!");
        } else {
            warn!("Failed to send verification proof: {}", response.status());
        }
    }
}
