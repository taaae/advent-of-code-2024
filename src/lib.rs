use std::fs;

pub fn get_input(day: i32) -> String {
    let filepath = format!("input/day{}.txt", day);
    fs::read_to_string(filepath).expect("Failed to read an input")
}

pub fn get_example_input(day: i32) -> String {
    let filepath = format!("input/day{}example.txt", day);
    fs::read_to_string(filepath).expect("Failed to read an example")
}

pub fn parse_2d_arr(input: String) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|s| s.split_whitespace().map(|s| s.to_owned()).collect())
        .collect()
}

pub fn input_to_grid(input: String) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_2d_arr() {
        assert_eq!(
            parse_2d_arr("1 2 10\na   b c".to_owned()),
            vec![vec!["1", "2", "10"], vec!["a", "b", "c"]]
        )
    }
}
