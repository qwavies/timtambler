use timtambler::Timetable;

fn main() {
    let timetable: Timetable = Timetable::read_toml_file("config.toml");

    println!("Classes:");
    for class in timetable.list_classes() {
        println!("{}", class)
    }

    println!("Assignments:");
    for assignment in timetable.list_assignments() {
        println!("{}", assignment)
    }
}
