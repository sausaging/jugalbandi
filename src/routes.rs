use actix_web::{get, post, web, HttpResponse, Responder};
use log::warn;

use crate::models::{ProofDataMiden, ProofDataSP1, VerificationResult, Ping, ProodDataRisc0};
use crate::services::{miden_verifier, risc0_verifier, sp1_verifier};
use crate::errors::VerificationError;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Verifying proofs for the world!")
}

#[post("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(Ping { success: true })
}

fn handle_response(result: Result<VerificationResult, VerificationError>) -> HttpResponse {
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => {
            warn!("Verification Error: {:?}", err);
            HttpResponse::BadRequest().json(VerificationResult { is_valid: false })
        }
    }
}

#[post("/sp1-verify")]
async fn verify_sp1(data: web::Json<ProofDataSP1>) -> impl Responder {
    let result = sp1_verifier::verify(data.into_inner()).await;
    handle_response(result)
}

#[post("/miden-verify")]
async fn verify_miden(data: web::Json<ProofDataMiden>) -> impl Responder {
    let result = miden_verifier::verify(data.into_inner()).await;
    handle_response(result)
}

#[post("/risc0-verify")]
async fn verify_risc0(data: web::Json<ProodDataRisc0>) -> impl Responder {
    let result = risc0_verifier::verify(data.into_inner()).await;
    handle_response(result)
}
