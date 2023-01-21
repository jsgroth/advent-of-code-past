//! Day 14: Space Stoichiometry
//! https://adventofcode.com/2019/day/14

use crate::SimpleError;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Chemical {
    name: String,
    amount: u32,
}

impl FromStr for Chemical {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (amount, name) = s.split_once(' ').ok_or_else(|| {
            SimpleError::new(format!("invalid chemical string, expected one space: {s}"))
        })?;

        let name = String::from(name);
        let amount = amount.parse()?;

        Ok(Self { name, amount })
    }
}

#[derive(Debug, Clone)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let reactions = parse_input(input)?;

    let reactions_dag = build_dag(&reactions);

    let sorted_chem_names = topological_sort(&reactions_dag);

    let name_to_reaction: HashMap<_, _> = reactions
        .iter()
        .map(|reaction| (reaction.output.name.as_str(), reaction))
        .collect();

    let mut required_chems: HashMap<&str, u32> = HashMap::new();
    for chem_name in &sorted_chem_names {
        let required = if chem_name.as_str() == "FUEL" {
            1
        } else {
            required_chems.get(chem_name.as_str()).copied().unwrap_or(0)
        };

        if chem_name.as_str() == "ORE" {
            return Ok(required);
        }

        let reaction = name_to_reaction.get(chem_name.as_str()).copied().unwrap();

        let amount_to_produce = (required as f64 / reaction.output.amount as f64).ceil() as u32;

        for input_reaction in &reaction.inputs {
            let input_required = amount_to_produce * input_reaction.amount;
            if let Some(value) = required_chems.get_mut(input_reaction.name.as_str()) {
                *value += input_required;
            } else {
                required_chems.insert(input_reaction.name.as_str(), input_required);
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn solve_part_2(input: &str) -> Result<u64, SimpleError> {
    let reactions = parse_input(input)?;

    let reactions_dag = build_dag(&reactions);

    let sorted_chem_names = topological_sort(&reactions_dag);

    let name_to_reaction: HashMap<_, _> = reactions
        .iter()
        .map(|reaction| (reaction.output.name.as_str(), reaction))
        .collect();

    let mut required_chems: HashMap<&str, f64> = HashMap::new();
    for chem_name in &sorted_chem_names {
        let required = if chem_name.as_str() == "FUEL" {
            1.0
        } else {
            required_chems
                .get(chem_name.as_str())
                .copied()
                .unwrap_or(0.0)
        };

        if chem_name.as_str() == "ORE" {
            return Ok((1_000_000_000_000.0 / required).floor() as u64);
        }

        let reaction = name_to_reaction.get(chem_name.as_str()).copied().unwrap();

        let amount_to_produce = required / reaction.output.amount as f64;

        for input_reaction in &reaction.inputs {
            let input_required = amount_to_produce * input_reaction.amount as f64;
            if let Some(value) = required_chems.get_mut(input_reaction.name.as_str()) {
                *value += input_required;
            } else {
                required_chems.insert(input_reaction.name.as_str(), input_required);
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn build_dag(reactions: &[Reaction]) -> HashMap<String, Vec<String>> {
    let mut reactions_dag = HashMap::new();

    for reaction in reactions {
        let input_names: Vec<_> = reaction
            .inputs
            .iter()
            .map(|chemical| chemical.name.clone())
            .collect();
        reactions_dag.insert(reaction.output.name.clone(), input_names);
    }

    reactions_dag.insert(String::from("ORE"), Vec::new());

    reactions_dag
}

fn topological_sort(reactions_dag: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut output_reverse = Vec::new();

    for output_name in reactions_dag.keys() {
        topological_sort_visit(
            output_name,
            reactions_dag,
            &mut visited,
            &mut output_reverse,
        );
    }

    output_reverse.into_iter().rev().collect()
}

fn topological_sort_visit(
    name: &str,
    reactions_dag: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    output_reverse: &mut Vec<String>,
) {
    if visited.contains(name) {
        return;
    }

    visited.insert(String::from(name));

    for input_name in reactions_dag.get(name).unwrap() {
        topological_sort_visit(input_name, reactions_dag, visited, output_reverse);
    }

    output_reverse.push(String::from(name));
}

fn parse_input(input: &str) -> Result<Vec<Reaction>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let (inputs, output) = line.split_once(" => ").ok_or_else(|| {
                SimpleError::new(format!("invalid line format, no ' => ': {line}"))
            })?;

            let inputs: Vec<_> = inputs
                .split(", ")
                .map(Chemical::from_str)
                .collect::<Result<_, _>>()?;

            let output = output.parse()?;

            Ok(Reaction { inputs, output })
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(u32, u64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = include_str!("sample_input/sample14.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample14-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("sample_input/sample14-3.txt");
    const SAMPLE_INPUT_4: &str = include_str!("sample_input/sample14-4.txt");
    const SAMPLE_INPUT_5: &str = include_str!("sample_input/sample14-5.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(31), solve_part_1(SAMPLE_INPUT_1));
        assert_eq!(Ok(165), solve_part_1(SAMPLE_INPUT_2));
        assert_eq!(Ok(13312), solve_part_1(SAMPLE_INPUT_3));
        assert_eq!(Ok(180697), solve_part_1(SAMPLE_INPUT_4));
        assert_eq!(Ok(2210736), solve_part_1(SAMPLE_INPUT_5));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(82892753), solve_part_2(SAMPLE_INPUT_3));
        assert_eq!(Ok(5586022), solve_part_2(SAMPLE_INPUT_4));
        assert_eq!(Ok(460664), solve_part_2(SAMPLE_INPUT_5));
    }
}
