pub mod data_getters;
mod misc;
mod server;
use server::server;
use std::env::args;

pub use data_getters::*;

const TIME_MUL: i32 = 5;

#[inline(always)]
fn process_data() -> CoreStat {
    let data = CORE_STAT.clone().update(get_voltage(), get_power());
    data
}

fn print_json() {
    match serde_json::to_string(&process_data()) {
        Ok(ser) => println!("{}", ser),
        Err(_) => println!("error serializing data"),
    };
}

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("error: wrong ammount of arguments (max 1)\n -r: access via rest api\n -j: output to terminal in json format");
    }

    match args[1].as_str() {
        "-r" => server(),
        "-j" => Ok(print_json()),

        &_ => Ok({
            println!("argument not recognized");
        }),
    }
}
