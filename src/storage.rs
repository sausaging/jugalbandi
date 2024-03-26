use crate::models::{MidenProof, Risc0Proof, Sp1Proof, VerifyProof};
use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref SP1_HASHMAP: Arc<Mutex<HashMap<String, Sp1Proof>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref MIDEN_HASHMAP: Arc<Mutex<HashMap<String, MidenProof>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref RISC0_HASHMAP: Arc<Mutex<HashMap<String, Risc0Proof>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref VERIFY_QUEUE: Arc<Mutex<VecDeque<VerifyProof>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    pub static ref INSTANTIATED_PORTS: Arc<Mutex<Vec<u16>>> =
        Arc::new(Mutex::new(Vec::from(vec![8081, 8082, 8083, 8084, 8085])));
    pub static ref UNINSTANTIATED_PORTS: Arc<Mutex<Vec<u16>>> =
        Arc::new(Mutex::new(Vec::from(vec![8086, 8087, 8088, 8089, 8090])));
    pub static ref PORT_INDEX: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}
