use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;
use log::info;
use std::io;

const CPU_USAGE: f32 = 31.82; 
const TOTAL_MEMORY: u64 = 17179869184; 
const USED_MEMORY: u64 = 11185905664; 
const AVAILABLE_MEMORY: u64 = 5515788288; 
const SERVER_ADDRESS: &str = "127.0.0.1:8080"; 

#[derive(Serialize)]
struct SystemMetrics {
    cpu_usage: f32,
    total_memory: u64,
    used_memory: u64,
    available_memory: u64,
}

fn generate_system_metrics() -> SystemMetrics {
    SystemMetrics {
        cpu_usage: CPU_USAGE,
        total_memory: TOTAL_MEMORY,
        used_memory: USED_MEMORY,
        available_memory: AVAILABLE_MEMORY,
    }
}

async fn get_system_metrics() -> impl Responder {
    info!("Fetching system metrics");
    web::Json(generate_system_metrics())
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize logging
    log4rs::init_file("config/log4rs.yaml", Default::default())
        .map_err(|e| {
            eprintln!("Failed to initialize logging: {}", e);
            io::Error::new(io::ErrorKind::Other, "Logging initialization failed")
        })?;
    
    info!("Starting server at {}", SERVER_ADDRESS);
    
    HttpServer::new(|| {
        App::new()
            .route("/metrics", web::get().to(get_system_metrics))
            .service(fs::Files::new("/", "./").index_file("index.html"))
    })
    .bind(SERVER_ADDRESS)
    .map_err(|e| {
        eprintln!("Failed to bind server: {}", e);
        io::Error::new(io::ErrorKind::Other, "Server binding failed")
    })?
    .run()
    .await
}
