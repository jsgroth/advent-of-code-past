//! Day 19: Medicine for Rudolph
//!
//! <https://adventofcode.com/2015/day/19>

use crate::SimpleError;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::error::Error;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let (replacements_map, molecule) = parse_input(input)?;

    let max_replacement_len = replacements_map.keys().map(String::len).max().unwrap();

    let mut unique_molecules: HashSet<String> = HashSet::new();
    for i in 0..molecule.len() {
        let max_len = cmp::min(molecule.len() - i, max_replacement_len);
        for len in 1..=max_len {
            if let Some(replacements) = replacements_map.get(&molecule[i..i + len]) {
                for replacement in replacements {
                    let mut s = String::from(&molecule[..i]);
                    s.push_str(replacement);
                    s.push_str(&molecule[i + len..]);
                    unique_molecules.insert(s);
                }
            }
        }
    }

    Ok(unique_molecules.len())
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let (replacements_map, target_molecule) = parse_input(input)?;
    let replacements_map = reverse_map(&replacements_map);

    let max_replacement_len = replacements_map.keys().map(String::len).max().unwrap();

    match search_reverse(&target_molecule, &replacements_map, max_replacement_len) {
        Some(result) => Ok(result),
        None => Err(SimpleError::new(String::from("no solution found"))),
    }
}

fn search_reverse(
    molecule: &str,
    replacements_map: &HashMap<String, String>,
    max_replacement_len: usize,
) -> Option<usize> {
    if molecule == "e" {
        return Some(0);
    }

    for i in (0..molecule.len()).rev() {
        let max_len = cmp::min(molecule.len() - i, max_replacement_len);
        for len in (1..=max_len).rev() {
            if let Some(replacement) = replacements_map.get(&molecule[i..i + len]) {
                let mut s = String::from(&molecule[..i]);
                s.push_str(replacement);
                s.push_str(&molecule[i + len..]);

                if let Some(result) = search_reverse(&s, replacements_map, max_replacement_len) {
                    return Some(result + 1);
                }
            }
        }
    }

    None
}

fn reverse_map(replacements_map: &HashMap<String, Vec<String>>) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    for (s, replacements) in replacements_map {
        for replacement in replacements {
            result.insert(replacement.clone(), s.clone());
        }
    }

    result
}

fn parse_input(input: &str) -> Result<(HashMap<String, Vec<String>>, String), SimpleError> {
    let lines: Vec<_> = input.lines().collect();
    let split: Vec<_> = lines.split(|line| line.is_empty()).collect();
    let (replacement_lines, molecule_lines) = match split.as_slice() {
        [a, b] => (*a, *b),
        _ => {
            return Err(SimpleError::new(String::from(
                "input does not contain exactly one blank line",
            )))
        }
    };

    if replacement_lines.is_empty() {
        return Err(SimpleError::new(String::from(
            "no replacement lines before blank line",
        )));
    }

    let mut replacements_map: HashMap<String, Vec<String>> = HashMap::new();
    for replacement_line in replacement_lines {
        let split: Vec<_> = replacement_line.split(' ').collect();
        match split.as_slice() {
            [from, "=>", to] => {
                if let Some(replacements) = replacements_map.get_mut(*from) {
                    replacements.push(String::from(*to));
                } else {
                    let replacements = vec![String::from(*to)];
                    replacements_map.insert(String::from(*from), replacements);
                }
            }
            _ => {
                return Err(SimpleError::new(format!(
                    "invalid replacement line: {replacement_line}"
                )))
            }
        }
    }

    let molecule_line = match molecule_lines.iter().next() {
        Some(line) => String::from(*line),
        None => {
            return Err(SimpleError::new(String::from(
                "no molecule line after blank line",
            )))
        }
    };

    Ok((replacements_map, molecule_line))
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample19.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample19-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(4), solve_part_1(SAMPLE_INPUT));
        assert_eq!(Ok(7), solve_part_1(SAMPLE_INPUT_2));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(3), solve_part_2(SAMPLE_INPUT));
        assert_eq!(Ok(6), solve_part_2(SAMPLE_INPUT_2));
    }
}
