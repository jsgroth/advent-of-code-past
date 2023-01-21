//! Day 14: Chocolate Charts
//! https://adventofcode.com/2018/day/14

use crate::SimpleError;
use std::error::Error;

const INITIAL_STATE: [u32; 2] = [3, 7];

fn solve_part_1(input: &str) -> Result<String, SimpleError> {
    let num_recipes: usize = crate::read_single_line(input)?.parse()?;

    let mut list = Vec::from(INITIAL_STATE);

    let mut elf0_index = 0;
    let mut elf1_index = 1;
    while list.len() < num_recipes + 10 {
        let new_recipe = list[elf0_index] + list[elf1_index];
        if new_recipe > 9 {
            list.push(1);
            list.push(new_recipe % 10);
        } else {
            list.push(new_recipe);
        }

        elf0_index = (elf0_index + list[elf0_index] as usize + 1) % list.len();
        elf1_index = (elf1_index + list[elf1_index] as usize + 1) % list.len();
    }

    let result = list[num_recipes..num_recipes + 10]
        .iter()
        .map(|&digit| char::from_digit(digit, 10).unwrap())
        .collect();

    Ok(result)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let target_sequence: Vec<_> = crate::read_single_line(input)?
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| SimpleError::new(format!("not a digit: {c}")))
        })
        .collect::<Result<_, _>>()?;

    let mut list = Vec::from(INITIAL_STATE);

    let mut elf0_index = 0;
    let mut elf1_index = 1;
    let mut target_index = 0;
    loop {
        let new_recipe = list[elf0_index] + list[elf1_index];
        let new_values = if new_recipe > 9 {
            vec![1, new_recipe % 10]
        } else {
            vec![new_recipe]
        };

        for (i, &new_value) in new_values.iter().enumerate() {
            if new_value == target_sequence[target_index] {
                target_index += 1;
            } else {
                target_index = usize::from(new_value == target_sequence[0]);
            }

            if target_index == target_sequence.len() {
                return Ok(list.len() + (i + 1) - target_sequence.len());
            }
        }

        for &new_value in &new_values {
            list.push(new_value);
        }

        elf0_index = (elf0_index + list[elf0_index] as usize + 1) % list.len();
        elf1_index = (elf1_index + list[elf1_index] as usize + 1) % list.len();
    }
}

pub fn solve(input: &str) -> Result<(String, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(String::from("5158916779")), solve_part_1("9"));
        assert_eq!(Ok(String::from("0124515891")), solve_part_1("5"));
        assert_eq!(Ok(String::from("9251071085")), solve_part_1("18"));
        assert_eq!(Ok(String::from("5941429882")), solve_part_1("2018"));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(9), solve_part_2("51589"));
        assert_eq!(Ok(5), solve_part_2("01245"));
        assert_eq!(Ok(18), solve_part_2("92510"));
        assert_eq!(Ok(2018), solve_part_2("59414"));
    }
}
