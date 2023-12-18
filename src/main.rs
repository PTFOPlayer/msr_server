#![feature(proc_macro_hygiene, decl_macro)]

pub mod data_getters;
mod misc;
mod server;
use server::server;
use std::env::args;

pub use data_getters::*;
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

fn print_json() {
    match serde_json::to_string(&process_data()) {
        Ok(ser) => println!("{}", ser),
        Err(_) => println!("error serializing data"),
    };
}

fn main() {
    let args = args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("error: wrong ammount of arguments (max 1)\n -r: access via rest api\n -j: output to terminal in json format");
    }

    match args[1].as_str() {
        "-r" => server(),
        "-j" => print_json(),

        &_ => {
            println!("argument not recognized");
        }
    }
}
