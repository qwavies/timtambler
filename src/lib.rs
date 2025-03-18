use std::fs;
use toml;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Format {
    pub class_format: String,
    pub assignment_format: String,
}

#[derive(Deserialize)]
pub struct Class {
    pub name: String,
    pub day: String,
    pub start_time: String,
    pub end_time: String,
    pub location: String,
}

#[derive(Deserialize)]
pub struct Assignment {
    pub name: String,
    pub points: String,
    pub due_date: String,
}

#[derive(Deserialize)]
pub struct Timetable {
    pub class: Vec<Class>,
    pub assignment: Vec<Assignment>,
    pub format: Format,
}

impl Timetable {
    pub fn read_toml_file(file: &str) -> Timetable {
        let content = fs::read_to_string(file).expect("Couldn't read toml file");
        let timetable: Timetable = toml::de::from_str(&content).expect("Couldn't parse toml file");

        timetable
    }

    pub fn list_classes(&self) {
        for class in &self.class {
            // addd functionality to find how long until next class
            let format_string = self.format.class_format
                .replace("{name}", &class.name)
                .replace("{day}", &class.day)
                .replace("{start_time}", &class.start_time)
                .replace("{end_time}", &class.end_time)
                .replace("{location}", &class.location);

            println!("{}", format_string);
        }
    }

    pub fn list_assignments(&self) {
        for assignment in &self.assignment {
            // addd functionality to find how long until next class
            let format_string = self.format.assignment_format
                .replace("{name}", &assignment.name)
                .replace("{points}", &assignment.points)
                .replace("{due_date}", &assignment.due_date);

            println!("{}", format_string);
        }
    }
}
