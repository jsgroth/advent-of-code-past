//! Day 17: No Such Thing as Too Much
//!
//! <https://adventofcode.com/2015/day/17>

use crate::SimpleError;
use std::cmp;
use std::error::Error;
use std::num::ParseIntError;

fn solve_part_1(input: &str, target: u32) -> Result<usize, SimpleError> {
    let container_sizes = parse_input(input)?;

    Ok(search_combinations(&container_sizes, target, 0, 0, None))
}

fn solve_part_2(input: &str, target: u32) -> Result<usize, SimpleError> {
    let container_sizes = parse_input(input)?;

    let min_containers = find_min_containers(&container_sizes, target, 0, 0);

    Ok(search_combinations(
        &container_sizes,
        target,
        0,
        0,
        Some(min_containers),
    ))
}

fn search_combinations(
    container_sizes: &[u32],
    target: u32,
    current_total: u32,
    containers: usize,
    target_containers: Option<usize>,
) -> usize {
    if current_total == target {
        return 1;
    }

    if current_total > target
        || container_sizes.is_empty()
        || (target_containers.is_some() && containers >= target_containers.unwrap())
    {
        return 0;
    }

    search_combinations(
        &container_sizes[1..],
        target,
        current_total,
        containers,
        target_containers,
    ) + search_combinations(
        &container_sizes[1..],
        target,
        current_total + container_sizes[0],
        containers + 1,
        target_containers,
    )
}

fn find_min_containers(
    container_sizes: &[u32],
    target: u32,
    current_total: u32,
    containers: usize,
) -> usize {
    if current_total == target {
        return containers;
    }

    if current_total > target || container_sizes.is_empty() {
        return usize::MAX;
    }

    let mut result = usize::MAX;

    result = cmp::min(
        result,
        find_min_containers(&container_sizes[1..], target, current_total, containers),
    );
    result = cmp::min(
        result,
        find_min_containers(
            &container_sizes[1..],
            target,
            current_total + container_sizes[0],
            containers + 1,
        ),
    );

    result
}

fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|line| line.parse::<u32>()).collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input, 150)?;
    let solution2 = solve_part_2(input, 150)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample17.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(4), solve_part_1(SAMPLE_INPUT, 25));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(3), solve_part_2(SAMPLE_INPUT, 25));
    }
}
