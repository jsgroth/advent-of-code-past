//! Day 23: Category Six
//!
//! <https://adventofcode.com/2019/day/23>

use crate::y2019::intcode;
use crate::y2019::intcode::{InputFn, IntcodeProgram, OutputFn};
use std::collections::VecDeque;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct NetworkInputFn {
    input_queue: Arc<Mutex<VecDeque<i64>>>,
    idle_flag: Arc<Mutex<bool>>,
}

struct NetworkOutputFn {
    output_queue: Arc<Mutex<VecDeque<i64>>>,
    output_buffer: Vec<i64>,
}

impl InputFn for NetworkInputFn {
    fn call(&mut self) -> Option<i64> {
        let input = self.input_queue.lock().unwrap().pop_front().unwrap_or(-1);
        if input == -1 {
            *self.idle_flag.lock().unwrap() = true;
            thread::sleep(Duration::from_millis(1));
        }
        Some(input)
    }
}

impl OutputFn for NetworkOutputFn {
    fn call(&mut self, output: i64) {
        self.output_buffer.push(output);
        if self.output_buffer.len() == 3 {
            self.output_queue
                .lock()
                .unwrap()
                .extend(self.output_buffer.iter().copied());
            self.output_buffer.clear();
        }
    }
}

fn solve_both_parts(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let program = intcode::parse_program(input)?;

    let packet_queue = Arc::new(Mutex::new(VecDeque::new()));

    let mut first_nat_y: Option<i64> = None;
    let mut last_nat_x: Option<i64> = None;
    let mut last_nat_y: Option<i64> = None;
    let mut last_delivered_y: Option<i64> = None;

    let mut program_queues = Vec::new();
    let mut idle_flags = Vec::new();
    for i in 0..50 {
        let idle_flag = Arc::new(Mutex::new(false));
        let program_queue = Arc::new(Mutex::new(VecDeque::from([i])));

        let mut program = IntcodeProgram::new(
            program.clone(),
            NetworkInputFn {
                input_queue: Arc::clone(&program_queue),
                idle_flag: Arc::clone(&idle_flag),
            },
            NetworkOutputFn {
                output_buffer: Vec::new(),
                output_queue: Arc::clone(&packet_queue),
            },
        );

        thread::spawn(move || {
            program.execute();
        });

        program_queues.push(program_queue);
        idle_flags.push(idle_flag);
    }

    loop {
        if packet_queue.lock().unwrap().is_empty() {
            let mut all_idle = true;

            for idle_flag in &idle_flags {
                let idle = *idle_flag.lock().unwrap();
                if !idle {
                    all_idle = false;
                    break;
                }
            }

            if !all_idle {
                thread::sleep(Duration::from_millis(1));
                continue;
            }

            // Read a second time in case something inserted in the interim
            if !packet_queue.lock().unwrap().is_empty() {
                continue;
            }

            if last_delivered_y.is_some() && last_delivered_y == last_nat_y {
                return Ok((first_nat_y.unwrap(), last_nat_y.unwrap()));
            }
            last_delivered_y = last_nat_y;

            {
                let mut program_queue_0 = program_queues[0].lock().unwrap();
                program_queue_0
                    .push_back(last_nat_x.expect("all programs idle before NAT packet sent"));
                program_queue_0
                    .push_back(last_nat_y.expect("all programs idle before NAT packet sent"));
            }

            *idle_flags[0].lock().unwrap() = false;

            continue;
        }

        let (address, x, y) = {
            let mut packet_queue = packet_queue.lock().unwrap();

            let address = packet_queue.pop_front().unwrap();
            let x = packet_queue.pop_front().unwrap();
            let y = packet_queue.pop_front().unwrap();

            (address, x, y)
        };

        if address == 255 {
            if first_nat_y.is_none() {
                first_nat_y = Some(y);
            }

            last_nat_x = Some(x);
            last_nat_y = Some(y);
        } else {
            {
                let mut program_queue = program_queues[address as usize].lock().unwrap();
                program_queue.push_back(x);
                program_queue.push_back(y);
            }

            *idle_flags[address as usize].lock().unwrap() = false;
        }
    }
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let (solution1, solution2) = solve_both_parts(input)?;

    Ok((solution1, solution2))
}
