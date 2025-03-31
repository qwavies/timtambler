use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use dirs::home_dir;
use std::path::PathBuf;

const DEFAULT_CONFIG: &[u8] = include_bytes!("../default_config.toml");

pub fn get_config() -> PathBuf {
    let config_path: PathBuf = match env::var("TIMTAM_DIR") {
        Ok(path) => {
            PathBuf::from(path)
        }
        Err(_) => {
            let mut default_path = home_dir().expect("No home directory found");
            default_path.push(".config");
            default_path.push("timtambler");
            default_path.push("config.toml");
            default_path
        }
    };
    config_path
}

pub fn generate_default_config(path: &PathBuf) -> io::Result<()>{
    fs::create_dir_all(&path.parent().unwrap()).expect("Failed to create the config directory");
    let mut file = File::create(path)?;
    file.write_all(DEFAULT_CONFIG)?;

    Ok(())
}


