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
    proof: String,
    elf: String,
}

#[derive(Deserialize, Debug)]
struct ProofDataMiden {
    code_frontend: String,
    inputs_frontend: String,
    outputs_frontend: String,
    proofs_frontend: String,
}

#[derive(Deserialize, Debug)]
struct ProodDataRisc0 {
    receipt: String,
    image_id: [u32; 8],
}

#[derive(Serialize)]
struct VerificationResult {
    is_valid: bool,
}

#[derive(Deserialize, Debug)]
struct Proof {
    proof: Vec<u8>, // Use Vec<u8> to hold the array elements
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Verifying proofs for the world!")
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("true")
}

#[post("/verify-sp1")]
async fn verify_sp1(data: web::Json<ProofDataSP1>) -> impl Responder {
    info!("{:?}", data);
    let proof_data = data.into_inner();
    let proof = proof_data.proof;
    let elf = proof_data.elf;
    // Read proof data from file
    let proof_json = fs::read_to_string(&proof).expect("Failed to read proof file");

    // Attempt to deserialize JSON proof
    let parsed_proof: Result<
        sp1_core::SP1ProofWithIO<sp1_core::utils::BabyBearBlake3>,
        serde_json::Error,
    > = from_str(&proof_json);

    match parsed_proof {
        Ok(proof) => {
            // Read exec_code from disk
            let elf = fs::read(&elf).expect("Failed to read elf");
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

#[post("/verify-miden")]
async fn verify_miden(data: web::Json<ProofDataMiden>) -> impl Responder {
    info!("{:?}", data);
    let proof_data = data.into_inner();
    let code_frontend = proof_data.code_frontend;
    let inputs_frontend = proof_data.inputs_frontend;
    let outputs_frontend = proof_data.outputs_frontend;
    let proof_data =
        fs::read_to_string(&proof_data.proofs_frontend).expect("Failed to read proof file");
    let parsed_data: Proof = serde_json::from_str(&proof_data).unwrap();
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

#[post("/verify-risc0")]
async fn verify_risc0(data: web::Json<ProodDataRisc0>) -> impl Responder {
    info!("{:?}", data);
    let proof_data = data.into_inner();
    let receipt_data = proof_data.receipt;
    let image_id = proof_data.image_id;
    let receipt_up = fs::read_to_string(&receipt_data).expect("Failed to read receipt file");
    let receipt_bytes: Proof = serde_json::from_str(&receipt_up).unwrap();
    let receipt: Receipt = bincode::deserialize(&receipt_bytes.proof).unwrap();
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
    })
    .workers(10)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
