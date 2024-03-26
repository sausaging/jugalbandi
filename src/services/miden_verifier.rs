use log::{info, warn};
use miden_wasm::verify_program;

use crate::config::{handle_delete_files, handle_proof_bytes};
use crate::errors::VerificationError;
use crate::models::{MidenProof, Proof, VerificationResult};

pub async fn verify(data: &MidenProof) -> Result<VerificationResult, VerificationError> {
    info!("{:?}", data);
    let code_frontend = &data.code_front_end;
    let inputs_frontend = &data.inputs_front_end;
    let outputs_frontend = &data.outputs_front_end;
    let proof = handle_proof_bytes(&data.proof_file_path)
        .await
        .map_err(|err| {
            return err;
        })?;
    let parsed_data: Proof = match serde_json::from_str(&proof) {
        Ok(x) => x,
        Err(err) => {
            return Err(VerificationError::JSONError(
                err,
                "Error parsing proof JSON".to_string(),
            ))
        }
    };
    let verification_result = verify_program(
        &code_frontend,
        &inputs_frontend,
        &outputs_frontend,
        parsed_data.proof,
    );
    let is_valid = match verification_result {
        Ok(x) => x == 96,
        Err(err) => {
            warn!("Verification failed : {:?}", err);
            false
        }
    };
    handle_delete_files(&vec![&data.proof_file_path]);
    return Ok(VerificationResult { is_valid });
}
