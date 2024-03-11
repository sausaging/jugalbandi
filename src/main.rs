use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::{self};
use log::{info, warn};
use miden_wasm::verify_program;
use risc0_zkvm::Receipt;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use sp1_core::SP1Verifier;
use std::fs;
extern crate bincode;

#[derive(Deserialize, Debug)]
struct ProofDataSP1 {
    proof_file_path: String,
    elf_file_path: String,
}

#[derive(Deserialize, Debug)]
struct ProofDataMiden {
    code_front_end: String,
    inputs_front_end: String,
    outputs_front_end: String,
    proof_file_path: String,
}

#[derive(Deserialize, Debug)]
struct ProodDataRisc0 {
    proof_file_path: String,
    risc_zero_image_id: String,
}

#[derive(Serialize)]
struct VerificationResult {
    is_valid: bool,
}

#[derive(Deserialize, Debug)]
struct Proof {
    proof: Vec<u8>, // Use Vec<u8> to hold the array elements
}

#[derive(Serialize)]
struct Ping {
    success: bool,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Verifying proofs for the world!")
}

#[post("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(Ping { success: true })
}

#[post("/sp1-verify")]
async fn verify_sp1(data: web::Json<ProofDataSP1>) -> impl Responder {
    info!("{:?}", data);
    let proof_data = data.into_inner();
    let proof = proof_data.proof_file_path;
    let elf = proof_data.elf_file_path;
    // Read proof data from file
    let proof_json = fs::read_to_string(&proof);

    match proof_json {
        Ok(proof) => {
            let parsed_proof: Result<
                sp1_core::SP1ProofWithIO<sp1_core::utils::BabyBearBlake3>,
                serde_json::Error,
            > = from_str(&proof);

            match parsed_proof {
                Ok(proof) => {
                    // Read exec_code from disk
                    let elf_res = fs::read(&elf);

                    if elf_res.is_err() {
                        warn!("Error reading ELF file");
                        return HttpResponse::BadRequest()
                            .json(VerificationResult { is_valid: false });
                    }
                    let elf = elf_res.unwrap();
                    // Verification using SP1Verifier
                    let verification_result = SP1Verifier::verify(&elf, &proof);

                    match verification_result {
                        Ok(_) => HttpResponse::Ok().json(VerificationResult { is_valid: true }),
                        Err(err) => {
                            warn!("Verification failed: {:?}", err);
                            HttpResponse::Ok().json(VerificationResult { is_valid: false })
                        }
                    }
                }
                Err(err) => {
                    warn!("Error parsing proof JSON: {}", err);
                    HttpResponse::BadRequest().json(VerificationResult { is_valid: false })
                }
            }
        }
        Err(_) => {
            warn!("Error reading proof file");
            HttpResponse::BadRequest().json(VerificationResult { is_valid: false })
        }
    }
}

#[post("/miden-verify")]
async fn verify_miden(data: web::Json<ProofDataMiden>) -> impl Responder {
    info!("{:?}", data);
    let proof_data = data.into_inner();
    let code_frontend = proof_data.code_front_end;
    let inputs_frontend = proof_data.inputs_front_end;
    let outputs_frontend = proof_data.outputs_front_end;
    let proof_data_result =
        fs::read_to_string(&proof_data.proof_file_path);
    if proof_data_result.is_err() {
        warn!("Error reading proof file");
        return HttpResponse::BadRequest().json(VerificationResult { is_valid: false });
    }
    let proof_data = proof_data_result.unwrap();
    let parsed_data: Proof = serde_json::from_str(&proof_data).unwrap(); //@todo error occured here
    let proof = parsed_data.proof;
    let verification_result =
        verify_program(&code_frontend, &inputs_frontend, &outputs_frontend, proof);
    match verification_result {
        Ok(x) => {
            if x == 96 {
                return HttpResponse::Ok().json(VerificationResult { is_valid: true });
            } else {
                return HttpResponse::Ok().json(VerificationResult { is_valid: false });
            }
        }
        Err(err) => {
            warn!("Verification failed: {:?}", err);
            HttpResponse::Ok().json(VerificationResult { is_valid: false })
        }
    }
}

#[post("/risc0-verify")]
async fn verify_risc0(data: web::Json<ProodDataRisc0>) -> impl Responder {
    info!("{:?}", data);
    let proof_data = data.into_inner();
    let receipt_data = proof_data.proof_file_path;
    let image_id_str = proof_data.risc_zero_image_id;
    let numbers_str: Vec<&str> = image_id_str
        .trim_matches(|c| c == '[' || c == ']')
        .split(", ")
        .collect();
    println!("{:?}", numbers_str);
    if numbers_str.len() != 8 {
        panic!("Expected 8 numbers, found {}", numbers_str.len());
    }
    let mut image_id: [u32; 8] = [0; 8];
    for (i, num_str) in numbers_str.iter().enumerate() {
        image_id[i] = num_str.parse::<u32>().unwrap();
    }

    let receipt_up_result = fs::read_to_string(&receipt_data);
    if receipt_up_result.is_err() {
        warn!("Error reading receipt file");
        return HttpResponse::BadRequest().json(VerificationResult { is_valid: false });
    }
    let receipt_up = receipt_up_result.unwrap();
    let receipt_bytes_result = serde_json::from_str(&receipt_up);
    if receipt_bytes_result.is_err() {
        warn!("Error parsing receipt JSON");
        return HttpResponse::BadRequest().json(VerificationResult { is_valid: false });
    }
    let receipt_bytes: Proof = receipt_bytes_result.unwrap();
    let receipt_result = bincode::deserialize(&receipt_bytes.proof);
    if receipt_result.is_err() {
        warn!("Error deserializing receipt");
        return HttpResponse::BadRequest().json(VerificationResult { is_valid: false });
    }
    let receipt: Receipt = receipt_result.unwrap();
    let verification_result = receipt.verify(image_id);
    match verification_result {
        Ok(_) => {
            return HttpResponse::Ok().json(VerificationResult { is_valid: true });
        }
        Err(err) => {
            warn!("Verification failed: {:?}", err);
            HttpResponse::Ok().json(VerificationResult { is_valid: false })
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(hello)
            .service(verify_sp1)
            .service(verify_miden)
            .service(verify_risc0)
            .service(ping)
    })
    .workers(10)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
