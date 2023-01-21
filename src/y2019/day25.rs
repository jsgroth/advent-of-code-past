//! Day 25: Cryostasis
//! https://adventofcode.com/2019/day/25

use std::error::Error;
use std::{env, fs};
use std::path::Path;
use std::str::FromStr;
use crate::SimpleError;
use crate::y2019::intcode;
use crate::y2019::intcode::InteractiveIntcodeProgram;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn invert(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::North => "north",
            Self::South => "south",
            Self::East => "east",
            Self::West => "west",
        }
    }
}

impl FromStr for Direction {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "north" => Ok(Self::North),
            "south" => Ok(Self::South),
            "east" => Ok(Self::East),
            "west" => Ok(Self::West),
            _ => Err(SimpleError::new(format!("invalid direction string: {s}")))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RoomState {
    name: String,
    exits: Vec<Direction>,
    items: Vec<String>,
}

#[derive(Debug, Clone)]
struct PlayerState {
    program: InteractiveIntcodeProgram,
    inventory: Vec<String>,
    from_direction: Option<Direction>,
}

fn solve(input: &str) -> Result<(), Box<dyn Error>> {
    let program = intcode::parse_program(input)?;

    let program = InteractiveIntcodeProgram::new(program);

    let PlayerState { mut program, inventory, from_direction } =
        traverse_map(PlayerState { program, inventory: Vec::new(), from_direction: None })?;

    for item in &inventory {
        program.push_line_as_ascii(&format!("drop {item}"));
    }
    program.execute();
    program.fetch_outputs();

    // Go back out and in to figure out which direction the exit is in
    program.push_line_as_ascii(from_direction.unwrap().invert().to_str());
    program.execute();
    program.fetch_outputs();

    program.push_line_as_ascii(from_direction.unwrap().to_str());
    program.execute();

    let final_room_state = parse_room_state(&program.fetch_outputs())?;

    let checkpoint_direction = final_room_state.exits.iter().copied()
        .find(|&direction| direction != from_direction.unwrap().invert())
        .unwrap();

    for inventory_bits in 1..2_u32.pow(inventory.len() as u32) {
        let mut program = program.clone();

        for (i, item) in inventory.iter().enumerate() {
            let i = i as u32;
            if inventory_bits & (1 << i) != 0 {
                program.push_line_as_ascii(&format!("take {item}"));
            }
        }

        program.execute();
        program.fetch_outputs();

        program.push_line_as_ascii(checkpoint_direction.to_str());

        if program.execute() {
            print!("{}", ascii_to_string(&program.fetch_outputs()));
            return Ok(());
        }
    }

    Err(Box::new(SimpleError::new(String::from("no solution found"))))
}

const BLACKLISTED_ITEMS: [&str; 2] = ["infinite loop", "giant electromagnet"];

fn traverse_map(
    mut state: PlayerState,
) -> Result<PlayerState, SimpleError> {
    state.program.execute();

    let room_state = parse_room_state(&state.program.fetch_outputs())?;

    for item in &room_state.items {
        if !BLACKLISTED_ITEMS.contains(&item.as_str()) {
            let mut test_program = state.program.clone();
            test_program.push_line_as_ascii(&format!("take {item}"));
            if test_program.execute() {
                // Halted, don't take this item
                continue;
            }

            state.program.push_line_as_ascii(&format!("take {item}"));
            state.program.execute();
            state.program.fetch_outputs();

            state.inventory.push(item.clone());
        }
    }

    let from_direction = state.from_direction;
    let mut exit_direction: Option<Direction> = None;

    for &exit in &room_state.exits {
        if from_direction == Some(exit.invert()) {
            continue;
        }

        let mut program = state.program.clone();
        program.push_line_as_ascii(exit.to_str());

        if contains_exit(program.clone(), exit)? {
            exit_direction = Some(exit);
        } else {
            state = traverse_map(PlayerState { program, inventory: state.inventory, from_direction: Some(exit) })?;
            state.from_direction = from_direction;
        }
    }

    if room_state.name.as_str() == "Security Checkpoint" {
        return Ok(state);
    }

    if exit_direction.is_none() {
        match from_direction {
            Some(from_direction) => {
                state.program.push_line_as_ascii(from_direction.invert().to_str());
                state.program.execute();
                state.program.fetch_outputs();
                return Ok(state);
            }
            None => panic!("not on exit path and from direction is not set")
        }
    }

    state.program.push_line_as_ascii(exit_direction.unwrap().to_str());

    traverse_map(PlayerState { from_direction: exit_direction, ..state })
}

fn contains_exit(
    mut program: InteractiveIntcodeProgram,
    from_direction: Direction,
) -> Result<bool, SimpleError> {
    program.execute();

    let room_state = parse_room_state(&program.fetch_outputs())?;

    if room_state.name.as_str() == "Security Checkpoint" {
        return Ok(true);
    }

    for &exit in &room_state.exits {
        if exit == from_direction.invert() {
            continue;
        }

        let mut program = program.clone();
        program.push_line_as_ascii(exit.to_str());

        if contains_exit(program, exit)? {
            return Ok(true);
        }
    }

    Ok(false)
}

fn parse_room_state(output: &[i64]) -> Result<RoomState, SimpleError> {
    let room_string = ascii_to_string(output);
    let lines: Vec<_> = room_string.lines().collect();

    let mut name = String::new();
    let mut exits: Vec<Direction> = Vec::new();
    let mut items: Vec<String> = Vec::new();

    for line_group in lines.split(|s| s.is_empty()) {
        if line_group.is_empty() {
            continue;
        }

        let first_line = line_group[0];
        if first_line.starts_with("== ") {
            name = String::from(&first_line[3..first_line.len() - 3]);
        } else if first_line == "Doors here lead:" {
            exits = line_group[1..].iter().map(|line| {
                Direction::from_str(&line[2..])
            })
                .collect::<Result<_, _>>()?;
        } else if first_line == "Items here:" {
            items = line_group[1..].iter().map(|line| {
                String::from(&line[2..])
            })
                .collect();
        }
    }

    if name.is_empty() || exits.is_empty() {
        return Err(SimpleError::new(format!("output has no name and/or doors: {room_string}")));
    }

    Ok(RoomState { name, exits, items })
}

fn ascii_to_string(output: &[i64]) -> String {
    let mut s = String::new();
    for &c in output {
        s.push((c as u8) as char);
    }
    s
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    let program_name = args.next().unwrap();
    let year = args.next().unwrap();
    let day = args.next().unwrap();

    let input_filename = args.next().ok_or_else(
        || SimpleError::new(format!("USAGE: {program_name} {year} {day} <input_filename>"))
    )?;

    let input = fs::read_to_string(Path::new(&input_filename))?;

    solve(&input)
}