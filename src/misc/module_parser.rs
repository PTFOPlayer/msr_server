use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::{fs, string::FromUtf8Error};

#[derive(Debug)]
pub enum ModuleError {
    ModuleLoadingError(String),
    ModuleParsingError(String),
    ModuleExecutionError(String),
    ModuleDataParsingError(String),
    ModuleSettingsNotFulfilled(String),
    JsonParsingError(serde_json::Error),
    TomlParsingErrorDe(toml::de::Error),
    TomlParsingErrorSe(toml::ser::Error),
}

impl From<FromUtf8Error> for ModuleError {
    fn from(value: FromUtf8Error) -> Self {
        ModuleError::ModuleDataParsingError(value.to_string())
    }
}

impl From<serde_json::Error> for ModuleError {
    fn from(value: serde_json::Error) -> Self {
        ModuleError::JsonParsingError(value)
    }
}

impl From<toml::de::Error> for ModuleError {
    fn from(value: toml::de::Error) -> Self {
        ModuleError::TomlParsingErrorDe(value)
    }
}

impl From<toml::ser::Error> for ModuleError {
    fn from(value: toml::ser::Error) -> Self {
        ModuleError::TomlParsingErrorSe(value)
    }
}

impl From<std::io::Error> for ModuleError {
    fn from(value: std::io::Error) -> Self {
        ModuleError::ModuleLoadingError(value.to_string())
    }
}

impl From<reqwest::Error> for ModuleError {
    fn from(value: reqwest::Error) -> Self {
        ModuleError::ModuleExecutionError(value.to_string())
    }
}

impl ToString for ModuleError {
    fn to_string(&self) -> String {
        match self {
            ModuleError::ModuleLoadingError(res) => return res.to_string(),
            ModuleError::ModuleParsingError(res) => return res.to_string(),
            ModuleError::ModuleExecutionError(res) => return res.to_string(),
            ModuleError::ModuleDataParsingError(res) => return res.to_string(),
            ModuleError::ModuleSettingsNotFulfilled(res) => return res.to_string(),
            ModuleError::JsonParsingError(res) => return res.to_string(),
            ModuleError::TomlParsingErrorDe(res) => return res.to_string(),
            ModuleError::TomlParsingErrorSe(res) => return res.to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum Mode {
    File {
        dir_path: Option<String>,
        file_name: String,
    },
    Command {
        dir_path: Option<String>,
        command: String,
        args: Option<Vec<String>>,
    },
    Api {
        url: String,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextSettings {
    check_ascii: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
struct ValCheck {
    field: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ObjSettings {
    check_value: Option<Vec<ValCheck>>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum InputType {
    Text { settings: Option<TextSettings> },
    Json { settings: Option<ObjSettings> },
    Toml { settings: Option<ObjSettings> },
}

#[derive(Deserialize, Debug, Clone)]
pub struct Module {
    name: String,
    mode: Mode,
    input_type: InputType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Modules {
    pub modules: Vec<Module>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ModuleResult {
    name: String,
    data: String,
}

pub fn load_modules() -> Result<Modules, ModuleError> {
    let file = fs::read_to_string("/var/msr_server/modules.json")?;
    return Ok(serde_json::from_str(&file)?);
}

impl Module {
    pub fn parse_input(self) -> Result<String, ModuleError> {
        let data = fetch_data(self.mode)?;

        fn check_settings(
            structure: &Value,
            settings: Option<ObjSettings>,
        ) -> Result<(), ModuleError> {
            if let Some(settings) = settings {
                if let Some(values) = settings.check_value {
                    for check in values {
                        if structure.get(check.field).is_none() {
                            return Err(ModuleError::ModuleSettingsNotFulfilled(
                                "checked field not existing".to_string(),
                            ));
                        }
                    }
                }
            }

            Ok(())
        }

        match self.input_type {
            InputType::Text { settings } => {
                if let Some(settings) = settings {
                    if settings.check_ascii == Some(true) {
                        if !data.is_ascii() {
                            return Err(ModuleError::ModuleSettingsNotFulfilled(
                                "ascii check failed".to_string(),
                            ));
                        }
                    }
                }

                return Ok(serde_json::to_string(&ModuleResult {
                    name: self.name,
                    data,
                })?);
            }
            InputType::Json { settings } => {
                let json: Value = serde_json::from_str(&data)?;

                check_settings(&json, settings)?;

                return Ok(serde_json::to_string(&ModuleResult {
                    name: self.name,
                    data: serde_json::to_string(&json)?,
                })?);
            }
            InputType::Toml { settings } => {
                let toml: Value = toml::from_str(&data)?;

                check_settings(&toml, settings)?;

                return Ok(toml::to_string(&ModuleResult {
                    name: self.name,
                    data: toml::to_string(&toml)?,
                })?);
            }
        }
    }
}

pub fn fetch_data(mode: Mode) -> Result<String, ModuleError> {
    match mode {
        Mode::File {
            dir_path,
            file_name,
        } => {
            let path = dir_path.clone().unwrap_or("".to_string()) + file_name.as_str();
            let data = fs::read_to_string(path)?;
            Ok(data)
        }
        Mode::Command {
            dir_path,
            command,
            args,
        } => {
            let path = dir_path.clone().unwrap_or("".to_string()) + command.as_str();
            let mut process = std::process::Command::new(path);
            process.args(args.unwrap_or(vec![]));
            match process.output() {
                Ok(res) => Ok(String::from_utf8(res.stdout)?),
                Err(err) => Err(ModuleError::ModuleExecutionError(err.to_string())),
            }
        }
        Mode::Api { url } => {
            let data = reqwest::blocking::get(url)?;
            let res = data.text()?;
            Ok(res)
        }
    }
}
