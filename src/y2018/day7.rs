//! Day 7: The Sum of Its Parts
//! https://adventofcode.com/2018/day/7

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::iter;
use crate::SimpleError;

#[derive(Debug, Clone, Copy)]
struct Worker {
    current_letter: Option<char>,
    time_left: u32,
}

impl Worker {
    fn new() -> Self {
        Self {
            current_letter: None,
            time_left: 0,
        }
    }
}

fn solve_part_1(input: &str) -> Result<String, SimpleError> {
    let requirements = parse_input(input)?;

    let all_letters = get_all_letters(&requirements);

    let requirements_map = build_requirements_map(&requirements, &all_letters);

    let sorted = topological_sort(&requirements_map, &all_letters);

    Ok(sorted.into_iter().collect())
}

fn solve_part_2(input: &str, workers: usize, add_sixty: bool) -> Result<u32, SimpleError> {
    let requirements = parse_input(input)?;

    let all_letters = get_all_letters(&requirements);

    let requirements_map = build_requirements_map(&requirements, &all_letters);

    let mut visited = HashSet::new();
    let mut total_time = 0;
    let mut workers = vec![Worker::new(); workers];
    loop {
        if workers.iter().all(|worker| worker.time_left > 0) {
            let elapsed = workers.iter().map(|worker| worker.time_left).min().unwrap();
            total_time += elapsed;
            for worker in &mut workers {
                worker.time_left -= elapsed;
            }
        }

        for worker in &mut workers {
            if worker.time_left == 0 && worker.current_letter.is_some() {
                visited.insert(worker.current_letter.unwrap());
                worker.current_letter = None;
            }
        }

        if visited.len() == all_letters.len() {
            break;
        }

        let letters_in_progress: HashSet<_> = workers.iter()
            .filter_map(|worker| worker.current_letter)
            .collect();

        let mut available_letters = HashSet::new();
        for &letter in &all_letters {
            available_letters.extend(
                topological_sort_iteration(&requirements_map, letter, &visited)
            );
        }

        available_letters.retain(|&letter| !letters_in_progress.contains(&letter));

        let mut available_letters: Vec<_> = available_letters.into_iter().collect();
        available_letters.sort();

        let (in_progress_workers, free_workers): (Vec<_>, Vec<_>) = workers.iter_mut()
            .partition(|worker| worker.current_letter.is_some());

        if available_letters.is_empty() {
            let elapsed = in_progress_workers.iter()
                .map(|worker| worker.time_left)
                .min().unwrap();

            total_time += elapsed;
            for worker in in_progress_workers {
                worker.time_left -= elapsed;
            }
        }

        for (free_worker, letter) in free_workers.into_iter().zip(available_letters) {
            free_worker.current_letter = Some(letter);
            free_worker.time_left = time_required(letter, add_sixty);
        }
    }

    Ok(total_time)
}

fn time_required(letter: char, add_sixty: bool) -> u32 {
    1 + (letter as u32) - ('A' as u32) + if add_sixty { 60 } else { 0 }
}

fn get_all_letters(requirements: &[(char, char)]) -> Vec<char> {
    requirements.iter()
        .flat_map(|&(before, after)| vec![before, after])
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn build_requirements_map(requirements: &Vec<(char, char)>, all_letters: &[char]) -> HashMap<char, Vec<char>> {
    let mut requirements_map = HashMap::new();
    for &letter in all_letters {
        requirements_map.insert(letter, Vec::new());
    }
    for &(before, after) in requirements {
        requirements_map.get_mut(&after).unwrap().push(before);
    }

    requirements_map
}

fn topological_sort(requirements: &HashMap<char, Vec<char>>, all_letters: &[char]) -> Vec<char> {
    let mut result = Vec::new();

    let mut visited = HashSet::new();
    while visited.len() < all_letters.len() {
        let mut newly_visited = HashSet::new();
        for &letter in all_letters {
            newly_visited.extend(topological_sort_iteration(requirements, letter, &visited));
        }

        let min_visited = newly_visited.into_iter().min().unwrap();

        result.push(min_visited);
        visited.insert(min_visited);
    }

    result
}

fn topological_sort_iteration(requirements: &HashMap<char, Vec<char>>, letter: char, visited: &HashSet<char>) -> HashSet<char> {
    if visited.contains(&letter) {
        return HashSet::new();
    }

    let unvisited_edges: Vec<_> = requirements.get(&letter).unwrap().iter()
        .copied()
        .filter(|&c| !visited.contains(&c))
        .collect();

    if unvisited_edges.is_empty() {
        return iter::once(letter).collect();
    }

    let mut result = HashSet::new();

    for &edge in &unvisited_edges {
        result.extend(topological_sort_iteration(requirements, edge, visited));
    }

    result
}

fn parse_input(input: &str) -> Result<Vec<(char, char)>, SimpleError> {
    input.lines().map(|line| {
        let split: Vec<_> = line.split(' ').collect();
        if split.len() != 10 {
            return Err(SimpleError::new(format!("expected 10 words in line: {line}")));
        }

        Ok((split[1].parse()?, split[7].parse()?))
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(String, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input, 5, true)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample7.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(String::from("CABDFE")), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(15), solve_part_2(SAMPLE_INPUT, 2, false));
    }
}