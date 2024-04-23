use jolt::{tracer, Jolt, Proof, RV32IJoltVM};
use log::{info, warn};
use std::path::PathBuf;

use super::helpers::{handle_bytes, handle_delete_files};
use crate::errors::VerificationError;
use crate::models::{JoltProof, VerificationResult};

pub async fn verify(data: &JoltProof) -> Result<VerificationResult, VerificationError> {
    info!("{:?}", data);

    handle_bytes(&data.proof_file_path).map_err(|err| {
        return VerificationError::IOError(err, "Error reading proof file".to_string());
    })?;

    let proof = Proof::from_file(&format!("{}.tmp", &data.proof_file_path)).unwrap();

    handle_bytes(&data.elf_file_path).map_err(|err| {
        return VerificationError::IOError(err, "Error reading elf file".to_string());
    })?;

    let (byte_code, memory_init) =
        tracer::decode(&PathBuf::from(&format!("{}.tmp", &data.elf_file_path)));

    let preproccessing = RV32IJoltVM::preprocess(byte_code, memory_init, 1 << 20, 1 << 20, 1 << 20);

    let verification_result = RV32IJoltVM::verify(preproccessing, proof.proof, proof.commitments);

    handle_delete_files(&vec![&data.proof_file_path, &data.elf_file_path]);

    match verification_result {
        Ok(_) => Ok(VerificationResult { is_valid: true }),
        Err(err) => {
            warn!("Verification failed: {:?}", err);
            Ok(VerificationResult { is_valid: false })
        }
    }
}
