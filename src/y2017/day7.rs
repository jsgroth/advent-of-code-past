//! Day 7: Recursive Circus
//! https://adventofcode.com/2017/day/7

use crate::SimpleError;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug, Clone)]
struct Program {
    name: String,
    weight: u32,
    holding: Vec<String>,
}

#[derive(Debug)]
struct ProgramTreeNode {
    weight: u32,
    holding: Vec<ProgramTreeNode>,
}

impl ProgramTreeNode {
    fn find_fixed_weight(&self) -> Result<u32, u32> {
        if self.holding.is_empty() {
            return Ok(self.weight);
        }

        let holding_weights: Result<Vec<_>, _> = self
            .holding
            .iter()
            .map(|node| node.find_fixed_weight())
            .collect();

        let holding_weights = holding_weights?;

        if holding_weights[1..]
            .iter()
            .all(|&weight| weight == holding_weights[0])
        {
            let weight_sum: u32 = holding_weights.into_iter().sum();
            return Ok(weight_sum + self.weight);
        }

        let mut weight_counts = HashMap::new();
        for &weight in &holding_weights {
            if let Some(value) = weight_counts.get_mut(&weight) {
                *value += 1;
            } else {
                weight_counts.insert(weight, 1);
            }
        }

        let correct_weight = weight_counts
            .iter()
            .find_map(
                |(&weight, &count)| {
                    if count > 1 {
                        Some(weight)
                    } else {
                        None
                    }
                },
            )
            .unwrap();

        let incorrect_index = holding_weights
            .iter()
            .position(|&weight| weight != correct_weight)
            .unwrap();

        let difference = (correct_weight as i32) - (holding_weights[incorrect_index] as i32);

        Err((self.holding[incorrect_index].weight as i32 + difference) as u32)
    }
}

fn solve_part_1(input: &str) -> Result<String, SimpleError> {
    let programs = parse_input(input)?;
    let sorted_programs = topological_sort(&programs);

    Ok(sorted_programs[0].name.clone())
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let programs = parse_input(input)?;
    let sorted_programs = topological_sort(&programs);

    let program_tree_root = build_tree(&sorted_programs);

    match program_tree_root.find_fixed_weight() {
        Err(result) => Ok(result),
        Ok(_) => Err(SimpleError::new(String::from("no solution found"))),
    }
}

fn build_tree(sorted_programs: &[Program]) -> ProgramTreeNode {
    let mut name_to_node: HashMap<&str, ProgramTreeNode> = HashMap::new();
    for program in sorted_programs.iter().rev() {
        let mut holding = Vec::with_capacity(program.holding.len());
        for holding_name in &program.holding {
            holding.push(name_to_node.remove(holding_name.as_str()).unwrap());
        }
        let node = ProgramTreeNode {
            weight: program.weight,
            holding,
        };
        name_to_node.insert(program.name.as_str(), node);
    }

    name_to_node.into_values().next().unwrap()
}

fn topological_sort(programs: &Vec<Program>) -> Vec<Program> {
    let name_map: HashMap<_, _> = programs
        .iter()
        .map(|program| (program.name.as_str(), program))
        .collect();

    let mut result_rev = Vec::new();
    let mut visited = HashSet::new();

    for program in programs {
        sort_visit(program, &name_map, &mut visited, &mut result_rev);
    }

    result_rev.into_iter().rev().collect()
}

fn sort_visit<'a>(
    program: &'a Program,
    programs: &HashMap<&str, &'a Program>,
    visited: &mut HashSet<&'a str>,
    result_rev: &mut Vec<Program>,
) {
    if visited.contains(program.name.as_str()) {
        return;
    }

    visited.insert(program.name.as_str());

    for holding_name in &program.holding {
        sort_visit(
            programs.get(holding_name.as_str()).unwrap(),
            programs,
            visited,
            result_rev,
        );
    }

    result_rev.push(program.clone());
}

fn parse_input(input: &str) -> Result<Vec<Program>, SimpleError> {
    input
        .lines()
        .map(|line| match line.split_once(" -> ") {
            Some((l, r)) => {
                let (name, weight) = parse_name_and_weight(l)?;
                let holding = r.split(", ").map(String::from).collect();
                Ok(Program {
                    name,
                    weight,
                    holding,
                })
            }
            None => {
                let (name, weight) = parse_name_and_weight(line)?;
                Ok(Program {
                    name,
                    weight,
                    holding: Vec::new(),
                })
            }
        })
        .collect()
}

fn parse_name_and_weight(s: &str) -> Result<(String, u32), SimpleError> {
    let (name, weight) = s
        .split_once(' ')
        .ok_or_else(|| SimpleError::new(format!("invalid name/weight string: {s}")))?;

    let weight = weight[1..weight.len() - 1].parse()?;

    Ok((String::from(name), weight))
}

pub fn solve(input: &str) -> Result<(String, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample7.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(String::from("tknk")), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(60), solve_part_2(SAMPLE_INPUT));
    }
}
