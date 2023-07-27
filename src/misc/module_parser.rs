
use serde::Deserialize;
use serde_json;
use std::{fs, io::BufReader};

#[derive(Debug)]
pub enum Errors {
    LoadingError(String),
    ModuleParsingError(String)
}

#[derive(Deserialize,Debug)]
pub enum Mode {
    File {dir_path: Option<String>, file_name: String},
    Command {dir_path: Option<String>, command: String, args: Option<Vec<String>>},
    Api {url: String}
}

#[derive(Deserialize,Debug)]
pub enum InputType {
    Text{default_data: Option<String>, format: Option<String>},
    Json{default_data: Option<String>, format: Option<String>},
    Toml{default_data: Option<String>, format: Option<String>}
}

#[derive(Deserialize, Debug)]
pub struct Module {
    name: String,
    mode: Mode,
    input_type: InputType
}

#[derive(Deserialize, Debug)]
pub struct Modules {
    modules: Vec<Module>
}

pub fn load_modules() -> Result<Modules, Errors>{
    if let Ok(file) = fs::File::open("./src/misc/modules.json") {
        let reader = BufReader::new(file);

        let data = match serde_json::from_reader(reader) {
            Ok(res) => res,
            Err(err) => return Err(Errors::ModuleParsingError(err.to_string())),
        };
        
        return Ok(data);
    } else {
        return Err(Errors::LoadingError("error loading ./modules.json".to_owned()))
    }
}

pub fn parse_input(module: Module)  {
    match module.input_type {
        InputType::Text { default_data, format } => {

        },
        InputType::Json { default_data, format } => {

        },
        InputType::Toml { default_data, format } => {

        },
    }
}

pub fn fetch_data(mode: Mode) {
    match mode {
        Mode::File { dir_path, file_name } => {

        },
        Mode::Command { dir_path, command, args } => {

        },
        Mode::Api { url } => {
        },
    }
}