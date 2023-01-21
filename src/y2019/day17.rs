//! Day 17: Set and Forget
//! https://adventofcode.com/2019/day/17

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};
use crate::SimpleError;
use crate::y2019::intcode;
use crate::y2019::intcode::InteractiveIntcodeProgram;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    i: i32,
    j: i32,
}

impl Point {
    fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Self::new(self.i - 1, self.j),
            Direction::South => Self::new(self.i + 1, self.j),
            Direction::East => Self::new(self.i, self.j + 1),
            Direction::West => Self::new(self.i, self.j - 1),
        }
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotated_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    fn rotated_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Robot {
    position: Point,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RobotCommand {
    RotateLeft,
    RotateRight,
    Move(usize),
}

impl Display for RobotCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::RotateLeft => write!(f, "L"),
            Self::RotateRight => write!(f, "R"),
            Self::Move(steps) => write!(f, "{steps}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RobotFunction {
    commands: Vec<RobotCommand>,
}

impl RobotFunction {
    fn new() -> Self {
        Self { commands: Vec::new() }
    }
}

impl Display for RobotFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let command_strs: Vec<_> = self.commands.iter()
            .map(RobotCommand::to_string)
            .collect();
        write!(f, "{}", command_strs.join(","))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RobotProgram {
    main_routine: Vec<char>,
    function_a: RobotFunction,
    function_b: RobotFunction,
    function_c: RobotFunction,
}

fn solve_part_1(input: &str) -> Result<usize, Box<dyn Error>> {
    let program = intcode::parse_program(input)?;

    let (map, _) = build_map_from_program(program)?;

    let mut alignment_sum = 0;
    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            if map[i][j] && map[i - 1][j] && map[i + 1][j] && map[i][j - 1] && map[i][j + 1] {
                alignment_sum += i * j;
            }
        }
    }

    Ok(alignment_sum)
}

fn solve_part_2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut program = intcode::parse_program(input)?;

    let (map, robot) = build_map_from_program(program.clone())?;

    let movement_program = create_movement_program(&map, robot)?;

    program[0] = 2;

    let mut intcode_program = InteractiveIntcodeProgram::new(program);

    intcode_program.push_line_as_ascii(&join_chars(&movement_program.main_routine));
    intcode_program.push_line_as_ascii(&movement_program.function_a.to_string());
    intcode_program.push_line_as_ascii(&movement_program.function_b.to_string());
    intcode_program.push_line_as_ascii(&movement_program.function_c.to_string());

    intcode_program.push_line_as_ascii("n");

    intcode_program.execute();

    Ok(intcode_program.fetch_outputs().last().copied().unwrap())
}

fn join_chars(chars: &[char]) -> String {
    let strings: Vec<_> = chars.iter().map(char::to_string).collect();
    strings.join(",")
}

fn create_movement_program(map: &Vec<Vec<bool>>, robot: Robot) -> Result<RobotProgram, SimpleError> {
    let full_path = find_full_path(map, robot);

    let program = search_for_movement_program(&full_path, Vec::new(), Vec::new());

    program.ok_or_else(|| SimpleError::new(String::from("no solution found")))
}

fn search_for_movement_program(path: &[RobotCommand], existing_functions: Vec<RobotFunction>, main_routine: Vec<usize>) -> Option<RobotProgram> {
    if path.is_empty() {
        let function_a = existing_functions.get(0).cloned().unwrap_or_else(RobotFunction::new);
        let function_b = existing_functions.get(1).cloned().unwrap_or_else(RobotFunction::new);
        let function_c = existing_functions.get(2).cloned().unwrap_or_else(RobotFunction::new);

        let main_routine: Vec<_> = main_routine.into_iter()
            .map(|i| {
                ((i as u8) + b'A') as char
            })
            .collect();

        return Some(RobotProgram { main_routine, function_a, function_b, function_c })
    }

    if main_routine.len() == 10 {
        // Can't fit another function call into the main routine, n function calls take (2n - 1) chars
        return None;
    }

    for (i, existing_function) in existing_functions.iter().enumerate() {
        if is_prefix(path, &existing_function.commands) {
            let new_path = remove_prefix(path, &existing_function.commands);
            let mut new_main_routine = main_routine;
            new_main_routine.push(i);

            return search_for_movement_program(&new_path, existing_functions.clone(), new_main_routine);
        }
    }

    if existing_functions.len() == 3 {
        // No room to create another function
        return None;
    }

    let mut new_function = RobotFunction::new();
    let mut i = 0;
    while i < path.len() && new_function.to_string().len() <= 20 {
        new_function.commands.push(path[i]);
        i += 1;
    }

    while !new_function.commands.is_empty() {
        if new_function.to_string().len() <= 20 {
            let new_path = remove_prefix(path, &new_function.commands);

            let mut new_existing_functions = existing_functions.clone();
            new_existing_functions.push(new_function.clone());

            let mut new_main_routine = main_routine.clone();
            new_main_routine.push(new_existing_functions.len() - 1);

            if let Some(movement_program) = search_for_movement_program(&new_path, new_existing_functions, new_main_routine) {
                return Some(movement_program);
            }
        }

        let last_command = new_function.commands.pop().unwrap();
        if let RobotCommand::Move(steps) = last_command {
            if steps > 1 {
                new_function.commands.push(RobotCommand::Move(steps - 1));
            }
        }
    }

    None
}

