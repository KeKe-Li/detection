use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct SystemMetrics {
    cpu_usage: f32,
    total_memory: u64,
    used_memory: u64,
    available_memory: u64,
}

async fn get_system_metrics() -> impl Responder {
    web::Json(SystemMetrics {
        cpu_usage: 31.82,
        total_memory: 17179869184,
        used_memory: 11185905664,
        available_memory: 5515788288,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/metrics", web::get().to(get_system_metrics))
            .service(fs::Files::new("/", "./").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
