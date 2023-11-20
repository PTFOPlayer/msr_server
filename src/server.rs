use std::io::Cursor;

pub use rocket::{get, response::Body, routes, Response};
use serde::Serialize;

use crate::{
    misc::{
        self,
        module_parser::{load_modules, ModuleError},
    },
    process_data,
};

pub fn server() {
    rocket::ignite()
        .mount("/", routes![full_data, modules_data])
        .launch();
}

#[get("/")]
fn full_data() -> Result<String, String> {
    let result = process_data();
    match serde_json::to_string(&result) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Serialize, Debug)]
struct Modules {
    modules: Vec<String>,
}

lazy_static::lazy_static! {
    static ref MODULES: Result<misc::module_parser::Modules, ModuleError> = load_modules();
}

#[inline(always)]
fn generate_error(err: String) -> Response<'static> {
    let mut response = Response::new();
    response.set_raw_status(500, "interla server error");
    response.set_raw_body(Body::Sized(Cursor::new(err.clone()), err.len() as u64));
    response
}

#[get("/modules")]
fn modules_data() -> Response<'static> {
    let mut response = Response::new();

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
                Ok(res) => {
                    response.set_raw_body(Body::Sized(Cursor::new(res.clone()), res.len() as u64));
                    response
                }
                Err(err) => generate_error(err.to_string()),
            }
        }
        Err(err) => generate_error(err.to_string()),
    }
}
