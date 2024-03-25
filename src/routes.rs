use actix_web::{get, post, web, HttpResponse, Responder};
use log::warn;
use std::process::Command;

use crate::models::{
    MidenProof, Ping, ProodDataRisc0, ProofDataMiden, ProofDataSP1, Risc0Proof, Sp1Proof,
    SubmitionResult, VerifyProof,
};
use crate::services::{miden_verifier, risc0_verifier, sp1_verifier};
use crate::storage::{
    CURRENT_PORT, INSTANTIATED_PORTS, MIDEN_HASHMAP, RISC0_HASHMAP, SP1_HASHMAP,
    UNINSTANTIATED_PORTS, VERIFY_QUEUE,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Verifying proofs for the world!")
}

#[get("/ping")]
async fn ping() -> impl Responder {
    // will instantiate a new port here on every ping call and return the port number
    let mut instantiated_ports = INSTANTIATED_PORTS.lock().unwrap();
    let mut uninstantiated_ports = UNINSTANTIATED_PORTS.lock().unwrap();
    let mut current_port = CURRENT_PORT.lock().unwrap();
    let instantiated_port = instantiated_ports.pop().unwrap();
    let uninstantiated_port = uninstantiated_ports.pop().unwrap();
    *current_port = uninstantiated_port;
    HttpResponse::Ok().json(Ping {
        success: true,
        instantiated_port,
        uninstantiated_port,
    })
}

#[post("/sp1-verify")]
async fn verify_sp1(data: web::Json<ProofDataSP1>) -> impl Responder {
    let mut sp1_hashmap = SP1_HASHMAP.lock().unwrap();
    let proof_data = data.into_inner();
    sp1_hashmap.insert(
        proof_data.tx_id.clone(),
        Sp1Proof {
            proof_file_path: proof_data.proof_file_path.clone(),
            elf_file_path: proof_data.elf_file_path.clone(),
        },
    );
    HttpResponse::Ok().json(SubmitionResult { is_submitted: true })
}

#[post("/miden-verify")]
async fn verify_miden(data: web::Json<ProofDataMiden>) -> impl Responder {
    let mut miden_hashmap = MIDEN_HASHMAP.lock().unwrap();
    let proof_data = data.into_inner();
    miden_hashmap.insert(
        proof_data.tx_id.clone(),
        MidenProof {
            code_front_end: proof_data.code_front_end.clone(),
            inputs_front_end: proof_data.inputs_front_end.clone(),
            outputs_front_end: proof_data.outputs_front_end.clone(),
            proof_file_path: proof_data.proof_file_path.clone(),
        },
    );
    HttpResponse::Ok().json(SubmitionResult { is_submitted: true })
}

#[post("/risc0-verify")]
async fn verify_risc0(data: web::Json<ProodDataRisc0>) -> impl Responder {
    let mut risc0_hashmap = RISC0_HASHMAP.lock().unwrap();
    let proof_data = data.into_inner();
    risc0_hashmap.insert(
        proof_data.tx_id.clone(),
        Risc0Proof {
            proof_file_path: proof_data.proof_file_path.clone(),
            risc_zero_image_id: proof_data.risc_zero_image_id.clone(),
        },
    );
    HttpResponse::Ok().json(SubmitionResult { is_submitted: true })
}

#[post("/verify")]
async fn verify(data: web::Json<VerifyProof>) -> impl Responder {
    let proof_data = data.into_inner();
    let mut verify_queue = VERIFY_QUEUE.lock().unwrap();
    match proof_data.proof_type {
        1 => {
            let sp1_hashmap = SP1_HASHMAP.lock().unwrap();
            match sp1_hashmap.get(&proof_data.tx_id) {
                Some(_sp1_proof) => {
                    verify_queue.push_back(proof_data);
                }
                None => {
                    warn!("Invalid SP1 proof ID");
                    return HttpResponse::Ok().json(SubmitionResult {
                        is_submitted: false,
                    });
                }
            }
        }
        2 => {
            let miden_hashmap = MIDEN_HASHMAP.lock().unwrap();
            match miden_hashmap.get(&proof_data.tx_id) {
                Some(_miden_proof) => {
                    verify_queue.push_back(proof_data);
                }
                None => {
                    warn!("Invalid MIDEN proof ID");
                    return HttpResponse::Ok().json(SubmitionResult {
                        is_submitted: false,
                    });
                }
            }
        }
        3 => {
            let risc0_hashmap = RISC0_HASHMAP.lock().unwrap();
            match risc0_hashmap.get(&proof_data.tx_id) {
                Some(_risc0_proof) => {
                    verify_queue.push_back(proof_data);
                }
                None => {
                    warn!("Invalid RISC0 proof ID");
                    return HttpResponse::Ok().json(SubmitionResult {
                        is_submitted: false,
                    });
                }
            }
        }
        _ => {
            warn!("Invalid proof type");
            return HttpResponse::Ok().json(SubmitionResult {
                is_submitted: false,
            });
        }
    }
    HttpResponse::Ok().json(SubmitionResult { is_submitted: true })
}
