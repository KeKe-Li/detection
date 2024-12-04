use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files as fs;
use std::sync::{Arc, Mutex};
use sysinfo::{System, SystemExt};
use crate::monitor::{get_detailed_metrics, DetailedMetrics};

pub struct AppState {
    system: System,
    metrics_history: Vec<DetailedMetrics>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            system: System::new_all(),
            metrics_history: Vec::new(),
        }
    }
}

async fn get_metrics(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let result = data.lock().map_err(|e| {
        println!("Failed to lock state: {:?}", e);
        HttpResponse::InternalServerError().json("Internal server error")
    });

    if let Ok(mut state) = result {
        state.system.refresh_all();
        let metrics = get_detailed_metrics(&mut state.system);
        
        // Save historical data
        state.metrics_history.push(metrics.clone());
        if state.metrics_history.len() > 100 {
            state.metrics_history.remove(0);
        }
        
        HttpResponse::Ok().json(metrics)
    } else {
        HttpResponse::InternalServerError().json("Failed to collect metrics")
    }
}

async fn get_history(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    match data.lock() {
        Ok(state) => HttpResponse::Ok().json(&state.metrics_history),
        Err(e) => {
            println!("Failed to lock state: {:?}", e);
            HttpResponse::InternalServerError().json("Internal server error")
        }
    }
}

pub async fn start_server() -> std::io::Result<()> {
    let state = web::Data::new(Arc::new(Mutex::new(AppState::default())));

    println!("Starting server at http://localhost:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(
                web::scope("/api")
                    .route("/metrics", web::get().to(get_metrics))
                    .route("/history", web::get().to(get_history))
            )
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 