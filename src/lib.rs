use chrono::{Datelike, Local, NaiveDateTime, Weekday};
use serde::Deserialize;
use std::{fs, path::PathBuf};
use toml;

pub mod config;

pub enum ClassState {
    InClass,
    OutOfClass,
}

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
    pub fn read_toml_file(file: PathBuf) -> Timetable {
        let content = fs::read_to_string(file).expect("Couldn't read toml file");
        let timetable: Timetable = toml::de::from_str(&content).expect("Couldn't parse toml file");
        timetable
    }

    pub fn list_classes(&self) -> Vec<String> {
        let mut output_classes: Vec<(String, i64)> = Vec::new();
        for class in &self.class {
            let start_time_unix: i64 =
                next_occurance_of_day_time_unix(&class.day, &class.start_time);
            let end_time_unix: i64 = next_occurance_of_day_time_unix(&class.day, &class.end_time);

            let (format_string, in_class, relevant_time_unix) = if end_time_unix <= start_time_unix
            {
                (
                    &self.format.in_class_format,
                    ClassState::InClass,
                    end_time_unix,
                )
            } else {
                (
                    &self.format.next_class_format,
                    ClassState::OutOfClass,
                    start_time_unix,
                )
            };

            let format_string = class.format_class_string(format_string.to_string(), in_class);
            output_classes.push((format_string, relevant_time_unix));
        }
        output_classes.sort_by_key(|time| time.1);
        let output_classes: Vec<String> = output_classes
            .into_iter()
            .map(|(formatted_string, _)| formatted_string)
            .collect();

        output_classes
    }

    pub fn list_assignments(&self) -> Vec<String> {
        let mut output_assignments: Vec<(String, i64)> = Vec::new();
        for assignment in &self.assignment {
            let (time, time_unix, timestate) =
                assignment.get_time(&self.format.assignment_time_format);
            let format_string = assignment.format_assignment_string(time, timestate, &self.format);
            output_assignments.push((format_string, time_unix))
        }
        output_assignments.sort_by_key(|time| time.1);
        let output_assignments: Vec<String> = output_assignments
            .into_iter()
            .map(|(formatted_string, _)| formatted_string)
            .collect();

        output_assignments
    }
}

impl Assignment {
    pub fn get_time(&self, format: &str) -> (String, i64, TimeState) {
        let due_date_unix = NaiveDateTime::parse_from_str(&self.due_date, format)
            .expect("Invalid date format provided")
            .and_utc()
            .timestamp();
        let current_time_unix = Local::now().timestamp();

        // positive times indicate future, negative times indicate past
        let time_difference = due_date_unix - current_time_unix;

        let current_time_state = if time_difference >= 0 {
            TimeState::Future
        } else {
            TimeState::Past
        };

        let formatted_time = format_time(time_difference.abs());

        (formatted_time, due_date_unix, current_time_state)
    }

    pub fn format_assignment_string(
        &self,
        time: String,
        timestate: TimeState,
        format: &Format,
    ) -> String {
        let format_string = match timestate {
            TimeState::Past => &format.assignment_overdue_format,
            TimeState::Future => &format.assignment_format,
        }
        .replace("{name}", &self.name)
        .replace("{points}", &self.points)
        .replace("{due_date}", &self.due_date)
        .replace("{time}", &time);
        format_string
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

    pub fn format_class_string(&self, format_string: String, in_class: ClassState) -> String {
        let time_format = match in_class {
            ClassState::InClass => &self.get_end_time(),
            ClassState::OutOfClass => &self.get_start_time(),
        };
        format_string
            .replace("{name}", &self.name)
            .replace("{day}", &self.day)
            .replace("{start_time}", &self.start_time)
            .replace("{end_time}", &self.end_time)
            .replace("{location}", &self.location)
            .replace("{time}", time_format)
    }
}

fn next_occurance_of_day_time_unix(weekday: &String, time: &String) -> i64 {
    let current_time = Local::now();
    let current_time_unix = current_time.timestamp();
    let current_hour_and_minutes: String = current_time.time().to_string();
    let current_hour_and_minutes: Vec<&str> = current_hour_and_minutes
        .split(|delimiter| delimiter == ':' || delimiter == '.')
        .collect();
    let current_hours: i64 = current_hour_and_minutes[0].parse().unwrap();
    let current_minutes: i64 = current_hour_and_minutes[1].parse().unwrap();
    let current_seconds: i64 = current_hour_and_minutes[2].parse().unwrap();
    let current_total_seconds =
        (current_hours * 60 * 60) + (current_minutes * 60) + current_seconds;
    let current_weekday = match current_time.weekday() {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };

    let target_hour_and_minutes: Vec<&str> = time.split(":").collect();
    let target_hours: i64 = target_hour_and_minutes[0].parse().unwrap();
    let target_minutes: i64 = target_hour_and_minutes[1].parse().unwrap();
    let target_total_seconds = (target_hours * 60 * 60) + (target_minutes * 60);
    let target_weekday = match weekday.to_lowercase().as_str() {
        "monday" => 0,
        "tuesday" => 1,
        "wednesday" => 2,
        "thursday" => 3,
        "friday" => 4,
        "saturday" => 5,
        "sunday" => 6,
        "mon" => 0,
        "tue" => 1,
        "wed" => 2,
        "thu" => 3,
        "fri" => 4,
        "sat" => 5,
        "sun" => 6,
        _ => panic!(
            "Invalid day of the week. {} is not a day of the week",
            weekday
        ),
    };

    let mut day_difference = (((target_weekday - current_weekday) % 7) + 7) % 7;
    // seconds_difference can be negative
    let seconds_difference = target_total_seconds - current_total_seconds;

    // edge case where its the same day but past the current time
    if (day_difference == 0) && (seconds_difference <= 0) {
        day_difference = 7;
    }

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
        if weeks == 1 {
            time_formats.push(format!("{} week", weeks));
        } else {
            time_formats.push(format!("{} weeks", weeks));
        }
    }
    if days > 0 {
        if days == 1 {
            time_formats.push(format!("{} day", days));
        } else {
            time_formats.push(format!("{} days", days));
        }
    }
    if hours > 0 && time_formats.len() < 2 {
        if hours == 1 {
            time_formats.push(format!("{} hour", hours));
        } else {
            time_formats.push(format!("{} hours", hours));
        }
    }
    if minutes > 0 && time_formats.len() < 2 {
        if minutes == 1 {
            time_formats.push(format!("{} minute", minutes));
        } else {
            time_formats.push(format!("{} minutes", minutes));
        }
    }
    if (seconds > 0 || time_formats.is_empty()) && time_formats.len() < 2 {
        if seconds == 0 {
            time_formats.push(format!("{} second", seconds));
        } else {
            time_formats.push(format!("{} seconds", seconds));
        }
    }

    time_formats.join(" & ")
}
