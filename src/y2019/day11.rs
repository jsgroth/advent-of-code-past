//! Day 11: Space Police
//! https://adventofcode.com/2019/day/11

use crate::y2019::intcode;
use crate::y2019::intcode::InteractiveIntcodeProgram;
use crate::SimpleError;
use std::collections::HashSet;
use std::error::Error;
use std::{cmp, mem};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn solve_part_1(input: &str) -> Result<usize, Box<dyn Error>> {
    let program = intcode::parse_program(input)?;

    let program = InteractiveIntcodeProgram::new(program);
    let (painted_points, _) = run_robot_program(program, false)?;

    Ok(painted_points.len())
}

fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let program = intcode::parse_program(input)?;

    let program = InteractiveIntcodeProgram::new(program);
    let (_, white_points) = run_robot_program(program, true)?;

    let (min_x, max_x, min_y, max_y) = white_points.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y), &point| {
            (
                cmp::min(min_x, point.x),
                cmp::max(max_x, point.x),
                cmp::min(min_y, point.y),
                cmp::max(max_y, point.y),
            )
        },
    );

    let mut s = String::new();
    for y in (min_y..=max_y).rev() {
        if y < max_y {
            s.push('\n');
        }

        for x in min_x..=max_x {
            if white_points.contains(&Point::new(x, y)) {
                s.push('█');
            } else {
                s.push(' ');
            }
        }
    }

    Ok(s)
}

fn run_robot_program(
    mut program: InteractiveIntcodeProgram,
    start_on_white: bool,
) -> Result<(HashSet<Point>, HashSet<Point>), Box<dyn Error>> {
    let mut painted_points = HashSet::new();
    let mut white_points = HashSet::new();

    let mut position = Point::new(0, 0);
    let mut dx = 0;
    let mut dy = 1;
    let mut halted = false;

    if start_on_white {
        white_points.insert(position);
    }

    while !halted {
        if white_points.contains(&position) {
            program.push_input(1);
        } else {
            program.push_input(0);
        }

        halted = program.execute();

        let outputs = program.fetch_outputs();
        if outputs.len() != 2 {
            return Err(Box::new(SimpleError::new(format!(
                "expected 2 outputs, got: {outputs:?}"
            ))));
        }

        if outputs[0] == 1 {
            white_points.insert(position);
        } else {
            white_points.remove(&position);
        }
        painted_points.insert(position);

        if outputs[1] == 1 {
            // Turn right
            dx = -dx;
            mem::swap(&mut dx, &mut dy);
        } else {
            // Turn left
            dy = -dy;
            mem::swap(&mut dx, &mut dy);
        }

        position = Point::new(position.x + dx, position.y + dy);
    }

    Ok((painted_points, white_points))
}

pub fn solve(input: &str) -> Result<(usize, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
