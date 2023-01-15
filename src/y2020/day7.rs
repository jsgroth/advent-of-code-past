//! Day 7: Handy Haversacks
//! https://adventofcode.com/2020/day/7

use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone)]
struct InnerBagRule {
    color: String,
    count: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Bag {
    color: String,
    rules: Vec<InnerBagRule>,
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let bag_rules = parse_input(input)?;

    let color_to_rule: HashMap<_, _> = bag_rules.iter()
        .map(|bag| (bag.color.as_str(), bag))
        .collect();

    let mut cache = HashMap::new();

    let shiny_gold_count = bag_rules.iter()
        .filter(|bag| {
            bag.color.as_str() != "shiny gold" && can_contain_shiny_gold(bag, &color_to_rule, &mut cache)
        })
        .count();

    Ok(shiny_gold_count)
}

fn solve_part_2(input: &str) -> Result<u64, SimpleError> {
    let bag_rules = parse_input(input)?;

    let color_to_rule: HashMap<_, _> = bag_rules.iter()
        .map(|bag| (bag.color.as_str(), bag))
        .collect();

    let mut cache = HashMap::new();

    let shiny_gold_bag = *color_to_rule.get("shiny gold").unwrap();
    let shiny_gold_inner_count = count_inner_bags(shiny_gold_bag, &color_to_rule, &mut cache);

    Ok(shiny_gold_inner_count)
}

fn count_inner_bags<'a>(bag: &'a Bag, rules: &HashMap<&'a str, &'a Bag>, cache: &mut HashMap<&'a str, u64>) -> u64 {
    if let Some(&value) = cache.get(bag.color.as_str()) {
        return value;
    }

    let mut total_inner_bags = 0;
    for inner_rule in &bag.rules {
        let inner_bag = rules.get(inner_rule.color.as_str()).copied().unwrap();
        total_inner_bags += inner_rule.count * (1 + count_inner_bags(inner_bag, rules, cache));
    }

    cache.insert(bag.color.as_str(), total_inner_bags);
    total_inner_bags
}

fn can_contain_shiny_gold<'a>(bag: &'a Bag, rules: &HashMap<&'a str, &'a Bag>, cache: &mut HashMap<&'a str, bool>) -> bool {
    if bag.color.as_str() == "shiny gold" {
        return true;
    }

    if let Some(&value) = cache.get(bag.color.as_str()) {
        return value;
    }

    for inner_rule in &bag.rules {
        let inner_bag = rules.get(inner_rule.color.as_str()).copied().unwrap();
        if can_contain_shiny_gold(inner_bag, rules, cache) {
            cache.insert(bag.color.as_str(), true);
            return true;
        }
    }

    cache.insert(bag.color.as_str(), false);
    false
}

fn parse_input(input: &str) -> Result<Vec<Bag>, SimpleError> {
    input.lines().map(|line| {
        let split: Vec<_> = line.splitn(5, ' ').collect();
        if split.len() != 5 {
            return Err(SimpleError::new(format!("line does not have enough spaces: {line}")));
        }

        let color = format!("{} {}", split[0], split[1]);

        if line.ends_with("no other bags.") {
            return Ok(Bag { color, rules: Vec::new() });
        }

        let mut rules = Vec::new();
        for inner_rule in split[4].split(", ") {
            let inner_rule_split: Vec<_> = inner_rule.split(' ').collect();
            if inner_rule_split.len() != 4 {
                return Err(SimpleError::new(format!("inner bag rules are invalid: {line}")));
            }

            let count = inner_rule_split[0].parse()?;
            let color = format!("{} {}", inner_rule_split[1], inner_rule_split[2]);

            rules.push(InnerBagRule { color, count });
        }

        Ok(Bag { color, rules })
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, u64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample7.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample7-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(4), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(32), solve_part_2(SAMPLE_INPUT));
        assert_eq!(Ok(126), solve_part_2(SAMPLE_INPUT_2));
    }
}