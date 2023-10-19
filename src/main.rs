
use std::fs;
use std::sync::{OnceLock, Arc, Mutex};
use std::process::{Command, Child};
use std::thread;
use lazy_static::lazy_static;
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

static CONFIG: OnceLock<Config> = OnceLock::new();

lazy_static! {
    static ref CHILD: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
}

fn init_config() {

    let config = std::fs::read_to_string("Config.toml").unwrap();
    CONFIG.set(toml::from_str(&config).unwrap()).unwrap();

}

fn spawn_child(bytes: Vec<u8>) {

    if let Some(child) = CHILD.lock().unwrap().as_mut() {
        child.kill().unwrap();
    }

    let executable_name: &str = &CONFIG.get().unwrap().executable_name;

    fs::write(executable_name, bytes).unwrap();

    let child = Command::new(executable_name)
        .spawn()
        .unwrap();

    CHILD.lock().unwrap().replace(child);

}

#[post("/")]
async fn upload_binary(request: web::Json<Binary>) -> impl Responder {

    thread::spawn(move || {
        spawn_child(request.into_inner().bytes);
    });

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