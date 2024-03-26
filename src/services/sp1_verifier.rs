use log::{info, warn};
use serde_json::from_str;
use sp1_core::{SP1ProofWithIO, SP1Verifier};
use std::fs;
use std::io::Read;

use crate::config::handle_delete_files;
use crate::errors::VerificationError;
use crate::models::{Sp1Proof, VerificationResult};

pub async fn verify(data: &Sp1Proof) -> Result<VerificationResult, VerificationError> {
    info!("{:?}", data);
    let mut file = fs::File::open(&data.proof_file_path)
        .map_err(|err| VerificationError::IOError(err, "Error opening proof file".to_string()))?;

    // remove the first 32 bytes from the file i.e hash of the file
    let mut buffer = vec![0; 32];
    file.read_exact(&mut buffer).map_err(|err| {
        VerificationError::IOError(err, "Error reading first 32 bytes".to_string())
    })?;

    let mut remaining_content = String::new();
    file.read_to_string(&mut remaining_content).map_err(|err| {
        VerificationError::IOError(err, "Error reading remaining content".to_string())
    })?;

    let parsed_proof: SP1ProofWithIO<sp1_core::utils::BabyBearBlake3> =
        from_str(&remaining_content).map_err(|err| {
            VerificationError::JSONError(err, "Error parsing proof JSON".to_string())
        })?;

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
