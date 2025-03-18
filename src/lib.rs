use std::fs;
use toml;

pub struct Timetable {
    pub classes: Vec<Class>,
    pub assignments: Vec<Assignment>,
    pub format: Format,

}
pub struct Format {
    pub class_format: String,
    pub assignment_format: String,
}

pub struct Class {
    pub name: String,
    pub day: String,
    pub start_time: String,
    pub end_time: String,
    pub location: String,
}

pub struct Assignment {
    pub name: String,
    pub points: String,
    pub due_date: String,
}
