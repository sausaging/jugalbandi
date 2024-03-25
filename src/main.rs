use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use tokio::task;

use crate::config::{process_verification_queue, Config};
use crate::logging::init_logger;
use crate::routes::{hello, ping, verify_miden, verify_risc0, verify_sp1};
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
    task::spawn(process_verification_queue(
        queue.clone(),
        sp1_hashmap.clone(),
        risc0_hashmap.clone(),
        miden_hashmap.clone(),
    ));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(queue.clone()))
            .app_data(web::Data::new(sp1_hashmap.clone()))
            .app_data(web::Data::new(risc0_hashmap.clone()))
            .app_data(web::Data::new(miden_hashmap.clone()))
            .service(hello)
            .service(verify_sp1)
            .service(verify_miden)
            .service(verify_risc0)
            .service(ping)
    })
    .workers(config.workers)
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
