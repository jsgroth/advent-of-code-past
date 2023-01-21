//! Day 25: The Halting Problem
//! https://adventofcode.com/2017/day/25

use crate::SimpleError;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug)]
struct State {
    name: char,
    zero_value: bool,
    one_value: bool,
    zero_direction: i32,
    one_direction: i32,
    zero_next_state: char,
    one_next_state: char,
}

#[derive(Debug)]
struct Input {
    states: Vec<State>,
    starting_state: char,
    checksum_steps: usize,
}

fn solve_part(input: &str) -> Result<usize, SimpleError> {
    let Input {
        states,
        starting_state,
        checksum_steps,
    } = parse_input(input)?;

    let states: HashMap<_, _> = states
        .into_iter()
        .map(|state| (state.name, state))
        .collect();

    let mut on_bits = HashSet::new();
    let mut current_state_name = starting_state;
    let mut current_pos = 0;
    for _ in 0..checksum_steps {
        let current_state = states.get(&current_state_name).unwrap();
        let (value, direction, next_state) = if on_bits.contains(&current_pos) {
            (
                current_state.one_value,
                current_state.one_direction,
                current_state.one_next_state,
            )
        } else {
            (
                current_state.zero_value,
                current_state.zero_direction,
                current_state.zero_next_state,
            )
        };

        if value {
            on_bits.insert(current_pos);
        } else {
            on_bits.remove(&current_pos);
        }

        current_pos += direction;
        current_state_name = next_state;
    }

    Ok(on_bits.len())
}

fn parse_input(input: &str) -> Result<Input, SimpleError> {
    let lines: Vec<_> = input.lines().collect();
    if lines.len() < 3 {
        return Err(SimpleError::new(format!(
            "input only has {} lines, expected at least 3",
            lines.len()
        )));
    }

    let starting_state = last_word_without_punctuation(lines[0])?;

    let split: Vec<_> = lines[1].split(' ').collect();
    if split.len() < 2 {
        return Err(SimpleError::new(format!(
            "invalid second line: {}",
            lines[1]
        )));
    }
    let checksum_steps: usize = split[split.len() - 2].parse()?;

    let states: Result<_, _> = lines[3..]
        .split(|s| s.is_empty())
        .map(|state_lines| {
            if state_lines.len() != 9 {
                return Err(SimpleError::new(format!(
                    "expected 8 state lines, have {}",
                    state_lines.len()
                )));
            }

            let name = last_word_without_punctuation(state_lines[0])?;

            let zero_value = "1" == last_word_without_punctuation(state_lines[2])?;
            let one_value = "1" == last_word_without_punctuation(state_lines[6])?;

            let zero_direction = if "left" == last_word_without_punctuation(state_lines[3])? {
                -1
            } else {
                1
            };
            let one_direction = if "left" == last_word_without_punctuation(state_lines[7])? {
                -1
            } else {
                1
            };

            let zero_next_state = last_word_without_punctuation(state_lines[4])?;
            let one_next_state = last_word_without_punctuation(state_lines[8])?;

            Ok(State {
                name: name.parse()?,
                zero_value,
                one_value,
                zero_direction,
                one_direction,
                zero_next_state: zero_next_state.parse()?,
                one_next_state: one_next_state.parse()?,
            })
        })
        .collect();

    Ok(Input {
        states: states?,
        starting_state: starting_state.parse()?,
        checksum_steps,
    })
}

fn last_word_without_punctuation(s: &str) -> Result<&str, SimpleError> {
    match s.split(' ').last() {
        Some(word) => Ok(&word[..word.len() - 1]),
        None => Err(SimpleError::new(format!("string has no spaces: {s}"))),
    }
}

pub fn solve(input: &str) -> Result<(usize, String), Box<dyn Error>> {
    let solution1 = solve_part(input)?;

    Ok((solution1, String::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample25.txt");

    #[test]
    fn test_sample_input() {
        assert_eq!(Ok(3), solve_part(SAMPLE_INPUT));
    }
}
