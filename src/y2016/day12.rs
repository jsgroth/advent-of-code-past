//! Day 12: Leonardo's Monorail
//! https://adventofcode.com/2016/day/12

use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;
use crate::y2016::assembunny::AssembunnyInstruction;

fn solve_part(input: &str, initial_c_value: i64) -> Result<i64, SimpleError> {
    let instructions = parse_input(input)?;

    let mut registers: HashMap<_, i64> =
        [('a', 0), ('b', 0), ('c', initial_c_value), ('d', 0)].into_iter().collect();

    let mut pc = 0;
    while pc < instructions.len() {
        let instruction = &instructions[pc];
        instruction.execute(&mut registers, &mut pc);
    }

    Ok(*registers.get(&'a').unwrap())
}

fn parse_input(input: &str) -> Result<Vec<AssembunnyInstruction>, SimpleError> {
    input.lines().map(AssembunnyInstruction::from_line).collect()
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part(input, 0)?;
    let solution2 = solve_part(input, 1)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample12.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(42), solve_part(SAMPLE_INPUT, 0));
    }
}