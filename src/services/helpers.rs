use log::info;
use std::num::ParseIntError;
use miden::StackOutputs;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Outputs {
    pub stack_output: Vec<u64>,
    pub overflow_addrs: Option<Vec<u64>>,
}

pub fn string_to_u64_vec(s: &str) -> Result<Vec<u64>, ParseIntError> {
    // Split the string by commas (,) with trim to remove leading/trailing spaces
    info!("Parsing string: {:?}", s);
    let mut result = Vec::new();
    for num_str in s.trim_matches(|c| c == '[' || c == ']').split(',') {
        let num = num_str.trim().parse::<u64>().unwrap_or(0); // Parse, or set to 0 on error
        result.push(num);
    }
    Ok(result)
}

pub fn deserialize_stack_outputs(outputs_as_str: &str) -> Result<StackOutputs, String> {
    info!("output as str: {:?}", outputs_as_str);
    let outputs_as_json: Outputs =
        serde_json::from_str(outputs_as_str).map_err(|e| e.to_string())?;

    let outputs = StackOutputs::new(
        outputs_as_json.stack_output,
        outputs_as_json.overflow_addrs.unwrap_or(vec![]),
    ).unwrap();

    Ok(outputs)
}