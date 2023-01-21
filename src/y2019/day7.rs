//! Day 7: Amplification Circuit
//! https://adventofcode.com/2019/day/7

use crate::y2019::intcode;
use crate::y2019::intcode::{InputFn, IntcodeProgram, OutputFn};
use crate::SimpleError;
use std::cell::RefCell;
use std::cmp;
use std::collections::VecDeque;
use std::error::Error;
use std::rc::Rc;

#[derive(Debug)]
struct QueueInputFn {
    input_queue: Rc<RefCell<VecDeque<i64>>>,
}

impl InputFn for QueueInputFn {
    fn call(&mut self) -> Option<i64> {
        self.input_queue.borrow_mut().pop_front()
    }
}

#[derive(Debug)]
struct QueueOutputFn {
    output_queue: Rc<RefCell<VecDeque<i64>>>,
}

impl OutputFn for QueueOutputFn {
    fn call(&mut self, output: i64) {
        self.output_queue.borrow_mut().push_back(output);
    }
}

fn solve_part_1(input: &str) -> Result<i64, Box<dyn Error>> {
    let program = intcode::parse_program(input)?;

    let phase_permutations = permutations(&(0..5).collect());

    let mut max_thruster_signal = 0;
    for phase_permutation in &phase_permutations {
        let mut last_amplifier_output = 0;

        for &phase in phase_permutation {
            let mut program = program.clone();
            let mut outputs = Vec::new();
            intcode::execute(
                &mut program,
                intcode::iterator_input_fn(vec![phase as i64, last_amplifier_output].into_iter()),
                |output| outputs.push(output),
            );

            if outputs.is_empty() {
                return Err(Box::new(SimpleError::new(format!(
                    "amplifier returned no output for phase {phase}"
                ))));
            }

            last_amplifier_output = outputs[0];
        }

        max_thruster_signal = cmp::max(max_thruster_signal, last_amplifier_output);
    }

    Ok(max_thruster_signal)
}

fn solve_part_2(input: &str) -> Result<i64, Box<dyn Error>> {
    let program = intcode::parse_program(input)?;

    let phase_permutations = permutations(&(5..10).collect());

    let mut max_thruster_signal = 0;
    for phase_permutation in &phase_permutations {
        let mut queues = Vec::new();
        for &phase in phase_permutation {
            let mut queue = VecDeque::new();
            queue.push_back(phase as i64);
            queues.push(Rc::new(RefCell::new(queue)));
        }

        queues[0].borrow_mut().push_back(0);

        let mut programs = Vec::new();
        for i in 0..5 {
            let input_queue = Rc::clone(&queues[i]);
            let output_queue = Rc::clone(&queues[(i + 1) % queues.len()]);

            let program = IntcodeProgram::new(
                program.clone(),
                QueueInputFn { input_queue },
                QueueOutputFn { output_queue },
            );
            programs.push(program);
        }

        loop {
            let mut all_halted = true;

            for program in &mut programs {
                all_halted &= program.execute();
            }

            if all_halted {
                break;
            }
        }

        let thruster_signal = queues[0].borrow_mut().pop_front().ok_or_else(|| {
            SimpleError::new(String::from("programs did not produce a thruster signal"))
        })?;
        max_thruster_signal = cmp::max(max_thruster_signal, thruster_signal);
    }

    Ok(max_thruster_signal)
}

fn permutations(numbers: &Vec<usize>) -> Vec<Vec<usize>> {
    if numbers.is_empty() {
        return vec![Vec::new()];
    }

    let mut result = Vec::new();
    for &number in numbers {
        let new_numbers = numbers.iter().copied().filter(|&n| n != number).collect();
        for mut sub_permutation in permutations(&new_numbers) {
            sub_permutation.push(number);
            result.push(sub_permutation);
        }
    }

    result
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(
            43210,
            solve_part_1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0").unwrap()
        );
        assert_eq!(
            54321,
            solve_part_1(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            )
            .unwrap()
        );
        assert_eq!(65210, solve_part_1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0").unwrap());
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(139629729, solve_part_2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5").unwrap());
        assert_eq!(18216, solve_part_2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10").unwrap());
    }
}
