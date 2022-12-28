//! Day 15: Science for Hungry People
//! https://adventofcode.com/2015/day/15

use std::cmp;
use std::error::Error;
use std::num::ParseIntError;
use crate::SimpleError;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn solve_part_1(input: &str) -> Result<i32, SimpleError> {
    let ingredients = parse_input(input)?;

    Ok(search_for_max(&ingredients, Vec::new(), 100, None))
}

fn solve_part_2(input: &str) -> Result<i32, SimpleError> {
    let ingredients = parse_input(input)?;

    Ok(search_for_max(&ingredients, Vec::new(), 100, Some(500)))
}

fn search_for_max(ingredients: &Vec<Ingredient>, teaspoons: Vec<i32>, remaining: i32, calorie_req: Option<i32>) -> i32 {
    if teaspoons.len() == ingredients.len() - 1 {
        let mut teaspoons = teaspoons;
        teaspoons.push(remaining);
        return score_cookie(ingredients, &teaspoons, calorie_req);
    }

    let mut result = i32::MIN;
    for i in 0..=remaining {
        let mut new_teaspoons = teaspoons.clone();
        new_teaspoons.push(i);
        result = cmp::max(result, search_for_max(ingredients, new_teaspoons, remaining - i, calorie_req));
    }

    result
}

fn score_cookie(ingredients: &Vec<Ingredient>, teaspoons: &Vec<i32>, calorie_req: Option<i32>) -> i32 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;

    for (ingredient, &ingredient_teaspoons) in ingredients.iter().zip(teaspoons) {
        capacity += ingredient.capacity * ingredient_teaspoons;
        durability += ingredient.durability * ingredient_teaspoons;
        flavor += ingredient.flavor * ingredient_teaspoons;
        texture += ingredient.texture * ingredient_teaspoons;
        calories += ingredient.calories * ingredient_teaspoons;
    }

    if calorie_req.is_some() && Some(calories) != calorie_req {
        return 0;
    }

    if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
        0
    } else {
        capacity * durability * flavor * texture
    }
}

fn parse_input(input: &str) -> Result<Vec<Ingredient>, SimpleError> {
    input.lines().map(|line| {
        let split: Vec<_> = line.split(' ').collect();

        if split.len() != 11 {
            return Err(SimpleError::new(format!("invalid line format: {line}")));
        }

        let capacity = parse_int(split[2])?;
        let durability = parse_int(split[4])?;
        let flavor = parse_int(split[6])?;
        let texture = parse_int(split[8])?;
        let calories = parse_int(split[10])?;

        Ok(Ingredient { capacity, durability, flavor, texture, calories })
    })
        .collect()
}

fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    if s.chars().last() == Some(',') {
        s[..s.len() - 1].parse()
    } else {
        s.parse()
    }
}

pub fn solve(input: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample15.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(62842880), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(57600000), solve_part_2(SAMPLE_INPUT));
    }
}