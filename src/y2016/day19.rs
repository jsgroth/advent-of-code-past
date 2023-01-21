//! Day 19: An Elephant Named Joseph
//!
//! <https://adventofcode.com/2016/day/19>

use crate::SimpleError;
use std::error::Error;

#[derive(Debug)]
struct Elf {
    next_index: usize,
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let elf_count: usize = crate::read_single_line(input)?.parse()?;

    let mut elf_circle = new_elf_circle(elf_count);

    let mut current_elf = 0;
    loop {
        let next_elf = elf_circle[current_elf].next_index;

        elf_circle[current_elf].next_index = elf_circle[next_elf].next_index;

        if elf_circle[current_elf].next_index == current_elf {
            return Ok(current_elf + 1);
        }

        current_elf = elf_circle[current_elf].next_index;
    }
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let elf_count: usize = crate::read_single_line(input)?.parse()?;
    if elf_count < 2 {
        return Err(SimpleError::new(format!(
            "elf_count is {elf_count}, must be at least 2"
        )));
    }

    let mut elf_circle = new_elf_circle(elf_count);

    let mut current_elf = 0;
    let mut remaining_elves = elf_count;
    let mut steal_target_elf = elf_count / 2;
    let mut steal_target_prev = steal_target_elf - 1;
    loop {
        let steal_target_next = elf_circle[steal_target_elf].next_index;
        elf_circle[steal_target_prev].next_index = steal_target_next;

        remaining_elves -= 1;

        if remaining_elves == 1 {
            return Ok(current_elf + 1);
        }

        current_elf = elf_circle[current_elf].next_index;

        if remaining_elves % 2 == 0 {
            steal_target_prev = steal_target_next;
            steal_target_elf = elf_circle[steal_target_next].next_index;
        } else {
            steal_target_elf = steal_target_next;
        }
    }
}

fn new_elf_circle(elf_count: usize) -> Vec<Elf> {
    let mut elves = Vec::new();
    for i in 0..elf_count {
        elves.push(Elf {
            next_index: (i + 1) % elf_count,
        });
    }
    elves
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(3), solve_part_1("5"));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(2), solve_part_2("5"));
    }
}
