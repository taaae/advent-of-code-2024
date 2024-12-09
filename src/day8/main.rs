use advent_of_code_2024::{get_input, input_to_grid};
use std::collections::HashMap;

fn solve_part1(input: String) -> i32 {
    let input = input_to_grid(input);
    let antenas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    for (x, v) in input.iter().enumerate() {
        for (y, &c) in v.iter().enumerate() {
            if c != b'.' {
                antenas. 
            }
        }
    }
    unimplemented!()
}

fn solve_part2(input: String) -> i32 {
    unimplemented!()
}

fn main() {
    println!("Part 1 answer: {}", solve_part1(get_input(8)));
    println!("Part 2 answer: {}", solve_part2(get_input(8)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::get_example_input;

    #[test]
    fn day8_part1() {
        assert_eq!(solve_part1(get_example_input(8)), 14);
    }

    #[test]
    fn day8_part2() {
        // assert_eq!(solve_part2(get_example_input(8)), 42);
    }
}
