//! Day 13: Knights of the Dinner Table
//! https://adventofcode.com/2015/day/13

use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug)]
struct Person {
    name: String,
    happiness_relations: HashMap<String, i32>,
}

impl Person {
    fn new(name: String) -> Self {
        Self { name, happiness_relations: HashMap::new() }
    }
}

fn solve_part(input: &str, include_you: bool) -> Result<i32, SimpleError> {
    let mut people = parse_input(input)?;

    if include_you {
        let you_name = "__you__";

        for person in people.values_mut() {
            person.happiness_relations.insert(String::from(you_name), 0);
        }

        let mut you = Person::new(String::from(you_name));
        for person_name in people.keys() {
            you.happiness_relations.insert(person_name.clone(), 0);
        }
        people.insert(String::from(you_name), you);
    }

    let arrangements = permutations(&people.values().collect());

    let max_happiness = arrangements.into_iter().map(|permutation| {
        let mut total_happiness = 0;
        for i in 0..permutation.len() {
            let prev_index = if i == 0 { permutation.len() - 1 } else { i - 1 };
            let next_index = (i + 1) % permutation.len();

            let person = permutation[i];
            total_happiness += person.happiness_relations.get(&permutation[prev_index].name).unwrap();
            total_happiness += person.happiness_relations.get(&permutation[next_index].name).unwrap();
        }
        total_happiness
    })
        .max()
        .unwrap_or(0);

    Ok(max_happiness)
}

fn permutations<'a, T>(items: &Vec<&'a T>) -> Vec<Vec<&'a T>> {
    let mut permutations: Vec<Vec<&T>> = Vec::new();
    permutations_helper(&mut permutations, items, Vec::new());
    permutations
}

fn permutations_helper<'a, T>(permutations: &mut Vec<Vec<&'a T>>, items: &Vec<&'a T>, visited: Vec<usize>) {
    if visited.len() == items.len() {
        permutations.push(visited.into_iter().map(|i| items[i]).collect());
        return;
    }

    for i in 0..items.len() {
        if !visited.contains(&i) {
            let mut new_visited = visited.clone();
            new_visited.push(i);
            permutations_helper(permutations, items, new_visited);
        }
    }
}

fn parse_input(input: &str) -> Result<HashMap<String, Person>, SimpleError> {
    let mut people: HashMap<String, Person> = HashMap::new();

    for line in input.lines() {
        let split: Vec<_> = line.split(' ').collect();
        if split.len() != 11 {
            return Err(SimpleError::new(format!("invalid line format: {line}")));
        }

        let person_name = split[0];
        let mut other_name = split[split.len() - 1];
        other_name = &other_name[..other_name.len() - 1];

        let mut happiness_change: i32 = split[3].parse()?;
        if split[2] == "lose" {
            happiness_change = -happiness_change;
        }

        if let Some(person) = people.get_mut(person_name) {
            person.happiness_relations.insert(String::from(other_name), happiness_change);
        } else {
            let mut person = Person::new(String::from(person_name));
            person.happiness_relations.insert(String::from(other_name), happiness_change);
            people.insert(String::from(person_name), person);
        }
    }

    Ok(people)
}

pub fn solve(input: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let solution1 = solve_part(input, false)?;
    let solution2 = solve_part(input, true)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample13.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(330), solve_part(SAMPLE_INPUT, false));
    }
}