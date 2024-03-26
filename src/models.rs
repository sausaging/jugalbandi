use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ProofDataSP1 {
    pub tx_id: String,
    pub proof_file_path: String,
    pub elf_file_path: String,
}

#[derive(Deserialize, Debug)]
pub struct Sp1Proof {
    pub proof_file_path: String,
    pub elf_file_path: String,
}

#[derive(Deserialize, Debug)]
pub struct ProofDataMiden {
    pub tx_id: String,
    pub code_front_end: String,
    pub inputs_front_end: String,
    pub outputs_front_end: String,
    pub proof_file_path: String,
}

#[derive(Deserialize, Debug)]
pub struct MidenProof {
    pub code_front_end: String,
    pub inputs_front_end: String,
    pub outputs_front_end: String,
    pub proof_file_path: String,
}

#[derive(Deserialize, Debug)]
pub struct ProodDataRisc0 {
    pub tx_id: String,
    pub proof_file_path: String,
    pub risc_zero_image_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Risc0Proof {
    pub proof_file_path: String,
    pub risc_zero_image_id: String,
}

#[derive(Deserialize, Debug)]
pub struct VerifyProof {
    pub tx_id: String,
    pub verify_type: u8,
}

#[derive(Serialize)]
pub struct SubmitionResult {
    pub is_submitted: bool,
}

#[derive(Serialize, Debug)]
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
    pub rust_port: String,
    pub uinit_port: String,
}

#[derive(Serialize, Debug)]
pub struct PingSingle {
    pub success: bool,
}

#[derive(Serialize, Debug)]
pub struct PostVerificationResult {
    pub tx_id: String,
    pub is_valid: bool,
}
