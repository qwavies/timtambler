use std::fs;
use toml;
use chrono::{ DateTime, Local };
use serde::Deserialize;

pub enum TimeState {
    Past,
    Future,
}

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
            // addd functionality to find how long until assignment due
            let format_string = self.format.assignment_format
                .replace("{name}", &assignment.name)
                .replace("{points}", &assignment.points)
                .replace("{due_date}", &assignment.due_date);

            println!("{}", format_string);
        }
    }

    pub fn calculate_time_until(time: &str) -> (String, TimeState) {
        // returns the number of seconds between the given date and the current time
        // negative numbers represent in the past
        //converts all times to Unix Epoch time and compares them
        let time_unix = DateTime::parse_from_rfc3339(time)
            .expect("Invalid rfc3339 time stamp")
            .timestamp();
        let current_time_unix = Local::now()
            .timestamp();
        let time_difference = time_unix - current_time_unix;

        format_time(time_difference)
    }
}

fn format_time(raw_seconds: i64) -> (String, TimeState) {
    let current_time_state = if raw_seconds >= 0 {
        TimeState::Future
    } else {
        TimeState::Past
    };

    let total_seconds = raw_seconds.abs();
    let weeks = total_seconds / (7 * 24 * 60 * 60);
    let days = (total_seconds % (7 * 24 * 60 * 60)) / (24 * 60 * 60);
    let hours = (total_seconds % (24 * 60 * 60)) / (60 * 60);
    let minutes = (total_seconds % (60 * 60)) / 60;
    let seconds = total_seconds % 60; 

    let mut time_formats = Vec::new();
    if weeks > 0 {
        time_formats.push(format!("{} weeks", weeks));
    }
    if days > 0 {
        time_formats.push(format!("{} days", days));
    }
    if hours > 0 {
        time_formats.push(format!("{} hours", hours));
    }
    if minutes > 0 {
        time_formats.push(format!("{} minutes", minutes));
    }
    if seconds > 0 || time_formats.is_empty() {
        time_formats.push(format!("{} seconds", seconds));
    }

    (time_formats.join(", "), current_time_state)
}
