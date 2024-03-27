use actix_cors::Cors;
use actix_web::{get, main, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

use crate::{
    get_drives, get_system,
    misc::{
        self,
        module_parser::{load_modules, ModuleError},
    },
    process_data,
};

#[main]
pub async fn server() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(hardware_data)
            .service(modules_data)
            .service(drives_data)
            .service(system_data)
    })
    .bind(("127.0.0.1", 7172))?
    .run()
    .await
}

#[get("/")]
async fn hardware_data() -> impl Responder {
    let result = match process_data() {
        Ok(res) => res,
        Err(err) => return HttpResponse::InternalServerError().body(err),
    };

    match serde_json::to_string(&result) {
        Ok(res) => return HttpResponse::Ok().body(res),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/drives")]
async fn drives_data() -> impl Responder {
    let result = get_drives();
    match serde_json::to_string(&result) {
        Ok(res) => HttpResponse::Ok().body(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/system")]
async fn system_data() -> impl Responder {
    let result = get_system();
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
