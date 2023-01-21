//! Day 19: Go With The Flow
//! https://adventofcode.com/2018/day/19

use crate::y2018::chronodevice::ChronoInstruction;
use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<u64, SimpleError> {
    let (ip, instructions) = parse_input(input)?;

    let mut registers = [0; 6];

    let mut pc = 0;
    while pc < instructions.len() {
        registers[ip] = pc as u64;

        let instruction = &instructions[pc];
        registers[instruction.c] = instruction
            .op
            .execute(&registers, instruction.a, instruction.b);
        pc = registers[ip] as usize;

        pc += 1;
    }

    Ok(registers[0])
}

// I have no idea how well this solution generalizes to other people's inputs
fn solve_part_2(input: &str) -> Result<u64, SimpleError> {
    let (ip, instructions) = parse_input(input)?;

    let mut registers = [0; 6];
    registers[0] = 1;

    let mut pc = 0;
    let mut last_assigned_value = 0;
    while pc != 1 {
        registers[ip] = pc as u64;

        let instruction = &instructions[pc];
        registers[instruction.c] = instruction
            .op
            .execute(&registers, instruction.a, instruction.b);
        pc = registers[ip] as usize;

        if instruction.c != ip && instruction.c != 0 {
            last_assigned_value = registers[instruction.c];
        }

        pc += 1;
    }

    let n = last_assigned_value;
    let sqrt_n = (n as f64).sqrt().floor() as u64;
    let mut divisor_sum = 0;
    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisor_sum += i + n / i;
        }
    }

    Ok(divisor_sum)
}

fn parse_input(input: &str) -> Result<(usize, Vec<ChronoInstruction>), SimpleError> {
    let first_line = crate::read_single_line(input)?;

    if !first_line.starts_with("#ip ") {
        return Err(SimpleError::new(format!(
            "expected '#ip ' prefix in first line: {first_line}"
        )));
    }

    let ip = first_line[4..].parse()?;

    let instructions: Result<_, _> = input
        .lines()
        .skip(1)
        .map(ChronoInstruction::from_line)
        .collect();
    let instructions = instructions?;

    Ok((ip, instructions))
}

pub fn solve(input: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample19.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(6), solve_part_1(SAMPLE_INPUT));
    }
}
