use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use tokio::task;

use crate::config::{Config, process_verification_queue};
use crate::logging::init_logger;
use crate::routes::{
    hello, ping, verify_miden, verify_risc0, verify_sp1,
};

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
    let queue = storage::VERIFY_QUEUE.clone();
    task::spawn(async move {
        process_verification_queue(&queue).await;  // Call with reference
      });
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
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
