use serde_json::from_str;
use sp1_core::{SP1ProofWithIO, SP1Verifier};
use std::fs;
use log::{warn, info};

use crate::errors::VerificationError;
use crate::models::{ProofDataSP1, VerificationResult};

pub async fn verify(data: ProofDataSP1) -> Result<VerificationResult, VerificationError> {
    info!("{:?}", data);
    let proof = fs::read_to_string(&data.proof_file_path)?;
    let parsed_proof_result: Result<SP1ProofWithIO<sp1_core::utils::BabyBearBlake3>, serde_json::Error> =
        from_str(&proof);
    let elf_result = fs::read(&data.elf_file_path);
    match (parsed_proof_result, elf_result) {
        (Ok(proof), Ok(elf)) => {
            let verification_result = SP1Verifier::verify(&elf, &proof);
            match verification_result {
                Ok(_) => Ok(VerificationResult { is_valid: true }),
                Err(err) => {
                    warn!("Verification failed: {:?}", err);
                    Ok(VerificationResult { is_valid: false })
                }
            }
        }
        (Err(parse_err), Err(io_err)) => {
            Err(VerificationError::JsonErrIOErr(parse_err, io_err, "Error parsing proof JSON and IO".to_string()))
        }
        (Err(parse_err), _) => {
            Err(VerificationError::JSONError(parse_err, "Error parsing proof JSON".to_string()))
        }
        (_, Err(io_err)) => {
            Err(VerificationError::IOError(io_err, "Error reading ELF file".to_string()))
        }
    }
}
