use log::warn;
use miden::StackOutputs;
use std::fs;
use std::io::Read;
use std::num::ParseIntError;

use crate::config::Config;
use crate::errors::VerificationError;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Outputs {
    pub stack: Vec<u64>,
    pub overflow_addrs: Option<Vec<u64>>,
}

pub fn string_to_u64_vec(s: &str) -> Result<Vec<u64>, ParseIntError> {
    let mut result = Vec::new();
    for num_str in s.trim_matches(|c| c == '[' || c == ']').split(',') {
        let num = num_str.trim().parse::<u64>().unwrap_or(0); // Parse, or set to 0 on error
        result.push(num);
    }
    Ok(result)
}

pub fn deserialize_stack_outputs(outputs_as_str: &str) -> Result<StackOutputs, String> {
    let outputs_as_json: Outputs =
        serde_json::from_str(outputs_as_str).map_err(|e| e.to_string())?;

    let outputs = StackOutputs::new(
        outputs_as_json.stack,
        outputs_as_json.overflow_addrs.unwrap_or(vec![]),
    )
    .unwrap();

    Ok(outputs)
}

pub async fn handle_proof_bytes(proof_file_path: &str) -> Result<String, VerificationError> {
    let mut file = fs::File::open(&proof_file_path)
        .map_err(|err| VerificationError::IOError(err, "Error opening receipt file".to_string()))?;

    let mut buffer: Vec<u8> = vec![0; 32];

    file.read_exact(&mut buffer).map_err(|err| {
        VerificationError::IOError(err, "Error reading first 32 bytes".to_string())
    })?;

    let mut remaining_content = String::new();
    file.read_to_string(&mut remaining_content).map_err(|err| {
        VerificationError::IOError(err, "Error reading remaining content".to_string())
    })?;

    Ok(remaining_content)
}

pub fn handle_delete_files(files: &Vec<&String>) {
    let config = Config::init();
    if config.delete_files {
        for file in files {
            let _ =
                std::fs::remove_file(file).map_err(|err| warn!("Error deleting file: {:?}", err));
        }
    }
}
