mod routes;
mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::Commands;
use crate::config::Config;
use std::path::Path;

#[actix_web::main]
pub async fn run_web_server(config: Config, run_type: Commands) -> std::io::Result<()> {
    println!("Running web server in {:?} mode\n Admin Panel: 127.0.0.1:8080", run_type);
    HttpServer::new(|| {
        App::new()
            // .service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
