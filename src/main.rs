use timtambler::Timetable;

fn main() {
    let timetable: Timetable = Timetable::read_toml_file("config.toml");

    //for assignment in timetable.assignment {
    //    println!("{}",Timetable::time_from_now(
    //            &assignment.due_date,
    //            &timetable.format.assignment_time_format
    //            ).0);
    //}

    timetable.list_assignments();
}
