use timtambler::Timetable;

fn main() {
    let timetable: Timetable = Timetable::read_toml_file("config.toml");

    timetable.list_classes();
    timetable.list_assignments();
}
