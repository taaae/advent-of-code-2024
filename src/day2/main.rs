use advent_of_code_2024::{get_input, parse_2d_arr};
use std::ops::Add;

/// Differences between 2 elements
/// D means that one of the elements was deleted
#[derive(PartialEq, Debug, Clone, Copy)]
enum Diff {
    I(i32),
    D,
}

impl Add for Diff {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Diff::I(a), Diff::I(b)) => Diff::I(a + b),
            (Diff::D, _) | (_, Diff::D) => Diff::D,
        }
    }
}

trait IntoDiff {
    fn into_diff_arr(self) -> Vec<Diff>;
}

impl IntoDiff for &[i32] {
    fn into_diff_arr(self) -> Vec<Diff> {
        let mut arr: Vec<Diff> = self
            .windows(2)
            .map(|pair| Diff::I(pair[1] - pair[0]))
            .collect();
        arr.insert(0, Diff::D);
        arr.push(Diff::D);
        arr
    }
}

fn is_good_pos(d: &Diff) -> bool {
    match d {
        Diff::D => true,
        Diff::I(1..=3) => true,
        Diff::I(_) => false,
    }
}

fn is_good_neg(d: &Diff) -> bool {
    match d {
        Diff::D => true,
        Diff::I(-3..=-1) => true,
        Diff::I(_) => false,
    }
}

fn bad_indexes<P>(diffs: &[Diff], predicate: &P) -> Vec<usize>
where
    P: Fn(&Diff) -> bool,
{
    diffs
        .iter()
        .enumerate()
        .filter(|(_, d)| !predicate(d))
        .map(|(i, _)| i)
        .collect()
}

fn solve_part1(input: String) -> usize {
    let reports: Vec<Vec<i32>> = parse_2d_arr(input)
        .iter()
        .map(|arr| arr.iter().map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();

    reports
        .iter()
        .map(|report| report.into_diff_arr())
        .filter(|report| {
            bad_indexes(report, &is_good_pos).is_empty()
                || bad_indexes(report, &is_good_neg).is_empty()
        })
        .count()
}

fn solve_part2(input: String) -> usize {
    let reports: Vec<Vec<i32>> = parse_2d_arr(input)
        .iter()
        .map(|arr| arr.iter().map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();

    fn tolerated<P>(diffs: &[Diff], predicate: P) -> bool
    where
        P: Fn(&Diff) -> bool,
    {
        let ids = bad_indexes(diffs, &predicate);
        ids.is_empty()
            || (ids.len() == 1
                && (predicate(&(diffs[ids[0]] + diffs[ids[0] + 1]))
                    || predicate(&(diffs[ids[0]] + diffs[ids[0] - 1]))))
            || (ids.len() == 2
                && ids[1] == ids[0] + 1
                && predicate(&(diffs[ids[0]] + diffs[ids[1]])))
    }

    reports
        .iter()
        .map(|report| report.into_diff_arr())
        .filter(|report| tolerated(report, is_good_pos) || tolerated(report, is_good_neg))
        .count()
}

fn main() {
    println!("Part 1 answer: {}", solve_part1(get_input(2)));
    println!("Part 2 answer: {}", solve_part2(get_input(2)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::get_example_input;

    #[test]
    fn new_diff_arr_test() {
        let arr = vec![1, 3, 2, 4];
        let expected = vec![Diff::D, Diff::I(2), Diff::I(-1), Diff::I(2), Diff::D];
        assert_eq!(arr.into_diff_arr(), expected);
    }

    #[test]
    fn bad_indexes_test() {
        let diffs = vec![Diff::D, Diff::I(4), Diff::I(-1), Diff::I(2), Diff::D];
        let expected = vec![1, 2];
        assert_eq!(bad_indexes(&diffs, &is_good_pos), expected);
    }

    #[test]
    fn day2_part1() {
        assert_eq!(solve_part1(get_example_input(2)), 2);
    }

    #[test]
    fn day2_part2() {
        assert_eq!(solve_part2(get_example_input(2)), 4);
    }
}
