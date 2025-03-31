use std::env;
use std::path::PathBuf;
use dirs::home_dir;

use timtambler::Timetable;
use timtambler::generate_default_config::generate_default_config;

fn main() {
    let config_path = get_config();

    if !config_path.exists() {
        println!("No timtambler config detected");
        println!("Generating a scaffold config...");
        match generate_default_config(&config_path) {
            Ok(_) => {
                println!("Scaffold config successfully created");
            }
            Err(_) => {
                panic!("Failed to generte a default config")
            }
        }
        println!("Change the path of the timtambler by setting the \"TIMTAM_DIR\" environment variable")
    }

    let timetable: Timetable = Timetable::read_toml_file(config_path);

    println!("Classes:");
    for class in timetable.list_classes() {
        println!("{}", class)
    }

    println!("Assignments:");
    for assignment in timetable.list_assignments() {
        println!("{}", assignment)
    }
}

fn get_config() -> PathBuf {
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
