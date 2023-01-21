//! Day 16: Permutation Promenade
//! https://adventofcode.com/2017/day/16

use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, Clone, Copy)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl DanceMove {
    fn from_str(s: &str) -> Result<Self, SimpleError> {
        match s.chars().next() {
            Some('s') => {
                Ok(Self::Spin(s[1..].parse()?))
            }
            Some('x') => {
                let (a, b) = s[1..].split_once('/').ok_or_else(
                    || SimpleError::new(format!("invalid exchange move: {s}"))
                )?;
                Ok(Self::Exchange(a.parse()?, b.parse()?))
            }
            Some('p') => {
                let (a, b) = s[1..].split_once('/').ok_or_else(
                    || SimpleError::new(format!("invalid exchange move: {s}"))
                )?;
                Ok(Self::Partner(a.parse()?, b.parse()?))
            }
            _ => Err(SimpleError::new(format!("invalid dance move string: {s}")))
        }
    }
}

fn solve_part_1(input: &str, num_programs: u8) -> Result<String, SimpleError> {
    let dance_moves = parse_input(input)?;

    let programs_end = (b'a' + num_programs) as char;
    let programs: Vec<_> = ('a'..programs_end).collect();

    let after_dance = simulate_dance(programs, &dance_moves);

    Ok(after_dance.into_iter().collect())
}

fn solve_part_2(input: &str) -> Result<String, SimpleError> {
    let dance_moves = parse_input(input)?;

    let mut programs: Vec<_> = ('a'..='p').collect();

    let mut arrangement_to_iteration: HashMap<String, _> = HashMap::new();
    arrangement_to_iteration.insert(programs.iter().copied().collect(), 0);

    for i in 1.. {
        programs = simulate_dance(programs, &dance_moves);

        let arrangement: String = programs.iter().copied().collect();
        if let Some(&prev_iteration) = arrangement_to_iteration.get(&arrangement) {
            let leftover_iterations = (1_000_000_000 - i) % (i - prev_iteration);
            for _ in 0..leftover_iterations {
                programs = simulate_dance(programs, &dance_moves);
            }
            return Ok(programs.into_iter().collect());
        } else {
            arrangement_to_iteration.insert(arrangement, i);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn simulate_dance(mut programs: Vec<char>, dance_moves: &[DanceMove]) -> Vec<char> {
    for &dance_move in dance_moves {
        match dance_move {
            DanceMove::Spin(x) => {
                programs = programs.iter().copied().skip(programs.len() - x)
                    .chain(programs.iter().copied().take(programs.len() - x))
                    .collect();
            }
            DanceMove::Exchange(a, b) => {
                programs.swap(a, b);
            }
            DanceMove::Partner(a, b) => {
                let a_index = programs.iter().position(|&c| c == a).unwrap();
                let b_index = programs.iter().position(|&c| c == b).unwrap();
                programs.swap(a_index, b_index);
            }
        }
    }

    programs
}

fn parse_input(input: &str) -> Result<Vec<DanceMove>, SimpleError> {
    let line = crate::read_single_line(input)?;

    line.split(',').map(DanceMove::from_str).collect()
}

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input, 16)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(String::from("baedc")), solve_part_1("s1,x3/4,pe/b", 5));
    }
}