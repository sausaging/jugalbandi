use log::{info, warn};
use miden::{Digest, ExecutionProof, Kernel, ProgramInfo, StackInputs};
use std::fs;

use super::helpers::{deserialize_stack_outputs, handle_delete_files, string_to_u64_vec};
use crate::errors::VerificationError;
use crate::models::{MidenProof, Proof, VerificationResult};

pub async fn verify(data: &MidenProof) -> Result<VerificationResult, VerificationError> {
    info!("{:?}", data);
    let program_hash = Digest::try_from(data.program_hash.clone()).map_err(|err| {
        return VerificationError::DigestError(format!("Error parsing program hash: {err:?}"));
    })?;
    let program_info = ProgramInfo::new(program_hash, Kernel::default());
    let inputs_u64 = string_to_u64_vec(&data.inputs_stack).map_err(|err| {
        return VerificationError::ParseError(format!("Error parsing inputs stack JSON: {err:?}"));
    })?;
    info!("Program inputs: {:?}", inputs_u64);
    let stack_inputs = StackInputs::try_from_values(inputs_u64).map_err(|err| {
        return VerificationError::ParseError(format!("Error parsing inputs stack JSON: {err:?}"));
    })?;
    let stack_outputs = deserialize_stack_outputs(&data.outputs_stack).map_err(|err| {
        return VerificationError::ParseError(format!("Error parsing outputs stack JSON: {err:?}"));
    })?;
    // uncomment this when running with hypersdk
    // let proof = handle_proof_bytes(&data.proof_file_path)
    //     .await
    //     .map_err(|err| {
    //         return err;
    //     })?;
    // comment this when running with hypersdk
    let proof = fs::read_to_string(&data.proof_file_path).map_err(|err| {
        return VerificationError::IOError(err, "Error reading proof file".to_string());
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
    let proof = ExecutionProof::from_bytes(&parsed_data.proof).unwrap();

    info!(
        "{:?}, {:?}, {:?}",
        stack_inputs, stack_outputs, program_info
    );
    let verification_result = miden::verify(program_info, stack_inputs, stack_outputs, proof);
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
