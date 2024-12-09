use advent_of_code_2024::get_input;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

fn rotate(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn move_pos(pos: (usize, usize), d: Direction, grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let (x, y) = pos;
    match d {
        Direction::Up => match x {
            0 => None,
            _ => Some((x - 1, y)),
        },
        Direction::Right => match y {
            _ if y == grid[x].len() - 1 => None,
            _ => Some((x, y + 1)),
        },
        Direction::Down => match x {
            _ if x == grid.len() - 1 => None,
            _ => Some((x + 1, y)),
        }
        Direction::Left => match y {
            0 => None,
            _ => Some((x, y - 1)),
        }
    }
}

fn solve_part1(input: String) -> i32 {
    let mut input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (mut x, mut y) = (0, 0);
    for (i, s) in input.iter().enumerate() {
        for (j, c) in s.iter().enumerate() {
            if *c == '^' {
                (x, y) = (i, j)
            }
        }
    }
    let mut dir = Direction::Up;
    let mut res = 0;

    'outer: loop {
        if input[x][y] != 'X' {
            res += 1;
        }
        input[x][y] = 'X';
        loop {
            let new_pos = move_pos((x, y), dir, &input);
            if let Some((new_x, new_y)) = new_pos {
                if input[new_x][new_y] == '.' || input[new_x][new_y] == 'X' {
                    (x, y) = (new_x, new_y);
                    break;
                }
                dir = rotate(dir);
            }
            else {
                break 'outer;
            }
        }
    }
    res
}

fn solve_part2(input: String) -> i32 {
    unimplemented!()
}

fn main() {
    println!("Part 1 answer: {}", solve_part1(get_input(6)));
    println!("Part 2 answer: {}", solve_part2(get_input(6)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::get_example_input;

    #[test]
    fn day6_part1() {
        assert_eq!(solve_part1(get_example_input(6)), 41);
    }

    #[test]
    fn day6_part2() {
        // assert_eq!(solve_part2(get_example_input(6)), 42);
    }
}
