use advent_of_code_2024::get_input;
use regex::Regex;
use std::str;

fn calc_xmas_occurrences_at_dir(input: &Vec<&[u8]>, position: (usize, usize), direction: (i32, i32)) -> usize {
    let (mut x, mut y) = position;
    let (dx, dy) = direction;
    let s = "XMAS".as_bytes();
    for i in 0..4 {
        if input.get(x) == None || input[x].get(y) == None || input[x][y] != s[i] {
            return 0;
        }
        if i == 3 {
            break;
        }
        if (x as isize + dx as isize) < 0 || (y as isize + dy as isize) < 0 {
            return 0;
        }
        x = (x as isize + dx as isize) as usize;
        y = (y as isize + dy as isize) as usize;
    }
    1
}

fn calc_x_mas_occurrences_at(input: &Vec<&[u8]>, position: (usize, usize), x_mas_re: &Regex) -> usize {
    let (x, y) = position;
    if x + 3 > input.len() || y + 3 > input[0].len() {
        return 0;
    }
    let mut cross = vec![];
    for i in 0..3 {
        for j in 0..3 {
            cross.push(input[x + i][y + j]);
        }
    }
    match x_mas_re.is_match(str::from_utf8(&cross).unwrap()) {
        true => 1,
        false => 0,
    }
}

fn solve_part1(input: String) -> usize {
    let input: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut res = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            for direction in [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
                res += calc_xmas_occurrences_at_dir(&input, (x, y), direction);
            }
        }
    }
    res
}

fn solve_part2(input: String) -> usize {
    let x_mas_re = Regex::new(r"^(M.S.A.M.S)|(S.S.A.M.M)|(S.M.A.S.M)|(M.M.A.S.S)$").unwrap();
    let input: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut res = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            res += calc_x_mas_occurrences_at(&input, (x, y), &x_mas_re);
        }
    }
    res
}

fn main() {
    println!("Part 1 answer: {}", solve_part1(get_input(4)));
    println!("Part 2 answer: {}", solve_part2(get_input(4)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::get_example_input;

    #[test]
    fn day4_part1() {
        assert_eq!(solve_part1(get_example_input(4)), 18);
    }

    #[test]
    fn day4_part2() {
        assert_eq!(solve_part2(get_example_input(4)), 9);
    }

    #[test]
    fn calc_xmas_occurrences_at_dir_test() {
        let input = vec!["XMAS".as_bytes(), "MMAA".as_bytes()];
        assert_eq!(calc_xmas_occurrences_at_dir(&input, (0, 0), (0, 1)), 1);
        assert_eq!(calc_xmas_occurrences_at_dir(&input, (0, 0), (0, -1)), 0);
        assert_eq!(calc_xmas_occurrences_at_dir(&input, (0, 0), (1, 1)), 0);
        assert_eq!(calc_xmas_occurrences_at_dir(&input, (1, 0), (0, 1)), 0);
    }

    #[test]
    fn calc_x_mas_occurrences_at_test() {
        let x_mas_re = Regex::new(r"^(M.S.A.M.S)|(S.S.A.M.M)|(S.M.A.S.M)|(M.M.A.S.S)$").unwrap();
        let input = vec!["MXS".as_bytes(), "FAF".as_bytes(), "MXS".as_bytes()];
        assert_eq!(calc_x_mas_occurrences_at(&input, (0, 0), &x_mas_re), 1);
        assert_eq!(calc_x_mas_occurrences_at(&input, (1, 1), &x_mas_re), 0);
    }
}
