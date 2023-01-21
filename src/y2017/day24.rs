//! Day 24: Electromagnetic Moat
//! https://adventofcode.com/2017/day/24

use crate::SimpleError;
use std::cmp;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let components = parse_input(input)?;

    let visited = vec![false; components.len()];
    let max_strength = search(&components, visited, 0, false, 0, None);

    Ok(max_strength)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let components = parse_input(input)?;

    let visited = vec![false; components.len()];
    let longest_bridge_len = search(&components, visited.clone(), 0, true, 0, None);

    let max_strength = search(&components, visited, 0, false, 0, Some(longest_bridge_len));

    Ok(max_strength)
}

fn search(
    components: &Vec<(u32, u32)>,
    visited: Vec<bool>,
    target_end: u32,
    find_longest_len: bool,
    prefix_score: u32,
    path_len_filter: Option<u32>,
) -> u32 {
    if let Some(path_len_filter) = path_len_filter {
        if visited.iter().filter(|&&b| b).count() == path_len_filter as usize {
            return prefix_score;
        }
    }

    let mut max_score = if path_len_filter.is_none() {
        prefix_score
    } else {
        0
    };
    for (i, &component) in components.iter().enumerate() {
        if visited[i] {
            continue;
        }

        let component_score = if find_longest_len {
            1
        } else {
            component.0 + component.1
        };

        if component.0 == target_end || component.1 == target_end {
            let mut new_visited = visited.clone();
            new_visited[i] = true;
            let new_target_end = if component.0 == target_end {
                component.1
            } else {
                component.0
            };
            let path_score = search(
                components,
                new_visited,
                new_target_end,
                find_longest_len,
                prefix_score + component_score,
                path_len_filter,
            );
            max_score = cmp::max(max_score, path_score);
        }
    }

    max_score
}

fn parse_input(input: &str) -> Result<Vec<(u32, u32)>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line
                .split_once('/')
                .ok_or_else(|| SimpleError::new(format!("line has no '/': {line}")))?;
            Ok((l.parse()?, r.parse()?))
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample24.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(31), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(19), solve_part_2(SAMPLE_INPUT));
    }
}
