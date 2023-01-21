//! Day 21: Allergen Assessment
//!
//! <https://adventofcode.com/2020/day/21>

use crate::SimpleError;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
struct FoodItem<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

fn solve_both_parts(input: &str) -> Result<(usize, String), SimpleError> {
    let food = parse_input(input)?;

    let num_allergens = food
        .iter()
        .flat_map(|food_item| food_item.allergens.iter().copied())
        .collect::<HashSet<_>>()
        .len();

    let mut unknown_ingredients: HashSet<_> = food
        .iter()
        .flat_map(|food_item| food_item.ingredients.iter().copied())
        .collect();

    let mut allergen_to_ingredient: HashMap<&str, &str> = HashMap::new();
    while allergen_to_ingredient.len() < num_allergens {
        let mut allergen_to_possible_ingredients: HashMap<&str, Vec<&str>> = HashMap::new();

        for food_item in &food {
            let possible_ingredients: Vec<_> = food_item
                .ingredients
                .iter()
                .copied()
                .filter(|&ingredient| unknown_ingredients.contains(ingredient))
                .collect();

            for &allergen in &food_item.allergens {
                if let Some(existing_ingredients) =
                    allergen_to_possible_ingredients.get_mut(allergen)
                {
                    *existing_ingredients = existing_ingredients
                        .iter()
                        .copied()
                        .filter(|&ingredient| possible_ingredients.contains(&ingredient))
                        .collect();
                } else {
                    allergen_to_possible_ingredients.insert(allergen, possible_ingredients.clone());
                }
            }
        }

        let found_ingredients: Vec<_> = allergen_to_possible_ingredients
            .into_iter()
            .filter(|(_, possible_ingredients)| possible_ingredients.len() == 1)
            .collect();
        for (allergen, possible_ingredients) in found_ingredients {
            let ingredient = possible_ingredients[0];
            allergen_to_ingredient.insert(allergen, ingredient);
            unknown_ingredients.remove(ingredient);
        }
    }

    let allergen_free_ingredient_count = food
        .iter()
        .map(|food_item| {
            food_item
                .ingredients
                .iter()
                .filter(|&&ingredient| unknown_ingredients.contains(ingredient))
                .count()
        })
        .sum();

    let mut dangerous_ingredients: Vec<_> = allergen_to_ingredient.into_iter().collect();
    dangerous_ingredients.sort_by_key(|&(allergen, _)| allergen);

    let dangerous_ingredient_list: Vec<_> = dangerous_ingredients
        .into_iter()
        .map(|(_, ingredient)| ingredient)
        .collect();

    Ok((
        allergen_free_ingredient_count,
        dangerous_ingredient_list.join(","),
    ))
}

fn parse_input(input: &str) -> Result<Vec<FoodItem>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let open_paren_index = line
                .chars()
                .position(|c| c == '(')
                .ok_or_else(|| SimpleError::new(format!("line does not contain '(': {line}")))?;

            let ingredients = line[..open_paren_index - 1].split(' ').collect();

            let allergens = line[open_paren_index + 1 + "contains ".len()..line.len() - 1]
                .split(", ")
                .collect();

            Ok(FoodItem {
                ingredients,
                allergens,
            })
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, String), Box<dyn Error>> {
    let (solution1, solution2) = solve_both_parts(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample21.txt");

    #[test]
    fn test_sample_input() {
        assert_eq!(
            Ok((5, String::from("mxmxvkd,sqjhc,fvjkl"))),
            solve_both_parts(SAMPLE_INPUT)
        );
    }
}
