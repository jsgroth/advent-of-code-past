//! Day 24: It Hangs in the Balance
//!
//! <https://adventofcode.com/2015/day/24>

use crate::SimpleError;
use std::cmp;
use std::collections::HashSet;
use std::error::Error;
use std::num::ParseIntError;

fn solve_part_1(input: &str) -> Result<u64, SimpleError> {
    let weights = parse_input(input)?;

    let total_weight: u64 = weights.iter().copied().sum();

    let all_groupings = find_all_shortest_groupings(
        &weights,
        0,
        total_weight / 3,
        Vec::new(),
        &mut usize::MAX.clone(),
    );
    let valid_groupings = find_valid_shortest_groupings(&all_groupings, &weights, total_weight, 3);

    let min_qe = find_qe_for_best_grouping(&valid_groupings);

    Ok(min_qe)
}

fn solve_part_2(input: &str) -> Result<u64, SimpleError> {
    let weights = parse_input(input)?;

    let total_weight: u64 = weights.iter().copied().sum();

    let all_groupings = find_all_shortest_groupings(
        &weights,
        0,
        total_weight / 4,
        Vec::new(),
        &mut usize::MAX.clone(),
    );
    let valid_groupings = find_valid_shortest_groupings(&all_groupings, &weights, total_weight, 4);

    let min_qe = find_qe_for_best_grouping(&valid_groupings);

    Ok(min_qe)
}

fn find_qe_for_best_grouping(groupings: &[Vec<u64>]) -> u64 {
    groupings
        .iter()
        .map(|a| a.iter().product::<u64>())
        .min()
        .unwrap_or(0)
}

fn find_valid_shortest_groupings(
    groupings: &[Vec<u64>],
    weights: &[u64],
    total_weight: u64,
    target_groups: usize,
) -> Vec<Vec<u64>> {
    let min_grouping_len = groupings
        .iter()
        .map(|grouping| grouping.len())
        .min()
        .unwrap_or(0);

    groupings
        .iter()
        .filter(|grouping| {
            grouping.len() == min_grouping_len
                && grouping_exists(
                    weights,
                    0,
                    total_weight / (target_groups as u64),
                    grouping.iter().copied().collect(),
                    Vec::new(),
                    target_groups - 1,
                )
        })
        .cloned()
        .collect()
}

fn find_all_shortest_groupings(
    weights: &[u64],
    index: usize,
    target_weight: u64,
    group_so_far: Vec<u64>,
    min_group_len: &mut usize,
) -> Vec<Vec<u64>> {
    let group_sum: u64 = group_so_far.iter().copied().sum();
    if group_sum == target_weight {
        *min_group_len = cmp::min(*min_group_len, group_so_far.len());
        return if group_so_far.len() == *min_group_len {
            vec![group_so_far]
        } else {
            Vec::new()
        };
    }

    if group_so_far.len() >= *min_group_len {
        return Vec::new();
    }

    if index == weights.len() {
        return Vec::new();
    }

    let mut groupings = Vec::new();
    groupings.extend(find_all_shortest_groupings(
        weights,
        index + 1,
        target_weight,
        group_so_far.clone(),
        min_group_len,
    ));

    let weight = weights[index];
    if weight + group_sum <= target_weight {
        let mut new_group_so_far = group_so_far;
        new_group_so_far.push(weight);
        groupings.extend(find_all_shortest_groupings(
            weights,
            index + 1,
            target_weight,
            new_group_so_far,
            min_group_len,
        ));
    }

    groupings
}

fn grouping_exists(
    weights: &[u64],
    index: usize,
    target_weight: u64,
    visited: HashSet<u64>,
    group_so_far: Vec<u64>,
    target_groups: usize,
) -> bool {
    if target_groups == 1 {
        return true;
    }

    let group_sum: u64 = group_so_far.iter().copied().sum();
    if group_sum == target_weight {
        let mut new_visited = visited;
        new_visited.extend(group_so_far);
        return grouping_exists(
            weights,
            0,
            target_weight,
            new_visited,
            Vec::new(),
            target_groups - 1,
        );
    }

    if index == weights.len() {
        return false;
    }

    if grouping_exists(
        weights,
        index + 1,
        target_weight,
        visited.clone(),
        group_so_far.clone(),
        target_groups,
    ) {
        return true;
    }

    let weight = weights[index];
    if !visited.contains(&weight) && group_sum + weight <= target_weight {
        let mut new_group_so_far = group_so_far;
        new_group_so_far.push(weight);
        if grouping_exists(
            weights,
            index + 1,
            target_weight,
            visited,
            new_group_so_far,
            target_groups,
        ) {
            return true;
        }
    }

    false
}

fn parse_input(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input.lines().map(|line| line.parse::<u64>()).collect()
}

pub fn solve(input: &str) -> Result<(u64, u64), Box<dyn Error>> {
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
        assert_eq!(Ok(99), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(44), solve_part_2(SAMPLE_INPUT));
    }
}
