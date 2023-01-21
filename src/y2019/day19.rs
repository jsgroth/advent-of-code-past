//! Day 19: Tractor Beam
//! https://adventofcode.com/2019/day/19

use crate::y2019::intcode;
use crate::SimpleError;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct HeapEntry {
    x: i64,
    y: i64,
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x + self.y).cmp(&(other.x + other.y)).reverse()
    }
}

fn solve_part_1(input: &str) -> Result<usize, Box<dyn Error>> {
    let program = intcode::parse_program(input)?;

    let mut points_affected = 0;
    for x in 0..50 {
        for y in 0..50 {
            if is_in_beam(program.clone(), x, y) {
                points_affected += 1;
            }
        }
    }

    Ok(points_affected)
}

fn solve_part_2(input: &str) -> Result<i64, Box<dyn Error>> {
    let program = intcode::parse_program(input)?;

    let mut heap = BinaryHeap::new();

    let mut y = 0;
    while !is_in_beam(program.clone(), 100, y) {
        y += 1;
    }

    heap.push(HeapEntry { x: 100, y });

    let mut visited = HashSet::new();

    while let Some(HeapEntry { x, y }) = heap.pop() {
        if is_in_beam(program.clone(), x + 100, y)
            && is_in_beam(program.clone(), x, y + 100)
            && is_in_beam(program.clone(), x + 100, y + 100)
        {
            return Ok(10000 * x + y);
        }

        if !visited.insert(HeapEntry { x, y }) {
            continue;
        }

        let horizontal_entry = HeapEntry { x: x + 1, y };
        if is_in_beam(program.clone(), x + 1, y) && !visited.contains(&horizontal_entry) {
            heap.push(horizontal_entry);
        }

        let vertical_entry = HeapEntry { x, y: y + 1 };
        if is_in_beam(program.clone(), x, y + 1) && !visited.contains(&vertical_entry) {
            heap.push(vertical_entry);
        }
    }

    Err(Box::new(SimpleError::new(String::from(
        "no solution found",
    ))))
}

fn is_in_beam(mut program: Vec<i64>, x: i64, y: i64) -> bool {
    let mut outputs = Vec::new();

    intcode::execute(
        &mut program,
        intcode::iterator_input_fn(vec![x, y].into_iter()),
        |output| outputs.push(output),
    );

    if outputs.is_empty() {
        panic!("intcode program returned no outputs");
    }

    outputs[0] == 1
}

pub fn solve(input: &str) -> Result<(usize, i64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
