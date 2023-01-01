//! Day 19: A Series of Tubes
//! https://adventofcode.com/2017/day/19

use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Void,
    HorizontalLine,
    VerticalLine,
    Turn,
    Letter(char),
}

impl Space {
    fn from_char(c: char) -> Result<Self, SimpleError> {
        match c {
            ' ' => Ok(Self::Void),
            '-' => Ok(Self::HorizontalLine),
            '|' => Ok(Self::VerticalLine),
            '+' => Ok(Self::Turn),
            c @ 'A'..='Z' => Ok(Self::Letter(c)),
            _ => Err(SimpleError::new(format!("invalid char: {c}")))
        }
    }
}

fn solve_both_parts(input: &str) -> Result<(String, usize), SimpleError> {
    let grid = parse_input(input)?;

    let start = match grid[0].iter().position(|&space| space == Space::VerticalLine) {
        Some(start) => start,
        None => return Err(SimpleError::new(String::from("first row does not contain a '|'"))),
    };

    let mut i = 0;
    let mut j = start;
    let mut di = 1;
    let mut dj = 0;
    let mut s = String::new();
    let mut steps = 1;
    loop {
        i = ((i as i32) + di) as usize;
        j = ((j as i32) + dj) as usize;
        steps += 1;

        match grid[i][j] {
            Space::Void => {
                return Ok((s, steps - 1));
            }
            Space::HorizontalLine | Space::VerticalLine => {},
            Space::Letter(c) => {
                s.push(c);
            }
            Space::Turn => {
                match try_turn(&grid, i, j, di, dj) {
                    Some((new_di, new_dj)) => {
                        di = new_di;
                        dj = new_dj;
                    }
                    None => return Err(SimpleError::new(format!("unable to turn at i={i}, j={j}, di={di}, dj={dj}"))),
                }
            }
        }
    }
}

fn try_turn(grid: &Vec<Vec<Space>>, i: usize, j: usize, di: i32, dj: i32) -> Option<(i32, i32)> {
    for (di_sign, dj_sign) in [(-1, 1), (1, -1)] {
        let new_di = dj * dj_sign;
        let new_dj = di * di_sign;
        let new_i = (i as i32) + new_di;
        let new_j = (j as i32) + new_dj;
        if new_i < 0 || new_j < 0 || new_i >= grid.len() as i32 || new_j >= grid[new_i as usize].len() as i32 {
            continue;
        }

        let new_i = new_i as usize;
        let new_j = new_j as usize;
        match grid[new_i][new_j] {
            Space::Void => {},
            Space::Letter(_) | Space::Turn => {
                return Some((new_di, new_dj));
            }
            Space::HorizontalLine => {
                if new_dj != 0 {
                    return Some((new_di, new_dj));
                }
            }
            Space::VerticalLine => {
                if new_di != 0 {
                    return Some((new_di, new_dj));
                }
            }
        }
    }

    None
}

fn parse_input(input: &str) -> Result<Vec<Vec<Space>>, SimpleError> {
    let grid: Result<Vec<Vec<_>>, _> = input.lines().map(|line| {
        line.chars().map(Space::from_char).collect()
    })
        .collect();
    let grid = grid?;

    if grid.is_empty() {
        return Err(SimpleError::new(String::from("input is empty")));
    }

    Ok(grid)
}

pub fn solve(input: &str) -> Result<(String, usize), Box<dyn Error>> {
    let (solution1, solution2) = solve_both_parts(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample19.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok((String::from("ABCDEF"), 38)), solve_both_parts(SAMPLE_INPUT));
    }
}