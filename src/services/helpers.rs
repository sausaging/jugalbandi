use log::{warn, info};
use miden::StackOutputs;
use std::fs;
use std::io::{Read, Write};
use std::num::ParseIntError;
use std::fs::{File, OpenOptions};

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

pub fn handle_bytes(source_file: &str) -> std::io::Result<()> {
    // target file is source file + 1;
    let target_file = format!("{}.tmp", source_file);
    let mut source = File::open(source_file)?;
    let mut target = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&target_file)?;

    let mut buffer = [0; 4096]; // Adjust buffer size for performance if needed

    // Skip first 32 bytes
    source.read(&mut buffer[..32])?;

    // Read remaining content and write to new file
    loop {
        let read_bytes = source.read(&mut buffer)?;
        if read_bytes == 0 {
            break;
        }
        target.write_all(&buffer[..read_bytes])?;
    }

    info!("Successfully created '{}' excluding first 32 bytes of '{}'", target_file, source_file);

    Ok(())
}
