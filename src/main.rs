use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use storage::CURRENT_PORT;
use tokio::task;

use crate::config::{process_verification_queue, Config};
use crate::logging::init_logger;
use crate::routes::{hello, ping, verify, verify_miden, verify_risc0, verify_sp1};
use crate::storage::{MIDEN_HASHMAP, RISC0_HASHMAP, SP1_HASHMAP, VERIFY_QUEUE};

mod config;
mod errors;
mod logging;
mod models;
mod routes;
mod services;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init();
    init_logger();
    let queue = VERIFY_QUEUE.clone();
    let sp1_hashmap = SP1_HASHMAP.clone();
    let risc0_hashmap = RISC0_HASHMAP.clone();
    let miden_hashmap = MIDEN_HASHMAP.clone();
    let current_port = CURRENT_PORT.clone();
    let instanstiated_ports = storage::INSTANTIATED_PORTS.clone();
    let uninstantiated_ports = storage::UNINSTANTIATED_PORTS.clone();
    task::spawn(process_verification_queue(
        queue.clone(),
        sp1_hashmap.clone(),
        risc0_hashmap.clone(),
        miden_hashmap.clone(),
        current_port.clone(),
    ));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(queue.clone()))
            .app_data(web::Data::new(sp1_hashmap.clone()))
            .app_data(web::Data::new(risc0_hashmap.clone()))
            .app_data(web::Data::new(miden_hashmap.clone()))
            .app_data(web::Data::new(current_port.clone()))
            .app_data(web::Data::new(instanstiated_ports.clone()))
            .app_data(web::Data::new(uninstantiated_ports.clone()))
            .service(hello)
            .service(verify_sp1)
            .service(verify_miden)
            .service(verify_risc0)
            .service(verify)
            .service(ping)
    })
    .workers(config.workers)
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
