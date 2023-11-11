#![feature(proc_macro_hygiene, decl_macro)]

pub mod data_getters;
mod misc;

use std::{
    env::args,
    io::Cursor,
};

pub use data_getters::*;
use misc::module_parser::{load_modules, ModuleError};
use serde::{Deserialize, Serialize};

const TIME_MUL: i32 = 5;

#[derive(Debug, Serialize, Deserialize)]
struct DataToJson {
    core: CoreStat,
}

fn process_data() -> DataToJson {
    let core = CORE_STAT.clone().update(get_voltage(), get_power());
    return DataToJson { core };
}

#[no_mangle]
pub extern "C" fn print_json() {
    match serde_json::to_string(&process_data()) {
        Ok(ser) => println!("{}", ser),
        Err(_) => println!("error serializing data"),
    };
}

#[no_mangle]
pub extern "C" fn print_toml() {
    match toml::to_string(&&process_data()) {
        Ok(ser) => println!("{}", ser),
        Err(_) => println!("error serializing data"),
    };
}

use rocket::{get, response::Body, routes, Response};

#[get("/")]
fn full_data() -> Result<String, String> {
    let result = process_data();
    match serde_json::to_string(&result) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}

#[no_mangle]
pub extern "C" fn server() {
    rocket::ignite()
        .mount("/", routes![full_data, modules_data])
        .launch();
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

fn main() {
    let args = args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("error: wrong ammount of arguments (max 1)\n -r: access via rest api\n -t: output to terminal in toml format\n -j: output to terminal in json format");
    }

    match args[1].as_str() {
        "-r" => server(),
        "-t" => print_toml(),
        "-j" => print_json(),

        &_ => {
            println!("argument not recognized");
        }
    }
}
