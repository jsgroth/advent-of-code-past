//! Day 12: Subterranean Sustainability
//!
//! <https://adventofcode.com/2018/day/12>

use crate::SimpleError;
use std::collections::{HashSet, VecDeque};
use std::error::Error;

fn solve_part_1(input: &str) -> Result<i64, SimpleError> {
    let (initial_state, plant_generate_rules) = parse_input(input)?;

    let mut state = VecDeque::from(initial_state);
    let mut index_0_position = 0;
    for _ in 0..20 {
        let (next_state, index_0_position_diff) = simulate_generation(state, &plant_generate_rules);
        state = next_state;
        index_0_position += index_0_position_diff;
    }

    Ok(score(&state, index_0_position))
}

fn solve_part_2(input: &str) -> Result<i64, SimpleError> {
    let (initial_state, plant_generate_rules) = parse_input(input)?;

    let mut state = VecDeque::from(initial_state);
    let mut index_0_position = 0;
    for i in 1.. {
        let (next_state, index_0_position_diff) =
            simulate_generation(state.clone(), &plant_generate_rules);

        if state == next_state {
            let final_index_0_position =
                (50_000_000_000 - i + 1) * index_0_position_diff + index_0_position;
            return Ok(score(&state, final_index_0_position));
        }

        state = next_state;
        index_0_position += index_0_position_diff;
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn score(plants: &VecDeque<bool>, index_0_position: i64) -> i64 {
    plants
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| {
            if b {
                Some(i as i64 + index_0_position)
            } else {
                None
            }
        })
        .sum()
}

fn simulate_generation(
    mut state: VecDeque<bool>,
    generation_rules: &HashSet<Vec<bool>>,
) -> (VecDeque<bool>, i64) {
    for _ in 0..4 {
        state.push_front(false);
        state.push_back(false);
    }

    let mut index_0_position_diff = -2;

    let mut next_state = VecDeque::new();
    for i in 0..state.len() - 5 {
        let window: Vec<_> = state.range(i..i + 5).copied().collect();
        next_state.push_back(generation_rules.contains(&window));
    }

    while next_state.front() == Some(&false) {
        next_state.pop_front();
        index_0_position_diff += 1;
    }

    while next_state.back() == Some(&false) {
        next_state.pop_back();
    }

    (next_state, index_0_position_diff)
}

fn parse_input(input: &str) -> Result<(Vec<bool>, HashSet<Vec<bool>>), SimpleError> {
    let lines: Vec<_> = input.lines().collect();
    if lines.len() < 3 {
        return Err(SimpleError::new(format!(
            "input only has {} lines, expected at least 3",
            lines.len()
        )));
    }

    let initial_state: Vec<_> = lines[0]["initial state: ".len()..]
        .chars()
        .map(|c| c == '#')
        .collect();

    let plant_generate_rules: HashSet<_> = lines[2..]
        .iter()
        .filter(|line| line.ends_with('#'))
        .map(|line| line[..5].chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();

    Ok((initial_state, plant_generate_rules))
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample12.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(325), solve_part_1(SAMPLE_INPUT));
    }
}
