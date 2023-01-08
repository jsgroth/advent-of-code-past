//! Day 2: 1202 Program Alarm
//! https://adventofcode.com/2019/day/2

use std::error::Error;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<i64, SimpleError> {
    let mut program = parse_input(input)?;

    program[1] = 12;
    program[2] = 2;

    execute_program(&mut program);

    Ok(program[0])
}

fn solve_part_2(input: &str) -> Result<i64, SimpleError> {
    let program = parse_input(input)?;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;

            execute_program(&mut program);

            if program[0] == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn execute_program(program: &mut Vec<i64>) {
    let mut ip = 0;
    while ip < program.len() {
        match program[ip] {
            1 => {
                let a = program[program[ip + 1] as usize];
                let b = program[program[ip + 2] as usize];
                let c = program[ip + 3] as usize;
                program[c] = a + b;
            }
            2 => {
                let a = program[program[ip + 1] as usize];
                let b = program[program[ip + 2] as usize];
                let c = program[ip + 3] as usize;
                program[c] = a * b;
            }
            99 => {
                break;
            }
            _ => panic!("invalid opcode: {}", program[ip])
        }

        ip += 4;
    }
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

fn parse_input(input: &str) -> Result<Vec<i64>, SimpleError> {
    crate::read_single_line(input)?
        .split(',')
        .map(|n| n.parse::<i64>().map_err(SimpleError::from))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_program_1() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        execute_program(&mut program);
        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], program);
    }

    #[test]
    fn test_execute_program_2() {
        let mut program = vec![1, 0, 0, 0, 99];
        execute_program(&mut program);
        assert_eq!(vec![2, 0, 0, 0, 99], program);
    }

    #[test]
    fn test_execute_program_3() {
        let mut program = vec![2, 3, 0, 3, 99];
        execute_program(&mut program);
        assert_eq!(vec![2, 3, 0, 6, 99], program);
    }

    #[test]
    fn test_execute_program_4() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        execute_program(&mut program);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], program);
    }

    #[test]
    fn test_execute_program_5() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute_program(&mut program);
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], program);
    }
}