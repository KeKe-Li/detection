use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder};
use log::{info, error};
use serde::Serialize;

#[derive(Serialize)]
struct SystemMetrics {
    cpu_usage: f32,
    total_memory: u64,
    used_memory: u64,
    available_memory: u64,
}

// Extract the logic to generate system metrics
fn generate_system_metrics() -> SystemMetrics {
    SystemMetrics {
        cpu_usage: 31.82, // constant
        total_memory: 17179869184, 
        used_memory: 11185905664, 
        available_memory: 5515788288, 
    }
}

async fn get_system_metrics() -> impl Responder {
    // Use serde_json to return JSON directly
    web::Json(generate_system_metrics())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let address = "127.0.0.1:8080"; // Can be read from the configuration file
    info!("Starting server at {}", address);
    HttpServer::new(|| {
        App::new()
            .route("/metrics", web::get().to(get_system_metrics))
            .service(fs::Files::new("/", "./").index_file("index.html"))
    })
    .bind(address)
    .map_err(|e| {
        error!("Failed to bind server: {}", e);
        e
    })?
    .run()
    .await
}
