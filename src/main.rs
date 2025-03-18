use timtambler::Timetable;

fn main() {
    let timetable = Timetable::read_toml_file("config.toml");
    println!("{}", timetable.format.assignment_format);
    println!("{}", timetable.format.class_format);
}
