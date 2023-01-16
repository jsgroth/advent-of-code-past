//! Day 14: Docking Data
//! https://adventofcode.com/2020/day/14

use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    MaskSet(String),
    MemSet { address: u64, value: u64 },
}

impl FromStr for Instruction {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split(' ').collect();
        if split.len() != 3 || split[1] != "=" {
            return Err(SimpleError::new(format!("invalid instruction format: {s}")));
        }

        if split[0] == "mask" {
            Ok(Self::MaskSet(String::from(split[2])))
        } else if split[0].starts_with("mem[") {
            let address = split[0];
            let address = address[4..address.len() - 1].parse()?;

            let value = split[2].parse()?;

            Ok(Self::MemSet { address, value })
        } else {
            Err(SimpleError::new(format!("invalid instruction format: {s}")))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct BitMask {
    zeroes: u64,
    ones: u64,
    x_indices: Vec<u64>,
}

impl BitMask {
    fn new() -> Self {
        Self {
            zeroes: !0,
            ones: 0,
            x_indices: Vec::new(),
        }
    }

    fn mask_value(&self, value: u64) -> u64 {
        (value & self.zeroes) | self.ones
    }

    fn mask_address(&self, address: u64) -> Vec<u64> {
        let initial_address = address | self.ones;
        self.x_indices.iter().copied()
            .fold(vec![initial_address], |addresses, x_index| {
                let mut next_addresses = Vec::with_capacity(2 * addresses.len());
                for address in addresses {
                    next_addresses.push(address & !(1 << x_index));
                    next_addresses.push(address | (1 << x_index));
                }
                next_addresses
            })
    }
}

impl FromStr for BitMask {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 64 {
            return Err(SimpleError::new(format!("bit mask string is too long: {s}")));
        }

        let mut bit_mask = Self::new();
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                'X' => {
                    bit_mask.x_indices.push(i as u64);
                },
                '0' => {
                    bit_mask.zeroes &= !(1 << i);
                }
                '1' => {
                    bit_mask.ones |= 1 << i;
                }
                _ => return Err(SimpleError::new(format!("invalid bit mask char: {c}")))
            }
        }

        Ok(bit_mask)
    }
}

fn solve_part_1(input: &str) -> Result<u64, SimpleError> {
    let instructions = parse_input(input)?;

    let mut bit_mask = BitMask::new();
    let mut memory = HashMap::new();

    for instruction in &instructions {
        match instruction {
            Instruction::MaskSet(bit_mask_str) => {
                bit_mask = bit_mask_str.parse()?;
            }
            &Instruction::MemSet { address, value } => {
                memory.insert(address, bit_mask.mask_value(value));
            }
        }
    }

    Ok(memory.values().sum())
}

fn solve_part_2(input: &str) -> Result<u64, SimpleError> {
    let instructions = parse_input(input)?;

    let mut bit_mask = BitMask::new();
    let mut memory = HashMap::new();

    for instruction in &instructions {
        match instruction {
            Instruction::MaskSet(bit_mask_str) => {
                bit_mask = bit_mask_str.parse()?;
            }
            &Instruction::MemSet { address, value } => {
                for masked_address in bit_mask.mask_address(address) {
                    memory.insert(masked_address, value);
                }
            }
        }
    }

    Ok(memory.values().sum())
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, SimpleError> {
    input.lines().map(Instruction::from_str).collect()
}

pub fn solve(input: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample14.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample14-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(165), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(208), solve_part_2(SAMPLE_INPUT_2));
    }
}