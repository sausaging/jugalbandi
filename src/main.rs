use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

use crate::logging::init_logger;
use crate::routes::{hello, ping, verify_sp1, verify_miden, verify_risc0};

mod logging;
mod errors;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    init_logger();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
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
