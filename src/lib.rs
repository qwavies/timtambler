use std::fs;
use toml;
use chrono::{ Local, NaiveDateTime };
use serde::Deserialize;

pub enum TimeState {
    Past,
    Future,
}

#[derive(Deserialize)]
pub struct Format {
    pub in_class_format: String,
    pub next_class_format: String,
    pub assignment_format: String,
    pub assignment_overdue_format: String,
    pub assignment_time_format: String,
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
            let start_time_unix = class.get_start_time();
            let end_time_unix = class.get_end_time();

            let format_string = if end_time_unix > start_time_unix {
                &self.format.in_class_format
            } else {
                &self.format.next_class_format
            }.replace("{name}", &class.name)
            .replace("{day}", &class.day)
            .replace("{start_time}", &class.start_time)
            .replace("{end_time}", &class.end_time)
            .replace("{location}", &class.location)
            .replace(
                "{time}",
                if end_time_unix > start_time_unix {
                    &end_time_unix
                } else {
                    &start_time_unix
                }
                );

            println!("{}", format_string);
        }
    }

    pub fn list_assignments(&self) {
        for assignment in &self.assignment {
            let (time, timestate) = assignment.get_time(&self.format.assignment_time_format);

            let format_string = match timestate {
                TimeState::Past => &self.format.assignment_overdue_format,
                TimeState::Future => &self.format.assignment_format,
            }.replace("{name}", &assignment.name)
            .replace("{points}", &assignment.points)
            .replace("{due_date}", &assignment.due_date)
            .replace("{time}", &time);

            println!("{}", format_string);
        }
    }
}

impl Assignment {
    pub fn get_time(&self, format: &str) -> (String, TimeState) {
        let due_date_unix = NaiveDateTime::parse_from_str(&self.due_date, format)
            .expect("Invalid date format provided")
            .and_utc().timestamp();
        let current_time_unix = Local::now().timestamp();

        // positive times indicate future, negative times indicate past
        let time_difference = due_date_unix - current_time_unix;

        let current_time_state = if time_difference >= 0 {
            TimeState::Future
        } else {
            TimeState::Past
        };

        let formatted_time = format_time(time_difference.abs());

        (formatted_time, current_time_state)
    }
}

impl Class {
    pub fn get_start_time(&self) -> String {
        let start_time_unix = next_occurance_of_day_time_unix(&self.day, &self.start_time);
        let current_time_unix = Local::now().timestamp();

        let time_difference = start_time_unix - current_time_unix;
        format_time(time_difference)
    }
    pub fn get_end_time(&self) -> String {
        let end_time_unix = next_occurance_of_day_time_unix(&self.day, &self.end_time);
        let current_time_unix = Local::now().timestamp();

        let time_difference = end_time_unix - current_time_unix;
        format_time(time_difference)
    }
}

fn next_occurance_of_day_time_unix(weekday: &String, time: &String) -> i64 {
    let current_time_unix = Local::now().timestamp();
    let target_weekday = match weekday.to_lowercase().as_str() {
        "monday" => 0,
        "tuesday" => 1,
        "wednesday" => 2,
        "thursday" => 3,
        "friday" => 4,
        "saturday" => 5,
        "sunday" => 6,
        _ => panic!("Invalid day of the week. {} is not a day of the week", weekday)
    };
    let current_weekday = ((current_time_unix / 86400) + 4) % 7;
    let day_difference = (target_weekday - current_weekday) % 7;

    let current_seconds = current_time_unix / 86400;
    let target_hour_and_minutes: Vec<&str> = time.split(":").collect();
    let target_hours: i64 = target_hour_and_minutes[0].parse().unwrap();
    let target_minutes: i64 = target_hour_and_minutes[1].parse().unwrap();
    let target_seconds = (target_hours * 3600) + (target_minutes * 60);
    // seconds seconds_difference can be negative
    let seconds_difference = target_seconds - current_seconds;

    let final_time_unix = current_time_unix + (day_difference * 86400) + seconds_difference;
    final_time_unix
}
fn format_time(raw_seconds: i64) -> String {

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

    time_formats.join(", ")
}
