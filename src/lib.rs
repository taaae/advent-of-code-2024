use std::fs;

pub fn get_input(day: i32) -> String {
    let filepath = format!("input/day{}.txt", day);
    let content = fs::read_to_string(filepath).expect("Failed to read an input");
    content
}

pub fn get_example_input(day: i32) -> String {
    let filepath = format!("input/day{}example.txt", day);
    let content = fs::read_to_string(filepath).expect("Failed to read an example");
    content
}
