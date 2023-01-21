//! Day 21: Scrambled Letters and Hash
//!
//! <https://adventofcode.com/2016/day/21>

use crate::SimpleError;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotatePositionOf(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Instruction {
    fn from_line(line: &str) -> Result<Self, SimpleError> {
        let split: Vec<_> = line.split(' ').collect();
        match split.as_slice() {
            ["swap", "position", x, "with", "position", y] => {
                Ok(Self::SwapPositions(x.parse()?, y.parse()?))
            }
            ["swap", "letter", x, "with", "letter", y] => {
                Ok(Self::SwapLetters(first_letter(x)?, first_letter(y)?))
            }
            ["rotate", "left", x, _] => Ok(Self::RotateLeft(x.parse()?)),
            ["rotate", "right", x, _] => Ok(Self::RotateRight(x.parse()?)),
            ["rotate", "based", "on", "position", "of", "letter", x] => {
                Ok(Self::RotatePositionOf(first_letter(x)?))
            }
            ["reverse", "positions", x, "through", y] => Ok(Self::Reverse(x.parse()?, y.parse()?)),
            ["move", "position", x, "to", "position", y] => Ok(Self::Move(x.parse()?, y.parse()?)),
            _ => Err(SimpleError::new(format!("invalid line: {line}"))),
        }
    }

    fn execute(&self, password: &mut Vec<char>) -> Result<(), SimpleError> {
        match *self {
            Self::SwapPositions(x, y) => {
                password.swap(x, y);
            }
            Self::SwapLetters(x, y) => {
                let x = find_letter(password, x)?;
                let y = find_letter(password, y)?;
                password.swap(x, y);
            }
            Self::RotateLeft(x) => {
                *password = rotate_left(password, x);
            }
            Self::RotateRight(x) => {
                *password = rotate_right(password, x);
            }
            Self::RotatePositionOf(x) => {
                let x = find_letter(password, x)?;
                let rotate_right_x = 1 + x + usize::from(x >= 4);
                *password = rotate_right(password, rotate_right_x);
            }
            Self::Reverse(x, y) => {
                *password = password
                    .iter()
                    .copied()
                    .take(x)
                    .chain(password.iter().copied().skip(x).take(y - x + 1).rev())
                    .chain(password.iter().copied().skip(y + 1))
                    .collect();
            }
            Self::Move(x, y) => {
                let c = password.remove(x);
                password.insert(y, c);
            }
        }

        Ok(())
    }

    fn reverse(&self, password: &mut Vec<char>) -> Result<(), SimpleError> {
        match *self {
            Self::SwapPositions(..) | Self::SwapLetters(..) | Self::Reverse(..) => {
                self.execute(password)?;
            }
            Self::RotateLeft(x) => {
                *password = rotate_right(password, x);
            }
            Self::RotateRight(x) => {
                *password = rotate_left(password, x);
            }
            Self::RotatePositionOf(x) => {
                let rotated_pos = find_letter(password, x)?;
                let original_pos = if rotated_pos % 2 == 1 {
                    rotated_pos / 2
                } else {
                    let shifted_rotated_pos = (rotated_pos + password.len() - 1) % password.len();
                    password.len() / 2 + shifted_rotated_pos / 2
                };

                *password = rotate_right(
                    password,
                    (original_pos + password.len() - rotated_pos) % password.len(),
                );
            }
            Self::Move(x, y) => {
                let c = password.remove(y);
                password.insert(x, c);
            }
        }

        Ok(())
    }
}

fn first_letter(s: &str) -> Result<char, SimpleError> {
    s.chars()
        .next()
        .ok_or_else(|| SimpleError::new(String::from("unexpected empty string")))
}

fn solve_part_1(input: &str, starting_password: &str) -> Result<String, SimpleError> {
    let instructions = parse_input(input)?;

    let mut current_password: Vec<_> = starting_password.chars().collect();
    for &instruction in &instructions {
        instruction.execute(&mut current_password)?;
    }

    Ok(current_password.into_iter().collect())
}

fn solve_part_2(input: &str) -> Result<String, SimpleError> {
    let instructions = parse_input(input)?;

    let mut current_password: Vec<_> = "fbgdceah".chars().collect();
    for &instruction in instructions.iter().rev() {
        instruction.reverse(&mut current_password)?;
    }

    Ok(current_password.into_iter().collect())
}

fn rotate_left(password: &Vec<char>, n: usize) -> Vec<char> {
    rotate_right(password, password.len() - (n % password.len()))
}

fn rotate_right(password: &Vec<char>, n: usize) -> Vec<char> {
    let n = n % password.len();

    password
        .iter()
        .copied()
        .skip(password.len() - n)
        .take(n)
        .chain(password.iter().copied().take(password.len() - n))
        .collect()
}

fn find_letter(password: &Vec<char>, letter: char) -> Result<usize, SimpleError> {
    password.iter().position(|&c| c == letter).ok_or_else(|| {
        SimpleError::new(format!(
            "password does not contain letter '{letter}': {password:?}"
        ))
    })
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, SimpleError> {
    input.lines().map(Instruction::from_line).collect()
}

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input, "abcdefgh")?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample21.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(
            Ok(String::from("decab")),
            solve_part_1(SAMPLE_INPUT, "abcde")
        );
    }
}
