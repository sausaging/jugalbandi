use actix_web::{get, post, web, HttpResponse, Responder};
use log::{info, warn};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::{
    MidenProof, Ping, PingSingle, Ports, ProodDataRisc0, ProofDataMiden, ProofDataSP1, Risc0Proof,
    Sp1Proof, SubmitionResult, VerifyProof,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Verifying proofs for the world!")
}

#[get("/ping")]
async fn ping(port_index: web::Data<Arc<Mutex<usize>>>, ports: web::Data<Ports>) -> impl Responder {
    info!("Pinging the server");
    // will instantiate a new port here on every ping call and return the port number
    let mut port_index = port_index.lock().await;
    if (*port_index) >= ports.instantiated_ports.len() {
        *port_index = 0;
        return HttpResponse::Ok().json(Ping {
            success: false,
            rust_port: "No more ports".to_string(),
            uinit_port: "No more ports".to_string(),
        });
    }
    let instantiated_port = ports.instantiated_ports[*port_index];
    let uninstantiated_port = ports.uninstantiated_ports[*port_index];
    *port_index += 1;
    info!(
        "Instantiated port: {}, Uninstantiated port: {}",
        instantiated_port, uninstantiated_port
    );
    HttpResponse::Ok().json(Ping {
        success: true,
        rust_port: instantiated_port.to_string(),
        uinit_port: uninstantiated_port.to_string(),
    })
}

#[get("/ping-single")]
async fn ping_single() -> impl Responder {
    HttpResponse::Ok().json(PingSingle { success: true })
}

#[post("/sp1-verify")]
async fn verify_sp1(
    sp1_hashmap: web::Data<Arc<Mutex<HashMap<String, Sp1Proof>>>>,
    data: web::Json<ProofDataSP1>,
) -> impl Responder {
    let mut sp1_hashmap = sp1_hashmap.lock().await;
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
async fn verify_miden(
    miden_hashmap: web::Data<Arc<Mutex<HashMap<String, MidenProof>>>>,
    data: web::Json<ProofDataMiden>,
) -> impl Responder {
    let mut miden_hashmap = miden_hashmap.lock().await;
    let proof_data = data.into_inner();
    miden_hashmap.insert(
        proof_data.tx_id.clone(),
        MidenProof {
            program_hash: proof_data.program_hash.clone(),
            inputs_front_end: proof_data.inputs_front_end.clone(),
            outputs_front_end: proof_data.outputs_front_end.clone(),
            proof_file_path: proof_data.proof_file_path.clone(),
        },
    );
    HttpResponse::Ok().json(SubmitionResult { is_submitted: true })
}

#[post("/risc0-verify")]
async fn verify_risc0(
    risc0_hashmap: web::Data<Arc<Mutex<HashMap<String, Risc0Proof>>>>,
    data: web::Json<ProodDataRisc0>,
) -> impl Responder {
    let mut risc0_hashmap = risc0_hashmap.lock().await;
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
async fn verify(
    queue: web::Data<Arc<Mutex<VecDeque<VerifyProof>>>>,
    sp1_hashmap: web::Data<Arc<Mutex<HashMap<String, Sp1Proof>>>>,
    risc0_hashmap: web::Data<Arc<Mutex<HashMap<String, Risc0Proof>>>>,
    miden_hashmap: web::Data<Arc<Mutex<HashMap<String, MidenProof>>>>,
    data: web::Json<VerifyProof>,
) -> impl Responder {
    let proof_data = data.into_inner();
    let mut verify_queue = queue.lock().await;
    match proof_data.verify_type {
        1 => {
            let sp1_hashmap = sp1_hashmap.lock().await;
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
            let miden_hashmap = miden_hashmap.lock().await;
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
            let risc0_hashmap = risc0_hashmap.lock().await;
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
