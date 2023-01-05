//! Day 16: Chronal Classification
//! https://adventofcode.com/2018/day/16

use std::collections::HashSet;
use std::error::Error;
use crate::SimpleError;
use crate::y2018::chronodevice::ChronoOperation;

#[derive(Debug, Clone)]
struct OpTest {
    before: [u32; 4],
    after: [u32; 4],
    opcode: usize,
    a: u32,
    b: u32,
    c: usize,
}

#[derive(Debug, Clone, Copy)]
struct TestInstruction {
    opcode: usize,
    a: u32,
    b: u32,
    c: usize,
}

impl OpTest {
    fn from_lines(lines: &[&str]) -> Result<Self, SimpleError> {
        if lines.len() != 3 {
            return Err(SimpleError::new(format!("expected 3 lines, got {}", lines.len())));
        }

        let before = parse_registers(&lines[0]["Before: ".len()..])?;
        let after = parse_registers(&lines[2]["After:  ".len()..])?;

        let split: Vec<_> = lines[1].split(' ').collect();
        if split.len() != 4 {
            return Err(SimpleError::new(format!("op line has incorrect number of spaces: {}", lines[1])));
        }

        let opcode = split[0].parse()?;
        let a = split[1].parse()?;
        let b = split[2].parse()?;
        let c = split[3].parse()?;

        Ok(Self {
            before,
            after,
            opcode,
            a,
            b,
            c,
        })
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let (op_tests, _) = parse_input(input)?;

    let result = op_tests.into_iter()
        .filter(|op_test| {
            let can_produce_count = ChronoOperation::ALL.iter()
                .filter(|op| op.can_produce(&op_test.before, &op_test.after, op_test.a, op_test.b, op_test.c))
                .count();
            can_produce_count >= 3
        })
        .count();

    Ok(result)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let (op_tests, test_program) = parse_input(input)?;

    let opcode_mapping = solve_for_opcodes(&op_tests);

    let mut registers = [0, 0, 0, 0];
    for instruction in &test_program {
        let op = opcode_mapping[instruction.opcode];
        registers[instruction.c] = op.execute(&registers, instruction.a, instruction.b);
    }

    Ok(registers[0])
}

fn solve_for_opcodes(op_tests: &[OpTest]) -> Vec<ChronoOperation> {
    let mut opcode_to_operation: Vec<Option<ChronoOperation>> = vec![None; ChronoOperation::ALL.len()];

    let mut found_operations = HashSet::new();
    while found_operations.len() < ChronoOperation::ALL.len() {
        for op_test in op_tests {
            let can_produce_ops: Vec<_> = ChronoOperation::ALL.iter().copied()
                .filter(|&op| !found_operations.contains(&op))
                .filter(|&op| {
                    op.can_produce(&op_test.before, &op_test.after, op_test.a, op_test.b, op_test.c)
                })
                .collect();
            if can_produce_ops.len() == 1 {
                opcode_to_operation[op_test.opcode] = Some(can_produce_ops[0]);
                found_operations.insert(can_produce_ops[0]);
            }
        }
    }

    opcode_to_operation.into_iter().map(Option::unwrap).collect()
}

fn parse_input(input: &str) -> Result<(Vec<OpTest>, Vec<TestInstruction>), SimpleError> {
    let lines: Vec<_> = input.lines().collect();

    let triple_blank_line_index = find_triple_blank_line_index(&lines)?;

    let mut op_tests = Vec::new();
    for line_group in lines[..triple_blank_line_index].split(|s| s.is_empty()) {
        op_tests.push(OpTest::from_lines(line_group)?);
    }

    let test_instructions: Vec<_> = lines[triple_blank_line_index + 3 ..].iter()
        .map(|line| {
            let split: Vec<_> = line.split(' ').collect();
            if split.len() != 4 {
                return Err(SimpleError::new(format!("invalid instruction line: {line}")));
            }

            Ok(TestInstruction {
                opcode: split[0].parse()?,
                a: split[1].parse()?,
                b: split[2].parse()?,
                c: split[3].parse()?,
            })
        })
        .collect::<Result<_, _>>()?;

    Ok((op_tests, test_instructions))
}

fn find_triple_blank_line_index(lines: &[&str]) -> Result<usize, SimpleError> {
    for (i, window) in lines.windows(3).enumerate() {
        if window.iter().all(|s| s.is_empty()) {
            return Ok(i);
        }
    }

    Err(SimpleError::new(String::from("input has no triple blank line")))
}

fn parse_registers(s: &str) -> Result<[u32; 4], SimpleError> {
    let numbers: Vec<_> = s[1..s.len() - 1].split(", ").collect();
    if numbers.len() != 4 {
        return Err(SimpleError::new(format!("string does not split into 4 numbers: {s}")));
    }

    let numbers: Vec<_> = numbers.into_iter()
        .map(|n| n.parse::<u32>().map_err(SimpleError::from))
        .collect::<Result<_, _>>()?;

    Ok([numbers[0], numbers[1], numbers[2], numbers[3]])
}

pub fn solve(input: &str) -> Result<(usize, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample16.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(1), solve_part_1(SAMPLE_INPUT));
    }
}