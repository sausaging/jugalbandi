use log::{info, warn};
use std::path::PathBuf;
use jolt::{Proof, RV32IJoltVM, tracer, Jolt};

use super::helpers::handle_delete_files;
use crate::errors::VerificationError;
use crate::models::{Sp1Proof, VerificationResult};

pub async fn verify(data: &Sp1Proof) -> Result<VerificationResult, VerificationError> {
    info!("{:?}", data);

    let proof = Proof::from_file(&data.proof_file_path).unwrap();

    let (byte_code, memory_init) = tracer::decode(&PathBuf::from(&data.elf_file_path));

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
