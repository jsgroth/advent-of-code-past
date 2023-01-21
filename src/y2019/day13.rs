//! Day 13: Care Package
//!
//! <https://adventofcode.com/2019/day/13>

use crate::y2019::intcode;
use crate::y2019::intcode::InteractiveIntcodeProgram;
use crate::SimpleError;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn from_int(n: i64) -> Result<Self, SimpleError> {
        let tile = match n {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => return Err(SimpleError::new(format!("invalid tile code: {n}"))),
        };
        Ok(tile)
    }
}

fn solve_part_1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut program = intcode::parse_program(input)?;

    let mut outputs = Vec::new();
    intcode::execute(
        &mut program,
        || panic!("input fn should not be called"),
        |output| outputs.push(output),
    );

    if outputs.is_empty() || outputs.len() % 3 != 0 {
        return Err(Box::new(SimpleError::new(format!(
            "expected outputs to be a positive multiple of 3, got {}",
            outputs.len()
        ))));
    }

    let block_tile_count = outputs
        .iter()
        .skip(2)
        .step_by(3)
        .filter(|&&n| n == 2)
        .count();

    Ok(block_tile_count)
}

fn solve_part_2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut program = intcode::parse_program(input)?;
    program[0] = 2;

    let mut program = InteractiveIntcodeProgram::new(program);

    let mut paddle_x = 0;
    let mut ball_x = 0;
    let mut score = 0;
    loop {
        let halted = program.execute();

        for chunk in program.fetch_outputs().chunks(3) {
            let x = chunk[0];
            let y = chunk[1];
            if x == -1 && y == 0 {
                score = chunk[2];
                continue;
            }

            let tile = Tile::from_int(chunk[2])?;

            match tile {
                Tile::Ball => ball_x = x,
                Tile::Paddle => paddle_x = x,
                _ => {}
            }
        }

        if halted {
            return Ok(score);
        }

        let next_input = (ball_x - paddle_x).signum();
        program.push_input(next_input);
    }
}

pub fn solve(input: &str) -> Result<(usize, i64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
