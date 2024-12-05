use advent_of_code_2024::get_input;
use std::collections::HashMap;
use std::iter::zip;

fn parse_input(input: String) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut ids1 = Vec::new();
    let mut ids2 = Vec::new();
    for line in lines {
        let mut line_split = line.split(' ').filter(|&s| !s.is_empty());
        ids1.push(line_split.next().unwrap().parse::<i32>().unwrap());
        ids2.push(line_split.next().unwrap().parse::<i32>().unwrap());
    }
    (ids1, ids2)
}

fn solve_part1(input: String) -> i32 {
    let (mut ids1, mut ids2) = parse_input(input);
    ids1.sort();
    ids2.sort();
    let mut res = 0;
    for (a, b) in zip(ids1, ids2) {
        res += (a - b).abs();
    }
    res
}

fn solve_part2(input: String) -> i32 {
    let (ids1, ids2) = parse_input(input);

    let mut counts2 = HashMap::new();
    for id in ids2 {
        let count = *counts2.get(&id).unwrap_or(&0) + 1;
        counts2.insert(id, count);
    }
    let mut res = 0;
    for id in ids1 {
        res += id * counts2.get(&id).unwrap_or(&0);
    }
    res
}

fn main() {
    println!("Part 1 answer: {}", solve_part1(get_input(1)));
    println!("Part 2 answer: {}", solve_part2(get_input(1)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::get_example_input;

    #[test]
    fn day1_part1() {
        assert_eq!(solve_part1(get_example_input(1)), 11);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(solve_part2(get_example_input(1)), 31);
    }
}
