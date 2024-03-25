use crate::models::{MidenProof, Risc0Proof, Sp1Proof, VerifyProof};
use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SP1_HASHMAP: Mutex<HashMap<String, Sp1Proof>> = Mutex::new(HashMap::new());
    pub static ref MIDEN_HASHMAP: Mutex<HashMap<String, MidenProof>> = Mutex::new(HashMap::new());
    pub static ref RISC0_HASHMAP: Mutex<HashMap<String, Risc0Proof>> = Mutex::new(HashMap::new());
    pub static ref VERIFY_QUEUE: Arc<Mutex<VecDeque<VerifyProof>>> = Arc::new(Mutex::new(VecDeque::new()));
    pub static ref INSTANTIATED_PORTS: Mutex<Vec<u16>> = Mutex::new(Vec::from(vec![8081, 8082, 8083, 8084, 8085]));
    pub static ref UNINSTANTIATED_PORTS: Mutex<Vec<u16>> = Mutex::new(Vec::from(vec![8086, 8087, 8088, 8089, 8090]));
    pub static ref CURRENT_PORT: Mutex<u16> = Mutex::new(0);
}