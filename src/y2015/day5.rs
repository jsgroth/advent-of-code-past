//! Day 5: Doesn't He Have Intern-Elves For This?
//! https://adventofcode.com/2015/day/5

use std::error::Error;

fn solve_part_1(input: &str) -> usize {
    input.lines().filter(|line| is_nice(line)).count()
}

fn solve_part_2(input: &str) -> usize {
    input.lines().filter(|line| is_nice_2(line)).count()
}

fn is_nice(line: &str) -> bool {
    if line.chars().filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c)).count() < 3 {
        return false;
    }

    let chars: Vec<_> = line.chars().collect();
    if !chars.windows(2).any(|window| window[0] == window[1]) {
        return false;
    }

    !chars.windows(2).any(|window| {
        match window {
            ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'] => true,
            _ => false
        }
    })
}

fn is_nice_2(line: &str) -> bool {
    let chars: Vec<_> = line.chars().collect();
    if !chars.windows(3).any(|window| window[0] == window[2]) {
        return false;
    }

    for (i, window) in chars.windows(2).enumerate() {
        for other_window in chars[i + 2..].windows(2) {
            if window == other_window {
                return true;
            }
        }
    }

    false
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input);
    let solution2 = solve_part_2(input);

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(1, solve_part_1("ugknbfddgicrmopn"));
        assert_eq!(1, solve_part_1("aaa"));
        assert_eq!(0, solve_part_1("jchzalrnumimnmhp"));
        assert_eq!(0, solve_part_1("haegwjzuvuyypxyu"));
        assert_eq!(0, solve_part_1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(1, solve_part_2("qjhvhtzxzqqjkmpb"));
        assert_eq!(1, solve_part_2("xxyxx"));
        assert_eq!(0, solve_part_2("uurcxstgmygtbstg"));
        assert_eq!(0, solve_part_2("ieodomkazucvgmuy"));
    }
}