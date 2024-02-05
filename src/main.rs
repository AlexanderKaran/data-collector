use actix_web::{get, web, App, HttpServer, Responder};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(long, default_value_t = 8080)]
    port: u16
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!\r\n"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> String {
    format!("Hello {}!\r\n", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    HttpServer::new(|| App::new().service(index).service(hello))
        .bind((args.host, args.port))?
        .run()
        .await
}