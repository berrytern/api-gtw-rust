
use std::collections::HashMap;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, Route};
use serde::{Serialize, Deserialize};
mod routes;
mod configs;
pub mod infrastructure;
use routes::loja::get_loja;
use infrastructure::yaml::controller::load_config;
use infrastructure::yaml::handlers::register_handlers;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

async fn index(info: web::Json<Point>) -> Result<String, ()> {
    Ok(format!("Welcome {}!", info.x))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut config = load_config("something.yaml");
    let mut hosts = register_handlers(config);

    let mut methods:HashMap<String, Box<dyn Fn() -> Route>> = HashMap::new();
    methods.insert("GET".to_string(), Box::new(web::get));
    methods.insert("POST".to_string(), Box::new(web::post));
    methods.insert("PATCH".to_string(), Box::new(web::patch));
    methods.insert("DELETE".to_string(), Box::new(web::delete));
    HttpServer::new(|| {
        let app = App::new();
        for host in hosts {
            for path in host.1{
                for method in path.1{
                    let handle = methods.get(&method.0.to_string());
                    if let Some(hd) = handle{
                        app
                        .route(&path.0, hd());
                    }
                }
            }
        }
        return app;
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}