fn is_prefix(path: &[RobotCommand], commands: &[RobotCommand]) -> bool {
    if commands.len() > path.len() {
        return false;
    }

    for (i, (path_command, prefix_command)) in path.iter().copied().zip(commands.iter().copied()).enumerate() {
        if path_command != prefix_command {
            return if i != commands.len() - 1 {
                false
            } else {
                match (path_command, prefix_command) {
                    (RobotCommand::Move(path_steps), RobotCommand::Move(command_steps)) => command_steps <= path_steps,
                    _ => false
                }
            }
        }
    }

    true
}

fn remove_prefix(path: &[RobotCommand], commands: &[RobotCommand]) -> Vec<RobotCommand> {
    if commands[commands.len() - 1] == path[commands.len() - 1] {
        return Vec::from(&path[commands.len()..]);
    }

    let mut new_path = Vec::new();
    match (path[commands.len() - 1], commands[commands.len() - 1]) {
        (RobotCommand::Move(path_steps), RobotCommand::Move(command_steps)) => {
            new_path.push(RobotCommand::Move(path_steps - command_steps));
        }
        _ => panic!("last elements should both be Move if not equal: {:?} != {:?}", path[commands.len() - 1], commands[commands.len() - 1])
    }

    new_path.extend_from_slice(&path[commands.len()..]);

    new_path
}

fn find_full_path(map: &Vec<Vec<bool>>, mut robot: Robot) -> Vec<RobotCommand> {
    let mut commands = Vec::new();

    let mut move_steps = 0;
    loop {
        if !is_scaffold(map, robot.position + robot.direction) {
            if move_steps > 0 {
                commands.push(RobotCommand::Move(move_steps));
                move_steps = 0;
            }

            match find_turn_command(map, robot) {
                Some(command) => {
                    match command {
                        RobotCommand::RotateLeft => {
                            robot.direction = robot.direction.rotated_left();
                        }
                        RobotCommand::RotateRight => {
                            robot.direction = robot.direction.rotated_right();
                        }
                        _ => panic!("find_turn_command returned a non-rotate command: {command:?}")
                    }

                    commands.push(command);
                }
                None => {
                    break;
                }
            }
        }

        robot.position += robot.direction;
        move_steps += 1;
    }

    commands
}

fn find_turn_command(map: &Vec<Vec<bool>>, robot: Robot) -> Option<RobotCommand> {
    if is_scaffold(map, robot.position + robot.direction.rotated_left()) {
        Some(RobotCommand::RotateLeft)
    } else if is_scaffold(map, robot.position + robot.direction.rotated_right()) {
        Some(RobotCommand::RotateRight)
    } else {
        None
    }
}

fn is_scaffold(map: &Vec<Vec<bool>>, p: Point) -> bool {
    let i = p.i;
    let j = p.j;
    i >= 0 && j >= 0 && i < map.len() as i32 && j < map[0].len() as i32 && map[i as usize][j as usize]
}

fn build_map_from_program(mut program: Vec<i64>) -> Result<(Vec<Vec<bool>>, Robot), SimpleError> {
    let mut outputs = Vec::new();
    intcode::execute(
        &mut program,
        || panic!("input fn should not be called"),
        |output| outputs.push(output),
    );

    let mut outputs_as_string = String::new();
    for &ascii_code in &outputs {
        if ascii_code < 0 {
            return Err(SimpleError::new(format!("invalid negative ASCII code output by program: {ascii_code}")));
        }

        let c = char::from_u32(ascii_code as u32).ok_or_else(
            || SimpleError::new(format!("invalid ASCII code output by program: {ascii_code}"))
        )?;

        outputs_as_string.push(c);
    }

    build_map(&outputs_as_string)
}

fn build_map(outputs_as_string: &str) -> Result<(Vec<Vec<bool>>, Robot), SimpleError> {
    let mut robot: Option<Robot> = None;
    let mut map = Vec::new();
    let mut current_row = Vec::new();
    for c in outputs_as_string.chars() {
        match c {
            '#' => current_row.push(true),
            '.' => current_row.push(false),
            '^' | '>' | 'v' | '<' => {
                current_row.push(true);

                let robot_position = Point::new(map.len() as i32, (current_row.len() - 1) as i32);
                let robot_direction = match c {
                    '^' => Direction::North,
                    '>' => Direction::East,
                    'v' => Direction::South,
                    '<' => Direction::West,
                    _ => panic!("match statements do not match, unexpected char: {c}")
                };

                robot = Some(Robot { position: robot_position, direction: robot_direction })
            },
            '\n' => {
                if !current_row.is_empty() {
                    map.push(current_row);
                    current_row = Vec::new();
                }
            }
            _ => return Err(SimpleError::new(format!("unexpected char in program output: {c}")))
        }
    }

    if robot.is_none() {
        return Err(SimpleError::new(String::from("program did not output a robot location")));
    }

    Ok((map, robot.unwrap()))
}

pub fn solve(input: &str) -> Result<(usize, i64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample17.txt");

    #[test]
    fn test_sample_input_part_2() {
        let (map, robot) = build_map(SAMPLE_INPUT).unwrap();

        assert!(create_movement_program(&map, robot).is_ok());
    }
}