
use std::sync::OnceLock;
use serde::Deserialize;

use actix_web::{
    post, 
    App,
    web,
    HttpResponse, 
    HttpServer, 
    Responder
};

#[derive(Deserialize, Debug)]
pub struct Binary {
    pub bytes: Vec<u8>
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub executable_name: String
}

static Config: OnceLock<Config> = OnceLock::new();


fn init_config() {

    let config = std::fs::read_to_string("Config.toml").unwrap();
    Config.set(toml::from_str(&config).unwrap()).unwrap();

}

#[post("/")]
async fn upload_binary(request: web::Json<Binary>) -> impl Responder {

    HttpResponse::Ok()
        .body("")

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    init_config();

    HttpServer::new(|| {
        App::new()
            .service(upload_binary)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}