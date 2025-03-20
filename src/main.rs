use timtambler::Timetable;

fn main() {
    let timetable: Timetable = Timetable::read_toml_file("config.toml");

    for assignment in timetable.assignment {
        println!("{}",Timetable::calculate_time_until(&assignment.due_date).0);
    }
}
