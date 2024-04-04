use log::{info, warn};
use serde_json::from_str;
use sp1_core::{SP1ProofWithIO, SP1Verifier};
use std::fs;

use super::helpers::{handle_delete_files, handle_proof_bytes};
use crate::errors::VerificationError;
use crate::models::{Sp1Proof, VerificationResult};

pub async fn verify(data: &Sp1Proof) -> Result<VerificationResult, VerificationError> {
    info!("{:?}", data);

    let proof = handle_proof_bytes(&data.proof_file_path)
        .await
        .map_err(|err| {
            return err;
        })?;

    let parsed_proof: SP1ProofWithIO<sp1_core::utils::BabyBearBlake3> = from_str(&proof)
        .map_err(|err| VerificationError::JSONError(err, "Error parsing proof JSON".to_string()))?;

    let elf = fs::read(&data.elf_file_path)
        .map_err(|err| VerificationError::IOError(err, "Error reading ELF file".to_string()))?;
    let sliced_elf = &elf[32..];
    let verification_result = SP1Verifier::verify(&sliced_elf, &parsed_proof);

    handle_delete_files(&vec![&data.proof_file_path, &data.elf_file_path]);

    match verification_result {
        Ok(_) => Ok(VerificationResult { is_valid: true }),
        Err(err) => {
            warn!("Verification failed: {:?}", err);
            Ok(VerificationResult { is_valid: false })
        }
    }
}
