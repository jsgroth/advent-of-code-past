//! Day 8: Two-Factor Authentication
//! https://adventofcode.com/2016/day/8

use std::error::Error;
use crate::SimpleError;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Rect(usize, usize),
    RotateColumn(usize, usize),
    RotateRow(usize, usize),
}

impl Instruction {
    fn from_line(line: &str) -> Result<Self, SimpleError> {
        let split: Vec<_> = line.split(' ').collect();
        let instruction = match split.as_slice() {
            ["rect", dimensions] => {
                let (width, height) = match dimensions.split_once('x') {
                    Some((width, height)) => (width, height),
                    None => return Err(SimpleError::new(format!("invalid rect instruction: {line}"))),
                };
                let width: usize = width.parse()?;
                let height: usize = height.parse()?;

                Self::Rect(width, height)
            }
            ["rotate", "column", x, "by", n] => {
                let x = match x.split_once('=') {
                    Some((_, x)) => x,
                    None => return Err(SimpleError::new(format!("invalid rotate column instruction: {line}"))),
                };
                let x: usize = x.parse()?;
                let n: usize = n.parse()?;

                Self::RotateColumn(x, n)
            }
            ["rotate", "row", y, "by", n] => {
                let y = match y.split_once('=') {
                    Some((_, y)) => y,
                    None => return Err(SimpleError::new(format!("invalid rotate row instruction: {line}"))),
                };
                let y: usize = y.parse()?;
                let n: usize = n.parse()?;

                Self::RotateRow(y, n)
            }
            _ => return Err(SimpleError::new(format!("unrecognized instruction: {line}")))
        };

        Ok(instruction)
    }
}

fn solve_both_parts(input: &str) -> Result<(usize, String), SimpleError> {
    let instructions = parse_input(input)?;

    let mut screen = vec![vec![false; 50]; 6];
    for &instruction in &instructions {
        match instruction {
            Instruction::Rect(width, height) => {
                for row in &mut screen[0..height] {
                    for value in &mut row[0..width] {
                        *value = true;
                    }
                }
            }
            Instruction::RotateRow(y, n) => {
                screen[y] = shift(screen[y].clone(), n);
            }
            Instruction::RotateColumn(x, n) => {
                let shifted_col = shift(col_vec(&screen, x), n);
                for (i, b) in shifted_col.into_iter().enumerate() {
                    screen[i][x] = b;
                }
            }
        }
    }

    let lit_count = screen.iter().map(|row| {
        row.iter().filter(|&&b| b).count()
    })
        .sum();

    let printed_screen = print_screen(&screen);

    Ok((lit_count, printed_screen))
}

fn col_vec(screen: &Vec<Vec<bool>>, x: usize) -> Vec<bool> {
    let mut col = vec![false; screen.len()];
    for i in 0..screen.len() {
        col[i] = screen[i][x];
    }
    col
}

fn shift(v: Vec<bool>, n: usize) -> Vec<bool> {
    let mut shifted = vec![false; v.len()];
    for (i, b) in v.into_iter().enumerate() {
        let new_i = (i + n) % shifted.len();
        shifted[new_i] = b;
    }
    shifted
}

fn print_screen(screen: &[Vec<bool>]) -> String {
    let mut s = String::new();

    for (i, row) in screen.iter().enumerate() {
        if i > 0 {
            s.push('\n');
        }

        for &b in row {
            if b {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
    }

    s
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, SimpleError> {
    input.lines().map(Instruction::from_line).collect()
}

pub fn solve(input: &str) -> Result<(usize, String), Box<dyn Error>> {
    let (solution1, solution2) = solve_both_parts(input)?;

    Ok((solution1, solution2))
}