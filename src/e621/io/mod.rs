extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, read_to_string, write, read_dir};
use std::io;
use std::path::Path;
use std::process::exit;

use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::io::Write;

pub mod tag;

/// Name of the configuration file.
pub static CONFIG_NAME: &'static str = "config.json";

/// Config that is used to do general setup.
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    /// Whether or not to create a directory for every tag used to search for images
    #[serde(rename = "createDirectories")]
    pub create_directories: bool,
    /// The location of the download directory
    #[serde(rename = "downloadDirectory")]
    pub download_directory: String,
    /// Holds all dates for every tag used.
    #[serde(rename = "lastRun")]
    pub last_run: HashMap<String, String>,
    /// Which part should be used as the name, that of which are: "id", or "md5"
    #[serde(rename = "partUsedAsName")]
    pub part_used_as_name: String,
}

/// Checks config and ensure it isn't missing.
///
/// # Example
///
/// ```
/// let config_exists = check_config();
/// ```
pub fn config_exists() -> bool {
    if !Path::new(CONFIG_NAME).exists() {
        println!("config.json: does not exist!");
        return false;
    }

    true
}

/// Creates config file.
///
/// # Example
///
/// ```
/// let config_exists = check_config();
/// if !config_exists {
///     create_config();
/// }
/// ```
pub fn create_config() -> Result<(), Box<Error>> {
    let json = to_string_pretty(&Config {
        create_directories: true,
        download_directory: String::from("downloads/"),
        last_run: HashMap::new(),
        part_used_as_name: String::from("md5"),
    })?;

    let mut config = File::create(Path::new(CONFIG_NAME))?;
    config.write(&json.as_bytes())?;

    Ok(())
}

/// Checks if config exist and, if not, creates config template.
///
/// ```
/// check_config();
/// let config = get_config();
/// ```
pub fn check_config() -> Result<(), Box<Error>> {
    if !config_exists() {
        println!("Creating config...");
        return create_config();
    }

    Ok(())
}

/// Loads and returns `config` for quick management and settings.
///
/// ```rust
/// # check_config();
/// let config = get_config();
/// ```
pub fn get_config() -> Result<Config, Box<Error>> {
    let config = serde_json::from_str::<Config>(&read_to_string(Path::new(CONFIG_NAME)).unwrap())?;
    Ok(config)
}

/// Saves new configuration for future run.
pub fn save_config(config: &Config) -> Result<(), Box<Error>> {
    let json = serde_json::to_string_pretty(config)?;
    write(Path::new(CONFIG_NAME), json)?;

    Ok(())
}

/// Exits the program after message explaining the error and prompting the user to press `ENTER`.
fn emergency_exit(error: &str) {
    println!("{}", error);
    println!("Press ENTER to close the application...");
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap_or_default();
    exit(0);
}

pub fn find_dir(target: &String, dir: &Path) -> io::Result<String> {
    for path in read_dir(dir)? {
        let dir_path = path?;
        if dir_path.path().is_dir() {
            let path_str = dir_path.path().to_str().unwrap();
            println!("{:?}", path_str);
            if path_str.contains(target) {
                return Ok(path_str.to_string());
            }
            return find_dir(&target, &dir_path.path());
        }
//        if dir_path.file_name().into_string()?.contains(dir) == dir {
//
//        }
    }

    Ok(String::from("Hello"))
}