use bincode::deserialize;
use log::{info, warn};
use risc0_zkvm::Receipt;
use serde_json::from_str;
use std::fs;

use crate::config::handle_delete_files;
use crate::errors::VerificationError;
use crate::models::{ProodDataRisc0, Proof, VerificationResult};

pub async fn verify(data: ProodDataRisc0) -> Result<VerificationResult, VerificationError> {
    info!("{:?}", data);
    let receipt_data = data.proof_file_path.clone();
    let image_id_str = data.risc_zero_image_id;
    let numbers_str: Vec<&str> = image_id_str
        .trim_matches(|c| c == '[' || c == ']')
        .split(", ")
        .collect();
    if numbers_str.len() != 8 {
        return Err(VerificationError::InvalidImageID(
            "Invalid image ID length".to_string(),
        ));
    }
    let mut image_id: [u32; 8] = [0; 8];
    for (i, num_str) in numbers_str.iter().enumerate() {
        image_id[i] = match num_str.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                return Err(VerificationError::InvalidImageID(
                    "Invalid image ID number".to_string(),
                ));
            }
        }
    }
    let receipt_up = fs::read_to_string(receipt_data)
        .map_err(|err| VerificationError::IOError(err, "Error reading receipt file".to_string()))?;

    let receipt_bytes: Proof = from_str(&receipt_up).map_err(|err| {
        VerificationError::JSONError(err, "Error parsing receipt JSON".to_string())
    })?;

    let receipt: Receipt = deserialize(&receipt_bytes.proof).map_err(|err| {
        VerificationError::BincodeError(err, "Error deserializing receipt".to_string())
    })?;

    let verification_result = receipt.verify(image_id);

    handle_delete_files(&vec![data.proof_file_path]);
    match verification_result {
        Ok(_) => Ok(VerificationResult { is_valid: true }),
        Err(err) => {
            warn!("Verification failed: {:?}", err);
            Ok(VerificationResult { is_valid: false })
        }
    }
}