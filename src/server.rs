use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

use crate::{
    misc::{
        self,
        module_parser::{load_modules, ModuleError},
    },
    process_data,
};

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(full_data).service(modules_data))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

#[get("/")]
async fn full_data() -> impl Responder {
    let result = process_data();
    match serde_json::to_string(&result) {
        Ok(res) => HttpResponse::Ok().body(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(Serialize, Debug)]
struct Modules {
    modules: Vec<String>,
}

lazy_static::lazy_static! {
    static ref MODULES: Result<misc::module_parser::Modules, ModuleError> = load_modules();
}

#[get("/")]
async fn modules_data() -> impl Responder {
    match MODULES.as_ref() {
        Ok(modules) => {
            let mut vec = vec![];
            for module in &modules.modules {
                match module.clone().parse_input() {
                    Ok(data) => vec.push(data.to_string()),
                    Err(err) => println!("{}", err.to_string()),
                }
            }
            match serde_json::to_string(&Modules { modules: vec }) {
                Ok(res) => HttpResponse::Ok().body(res),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
