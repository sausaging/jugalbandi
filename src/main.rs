use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use sp1_core::SP1Verifier;
use std::fs;

#[derive(Deserialize, Debug)]
struct ProofData {
    proof: String,
    elf: String,
}

#[derive(Serialize)]
struct VerificationResult {
    is_valid: bool,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Verifying proofs for the world!")
}

#[post("/verify")]
async fn verify_proof(data: web::Json<ProofData>) -> impl Responder {
    println!("{:?}", data);
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
                    eprintln!("Verification failed: {:?}", err);
                    HttpResponse::Ok().json(VerificationResult { is_valid: false })
                }
            }
        }
        Err(err) => {
            eprintln!("Error parsing proof JSON: {}", err);
            HttpResponse::BadRequest().json(VerificationResult { is_valid: false })
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(verify_proof))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
