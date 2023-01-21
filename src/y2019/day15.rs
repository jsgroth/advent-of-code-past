//! Day 15: Oxygen System
//! https://adventofcode.com/2019/day/15

use crate::y2019::intcode;
use crate::y2019::intcode::InteractiveIntcodeProgram;
use crate::SimpleError;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::ops::Add;
use std::{cmp, iter};

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

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Self::new(self.x, self.y + 1),
            Direction::South => Self::new(self.x, self.y - 1),
            Direction::West => Self::new(self.x - 1, self.y),
            Direction::East => Self::new(self.x + 1, self.y),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    const ALL: [Self; 4] = [Self::North, Self::South, Self::West, Self::East];

    fn to_intcode_command(self) -> i64 {
        match self {
            Self::North => 1,
            Self::South => 2,
            Self::West => 3,
            Self::East => 4,
        }
    }
}

fn solve_both_parts(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let program = intcode::parse_program(input)?;

    let (program_at_oxygen_system, steps_to_oxygen_system) = find_oxygen_system(program)?;

    let mut visited: HashSet<_> = iter::once(Point::new(0, 0)).collect();

    let mut queue = VecDeque::new();
    queue.push_back((program_at_oxygen_system, Point::new(0, 0), 0));

    let mut max_steps_from_oxygen_system = 0;
    while let Some((program, position, steps)) = queue.pop_front() {
        max_steps_from_oxygen_system = cmp::max(max_steps_from_oxygen_system, steps);

        for direction in Direction::ALL {
            let new_position = position + direction;
            if !visited.contains(&new_position) {
                visited.insert(new_position);

                let mut program = program.clone();
                program.push_input(direction.to_intcode_command());

                program.execute();

                let outputs = program.fetch_outputs();

                match outputs[0] {
                    0 | 2 => {}
                    1 => {
                        queue.push_back((program, new_position, steps + 1));
                    }
                    _ => {
                        return Err(Box::new(SimpleError::new(format!(
                            "unexpected program output: {}",
                            outputs[0]
                        ))))
                    }
                }
            }
        }
    }

    Ok((steps_to_oxygen_system, max_steps_from_oxygen_system))
}

fn find_oxygen_system(
    program: Vec<i64>,
) -> Result<(InteractiveIntcodeProgram, usize), Box<dyn Error>> {
    let mut visited: HashSet<_> = iter::once(Point::new(0, 0)).collect();

    let mut queue = VecDeque::new();
    queue.push_back((InteractiveIntcodeProgram::new(program), Point::new(0, 0), 0));

    while let Some((program, position, steps)) = queue.pop_front() {
        for direction in Direction::ALL {
            let new_position = position + direction;
            if !visited.contains(&new_position) {
                visited.insert(new_position);

                let mut program = program.clone();
                program.push_input(direction.to_intcode_command());

                program.execute();

                let outputs = program.fetch_outputs();
                if outputs.len() != 1 {
                    return Err(Box::new(SimpleError::new(format!(
                        "expected 1 output from program, got {}",
                        outputs.len()
                    ))));
                }

                match outputs[0] {
                    0 => {}
                    1 => {
                        queue.push_back((program, new_position, steps + 1));
                    }
                    2 => {
                        return Ok((program, steps + 1));
                    }
                    _ => {
                        return Err(Box::new(SimpleError::new(format!(
                            "unexpected program output: {}",
                            outputs[0]
                        ))))
                    }
                }
            }
        }
    }

    Err(Box::new(SimpleError::new(String::from(
        "no solution found",
    ))))
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let (solution1, solution2) = solve_both_parts(input)?;

    Ok((solution1, solution2))
}
