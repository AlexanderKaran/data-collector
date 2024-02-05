use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use chrono::{DateTime, Utc};

use std::sync::{Mutex, RwLock};

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(long, default_value_t = 8080)]
    port: u16,
}

struct AppState {
    start: RwLock<DateTime<Utc>>,
    pings: Mutex<usize>,
}

impl AppState {
    fn init() -> AppState {
        let now = chrono::offset::Utc::now();
        AppState {
            start: RwLock::new(now),
            pings: Mutex::new(0),
        }
    }
}

#[post("/ping")]
async fn ping(data: web::Data<AppState>) -> impl Responder {
    let mut pings = data.pings.lock().unwrap();
    *pings += 1;

    HttpResponse::Ok()
        .body("")
}


#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!\r\n"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> String {
    format!("Hello {}!\r\n", &name)
}


#[post("/flush")]
async fn flush(data: web::Data<AppState>) -> impl Responder {
    let orig_start = {
        let orig_start = *data.start.read().unwrap();
        let mut start = data.start.write().unwrap();
        *start = chrono::offset::Utc::now();
        orig_start
    };

    let orig_pings = {
        let mut pings = data.pings.lock().unwrap();
        let orig_pings = *pings;
        *pings = 0;
        orig_pings
    };

    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .body(format!("{{\"start\": \"{orig_start:?}\", \"pings\": {orig_pings} }}\r\n"))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let initial_state = web::Data::new(AppState::init());

    let app = move || {
        App::new()
            .app_data(initial_state.clone())
            .service(index)
            .service(ping)
            .service(flush)
    };

    HttpServer::new(app)
        .bind((args.host, args.port))?
        .run()
        .await

}