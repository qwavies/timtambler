use timtambler::format_time;
#[test]
fn testing_tests() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn format_time_test() {
    let one_minute_formatted = format_time(60);
    assert_eq!(one_minute_formatted, "1 minute");

    let one_minute_one_second_formatted = format_time(61);
    assert_eq!(one_minute_one_second_formatted, "1 minute & 1 second");

    let one_minute_fifty_nine_formatted = format_time(119);
    assert_eq!(one_minute_fifty_nine_formatted, "1 minute & 59 seconds");

    let two_minutes_formatted = format_time(120);
    assert_eq!(two_minutes_formatted, "2 minutes");

    let fifty_nine_minutes_fifty_nine_seconds_formatted = format_time(3599);
    assert_eq!(fifty_nine_minutes_fifty_nine_seconds_formatted, "59 minutes & 59 seconds")
}
