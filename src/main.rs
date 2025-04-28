use timtambler::Timetable;
use timtambler::config::{get_config, generate_default_config};

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
        println!("ChangE the path of the timtambler by setting the \"TIMTAM_DIR\" environment variable")
    }

    let timetable: Timetable = Timetable::read_toml_file(config_path);

    println!("Classes:");
    for class in timetable.list_classes().iter().take(3) {
        println!("{}", class)
    }

    println!("Assignments:");
    for assignment in timetable.list_assignments().iter().take(4) {
        println!("{}", assignment)
    }
}

