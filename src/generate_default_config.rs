use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

const DEFAULT_CONFIG: &[u8] = include_bytes!("../default_config.toml");

pub fn generate_default_config(path: &PathBuf) -> io::Result<()>{
    fs::create_dir_all(&path.parent().unwrap()).expect("Failed to create the config directory");
    let mut file = File::create(path)?;
    file.write_all(DEFAULT_CONFIG)?;

    Ok(())
}


