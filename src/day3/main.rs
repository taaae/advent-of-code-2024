use advent_of_code_2024::get_input;
use regex::Regex;

fn solve_part1(input: String) -> i32 {
    let mul_re = Regex::new(r"mul\(-?(0|[1-9][0-9]*),(0|-?[1-9][0-9]*)\)").unwrap();
    let mut res = 0;
    for r in mul_re.captures_iter(&input) {
        res += r[1].parse::<i32>().unwrap() * r[2].parse::<i32>().unwrap();
    }
    res
}

fn solve_part2(input: String) -> i32 {
    let mul_re = Regex::new(r"mul\(-?(0|[1-9][0-9]*),(0|-?[1-9][0-9]*)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let op_re = Regex::new(&format!(r"({mul_re})|({do_re})|({dont_re})")).unwrap();
    let mut res = 0;
    let mut doo = true;
    for r in op_re.find_iter(&input) {
        let s = r.as_str();
        if do_re.is_match(s) {
            doo = true;
        } else if dont_re.is_match(s) {
            doo = false;
        } else if doo && mul_re.is_match(s) {
            let c = mul_re.captures(s).unwrap();
            res += c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap();
        }
    }
    res
}

fn main() {
    println!("Part 1 answer: {}", solve_part1(get_input(3)));
    println!("Part 2 answer: {}", solve_part2(get_input(3)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::get_example_input;

    #[test]
    fn day3_part1() {
        assert_eq!(solve_part1(get_example_input(3)), 161);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(solve_part2(get_example_input(3)), 48);
    }
}
