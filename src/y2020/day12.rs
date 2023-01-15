//! Day 12: Rain Risk
//! https://adventofcode.com/2020/day/12

use std::error::Error;
use std::mem;
use std::str::FromStr;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl FromStr for Instruction {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: i32 = s[1..].parse()?;
        match s.chars().next() {
            Some('N') => Ok(Self::North(n)),
            Some('S') => Ok(Self::South(n)),
            Some('E') => Ok(Self::East(n)),
            Some('W') => Ok(Self::West(n)),
            Some('L') => Ok(Self::Left(n)),
            Some('R') => Ok(Self::Right(n)),
            Some('F') => Ok(Self::Forward(n)),
            _ => Err(SimpleError::new(format!("invalid instruction string: {s}")))
        }
    }
}

fn solve_part_1(input: &str) -> Result<i32, SimpleError> {
    let instructions = parse_input(input)?;

    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;

    for &instruction in &instructions {
        match instruction {
            Instruction::North(n) => {
                y += n;
            }
            Instruction::South(n) => {
                y -= n;
            }
            Instruction::East(n) => {
                x += n;
            }
            Instruction::West(n) => {
                x -= n;
            }
            Instruction::Forward(n) => {
                x += dx * n;
                y += dy * n;
            }
            Instruction::Left(n) | Instruction::Right(n) => {
                // R(n) is equivalent to L(360 - n)
                let n = if let Instruction::Right(_) = instruction {
                    360 - n
                } else {
                    n
                };
                match n {
                    90 => {
                        dy = -dy;
                        mem::swap(&mut dx, &mut dy);
                    }
                    180 => {
                        dx = -dx;
                        dy = -dy;
                    }
                    270 => {
                        dx = -dx;
                        mem::swap(&mut dx, &mut dy);
                    }
                    _ => return Err(SimpleError::new(format!("invalid rotation instruction: {instruction:?}")))
                }
            }
        }
    }

    Ok(x.abs() + y.abs())
}

fn solve_part_2(input: &str) -> Result<i32, SimpleError> {
    let instructions = parse_input(input)?;

    let mut x = 0;
    let mut y = 0;
    let mut dx = 10;
    let mut dy = 1;

    for &instruction in &instructions {
        match instruction {
            Instruction::North(n) => {
                dy += n;
            }
            Instruction::South(n) => {
                dy -= n;
            }
            Instruction::East(n) => {
                dx += n;
            }
            Instruction::West(n) => {
                dx -= n;
            }
            Instruction::Forward(n) => {
                x += n * dx;
                y += n * dy;
            }
            Instruction::Left(n) | Instruction::Right(n) => {
                // R(n) is equivalent to L(360 - n)
                let n = if let Instruction::Right(_) = instruction {
                    360 - n
                } else {
                    n
                };
                match n {
                    90 => {
                        dy = -dy;
                        mem::swap(&mut dx, &mut dy);
                    }
                    180 => {
                        dx = -dx;
                        dy = -dy;
                    }
                    270 => {
                        dx = -dx;
                        mem::swap(&mut dx, &mut dy);
                    }
                    _ => return Err(SimpleError::new(format!("invalid rotation instruction: {instruction:?}")))
                }
            }
        }
    }

    Ok(x.abs() + y.abs())
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, SimpleError> {
    input.lines().map(Instruction::from_str).collect()
}

pub fn solve(input: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample12.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(25), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(286), solve_part_2(SAMPLE_INPUT));
    }
}