//! Day 2: Bathroom Security
//! https://adventofcode.com/2016/day/2

use std::error::Error;
use crate::SimpleError;

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Result<Self, SimpleError> {
        let direction = match c {
            'U' => Self::Up,
            'L' => Self::Left,
            'R' => Self::Right,
            'D' => Self::Down,
            _ => return Err(SimpleError::new(format!("invalid direction char: {c}")))
        };

        Ok(direction)
    }
}

const KEYPAD_1: [[char; 3]; 3] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
];

const KEYPAD_2: [[char; 5]; 5] = [
    [' ', ' ', '1', ' ', ' '],
    [' ', '2', '3', '4', ' '],
    ['5', '6', '7', '8', '9'],
    [' ', 'A', 'B', 'C', ' '],
    [' ', ' ', 'D', ' ', ' '],
];

fn solve_part_1(input: &str) -> Result<String, SimpleError> {
    let instructions = parse_input(input)?;

    let mut i = 1;
    let mut j = 1;
    let mut s = String::new();
    for instruction in &instructions {
        for direction in instruction {
            match direction {
                Direction::Up => {
                    if i > 0 {
                        i -= 1;
                    }
                }
                Direction::Left => {
                    if j > 0 {
                        j -= 1;
                    }
                }
                Direction::Right => {
                    if j < 2 {
                        j += 1;
                    }
                }
                Direction::Down => {
                    if i < 2 {
                        i += 1;
                    }
                }
            }
        }

        s.push(KEYPAD_1[i][j]);
    }

    Ok(s)
}

fn solve_part_2(input: &str) -> Result<String, SimpleError> {
    let instructions = parse_input(input)?;

    let mut i = 2;
    let mut j = 0;
    let mut s = String::new();
    for instruction in &instructions {
        for direction in instruction {
            match direction {
                Direction::Up => {
                    if i > 0 && KEYPAD_2[i - 1][j] != ' ' {
                        i -= 1;
                    }
                }
                Direction::Left => {
                    if j > 0 && KEYPAD_2[i][j - 1] != ' ' {
                        j -= 1;
                    }
                }
                Direction::Right => {
                    if j < 4 && KEYPAD_2[i][j + 1] != ' ' {
                        j += 1;
                    }
                }
                Direction::Down => {
                    if i < 4 && KEYPAD_2[i + 1][j] != ' ' {
                        i += 1;
                    }
                }
            }
        }

        s.push(KEYPAD_2[i][j]);
    }

    Ok(s)
}

fn parse_input(input: &str) -> Result<Vec<Vec<Direction>>, SimpleError> {
    input.lines().map(|line| {
        line.chars().map(Direction::from_char).collect()
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(String::from("1985")), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(String::from("5DB3")), solve_part_2(SAMPLE_INPUT));
    }
}