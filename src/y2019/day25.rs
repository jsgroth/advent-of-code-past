//! Day 25: Cryostasis
//! https://adventofcode.com/2019/day/25

use std::error::Error;
use std::{env, fs, io};
use std::path::Path;
use crate::SimpleError;
use crate::y2019::intcode;
use crate::y2019::intcode::InteractiveIntcodeProgram;

fn solve(input: &str) -> Result<(), Box<dyn Error>> {
    let program = intcode::parse_program(input)?;

    let mut program = InteractiveIntcodeProgram::new(program);

    loop {
        let halted = program.execute();

        for output in program.fetch_outputs() {
            if let Ok(_) = <i64 as TryInto<u8>>::try_into(output) {
                print!("{}", (output as u8) as char);
            } else {
                println!("{output}");
            }
        }

        if halted {
            break;
        }

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        program.push_line_as_ascii(line.trim_end());
    }

    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    let program_name = args.next().unwrap();
    let year = args.next().unwrap();
    let day = args.next().unwrap();

    let input_filename = args.next().ok_or(
        SimpleError::new(format!("USAGE: {program_name} {year} {day} <input_filename>"))
    )?;

    let input = fs::read_to_string(Path::new(&input_filename))?;

    solve(&input)
}