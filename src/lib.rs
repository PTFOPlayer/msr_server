#![feature(proc_macro_hygiene, decl_macro)]

pub mod data_getters;
mod misc;

use std::io::Cursor;

pub use data_getters::*;
use misc::module_parser::{load_modules, ModuleError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DataToJson {
    cpu: CpuCore,
    memory: Memory,
}

fn process_data(voltage: *mut f64, package_power: *mut f64, time_mul: i32) -> DataToJson {
    let (vendor, name) = name_and_vendor();
    let cs = sys_utils(time_mul);
    let (cpu, mem) = unsafe { cs.produce_final_data(*voltage, *package_power, vendor, name) };
    return DataToJson { cpu, memory: mem };
}

#[no_mangle]
pub extern "C" fn print_json_rs(voltage: *mut f64, package_power: *mut f64, time_mul: i32) {
    if let Ok(serialized) = serde_json::to_string(&process_data(voltage, package_power, time_mul)) {
        println!("{}", serialized);
    } else {
        println!("error serializing data")
    };
}

#[no_mangle]
pub extern "C" fn print_toml_rs(voltage: *mut f64, package_power: *mut f64, time_mul: i32) {
    if let Ok(serialized) = toml::to_string(&&process_data(voltage, package_power, time_mul)) {
        println!("{}", serialized);
    } else {
        println!("error serializing data")
    };
}

use rocket::{get, http::Status, response::Body, routes, Response, State};
struct DataStruct(*mut f64, *mut f64, i32);

unsafe impl Send for DataStruct {}
unsafe impl Sync for DataStruct {}

#[get("/")]
fn full_data(data: State<DataStruct>) -> Result<String, String> {
    let result = process_data(data.0, data.1, data.2);
    match serde_json::to_string(&result) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}

#[get("/cpu")]
fn cpu_data(data: State<DataStruct>) -> Result<String, String> {
    let result = process_data(data.0, data.1, data.2).cpu;
    match serde_json::to_string(&result) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}

#[get("/memory")]
fn memory_data(data: State<DataStruct>) -> Result<String, String> {
    let result = process_data(data.0, data.1, data.2).memory;
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
                Err(err) => {
                    response.set_raw_status(500, "interla server error");
                    response.set_raw_body(Body::Sized(
                        Cursor::new(err.to_string()),
                        err.to_string().len() as u64,
                    ));
                    response
                }
            }
        }
        Err(err) => {
            response.set_raw_status(500, "interla server error");
            response.set_raw_body(Body::Sized(
                Cursor::new(err.to_string()),
                err.to_string().len() as u64,
            ));
            response
        }
    }
}

#[no_mangle]
pub extern "C" fn server_rs(voltage: *mut f64, package_power: *mut f64, time_mul: i32) {
    rocket::ignite()
        .mount("/", routes![full_data, cpu_data, memory_data, modules_data])
        .manage(DataStruct(voltage, package_power, time_mul))
        .launch();
}
