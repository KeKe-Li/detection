use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    memory_monitor::web::start_server().await
}
