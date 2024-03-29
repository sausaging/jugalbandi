use log::{info, warn};
use miden::{Digest, ExecutionProof, Kernel, ProgramInfo, StackInputs, StackOutputs};

use super::helpers::{string_to_u64_vec, deserialize_stack_outputs};
use crate::config::{handle_delete_files, handle_proof_bytes};
use crate::errors::VerificationError;
use crate::models::{MidenProof, Proof, VerificationResult};

pub async fn verify(data: &MidenProof) -> Result<VerificationResult, VerificationError> {
    info!("{:?}", data);
    let program_hash = Digest::try_from(data.program_hash.clone()).unwrap();
    let program_info = ProgramInfo::new(program_hash, Kernel::default());
    let inputs_u64 = string_to_u64_vec(&data.inputs_front_end).unwrap();
    let stack_inputs = StackInputs::try_from_values(inputs_u64).unwrap();
    let stack_outputs = deserialize_stack_outputs(&data.outputs_front_end).unwrap();
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
    let proof = ExecutionProof::from_bytes(&parsed_data.proof)
    .unwrap();
    let verification_result = miden::verify(
        program_info,
        stack_inputs,
        stack_outputs,
        proof,
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
