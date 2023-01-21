//! Day 21: Springdroid Adventure
//! https://adventofcode.com/2019/day/21

use std::error::Error;
use crate::SimpleError;
use crate::y2019::intcode;
use crate::y2019::intcode::InteractiveIntcodeProgram;

// J = !A || (!B && D) || (!C && D)
// When a hole is within 3 spaces, jump as early as possible as long as the robot will land on
// ground
const PART_1_INSTRUCTIONS: &str = "\
NOT A J

NOT B T
AND D T
OR T J

NOT C T
AND D T
OR T J

WALK
";

// J = (!A || (!B && D) || (!C && D)) && (E || H)
// Same as part 1 but only jump if there is ground either 5 or 8 spaces ahead
const PART_2_INSTRUCTIONS: &str = "\
NOT A J

NOT B T
AND D T
OR T J

NOT C T
AND D T
OR T J

NOT E T
NOT T T
OR H T
AND T J

RUN
";

fn solve_part(input: &str, instructions: &str) -> Result<i64, Box<dyn Error>> {
    let program = intcode::parse_program(input)?;
    let mut program = InteractiveIntcodeProgram::new(program);

    let instructions: Vec<_> = instructions.lines().filter(|s| !s.is_empty()).collect();

    for line in instructions {
        program.push_line_as_ascii(line);
    }

    program.execute();

    let outputs = program.fetch_outputs();
    let solution = outputs.last().copied().unwrap();
    if <i64 as TryInto<u8>>::try_into(solution).is_ok() {
        // If the number fits in a u8 then the intcode program did not actually output a solution

        for &output in &outputs {
            print!("{}", (output as u8) as char);
        }

        return Err(Box::new(SimpleError::new(String::from(
            "intcode program did not return a solution, see above output"
        ))));
    }

    Ok(solution)
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part(input, PART_1_INSTRUCTIONS)?;
    let solution2 = solve_part(input, PART_2_INSTRUCTIONS)?;

    Ok((solution1, solution2))
}