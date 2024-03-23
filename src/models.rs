use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ProofDataSP1 {
    pub proof_file_path: String,
    pub elf_file_path: String,
}

#[derive(Deserialize, Debug)]
pub struct ProofDataMiden {
    pub code_front_end: String,
    pub inputs_front_end: String,
    pub outputs_front_end: String,
    pub proof_file_path: String,
}

#[derive(Deserialize, Debug)]
pub struct ProodDataRisc0 {
    pub proof_file_path: String,
    pub risc_zero_image_id: String,
}

#[derive(Serialize)]
pub struct VerificationResult {
    pub is_valid: bool,
}

#[derive(Deserialize, Debug)]
pub struct Proof {
    pub proof: Vec<u8>,
}

#[derive(Serialize)]
pub struct Ping {
    pub success: bool,
    pub port: u16,
}
