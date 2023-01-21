//! Day 23: Crab Cups
//! https://adventofcode.com/2020/day/23

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Debug;
use crate::SimpleError;

#[derive(Debug, Clone)]
struct CircularLinkedHashMap {
    nexts: Vec<usize>,
}

impl CircularLinkedHashMap {
    fn remove_after(&mut self, label: u32) -> u32 {
        let index = label as usize;
        let next_index = self.nexts[index];

        self.nexts[index] = self.nexts[next_index];

        next_index as u32
    }

    fn insert_after(&mut self, label: u32, new_value: u32) {
        let index = label as usize;
        let next_index = self.nexts[index];

        let new_value_index = new_value as usize;
        self.nexts[new_value_index] = next_index;
        self.nexts[index] = new_value_index;
    }
}

impl From<Vec<u32>> for CircularLinkedHashMap {
    fn from(value: Vec<u32>) -> Self {
        if value.is_empty() {
            return Self { nexts: Vec::new() };
        }

        let max_value = *value.iter().max().unwrap();

        let head = value[0] as usize;

        let mut nexts = vec![usize::MAX; max_value as usize + 1];
        let mut last_label = 0;
        for number in value {
            nexts[number as usize] = head;
            if last_label > 0 {
                nexts[last_label] = number as usize;
            }

            last_label = number as usize;
        }

        Self { nexts }
    }
}

fn solve_part_1(input: &str, moves: usize) -> Result<String, SimpleError> {
    let mut numbers: VecDeque<_> = crate::read_single_line(input)?.chars()
        .map(|c| c.to_digit(10).ok_or_else(|| SimpleError::new(format!("not a digit: {c}"))))
        .collect::<Result<_, _>>()?;

    let min = numbers.iter().copied().min().unwrap();
    let max = numbers.iter().copied().max().unwrap();

    for _ in 0..moves {
        let current_cup = *numbers.front().unwrap();

        numbers.rotate_left(1);
        let mut removed = Vec::new();
        for _ in 0..3 {
            removed.push(numbers.pop_front().unwrap());
        }

        let mut destination_cup = if current_cup == min {
            max
        } else {
            current_cup - 1
        };

        while removed.contains(&destination_cup) {
            destination_cup = if destination_cup == min {
                max
            } else {
                destination_cup - 1
            };
        }

        let destination_index = numbers.iter().position(|&n| n == destination_cup).unwrap();
        numbers.rotate_left((destination_index + 1) % numbers.len());
        numbers.extend(removed.into_iter());

        let current_index = numbers.iter().position(|&n| n == current_cup).unwrap();
        numbers.rotate_left((current_index + 1) % numbers.len());
    }

    let one_position = numbers.iter().position(|&n| n == 1).unwrap();
    numbers.rotate_left(one_position % numbers.len());
    numbers.pop_front();

    Ok(numbers.into_iter().map(|n| char::from_digit(n, 10).unwrap()).collect())
}

fn solve_part_2(input: &str) -> Result<u64, SimpleError> {
    let mut numbers: Vec<_> = crate::read_single_line(input)?.chars()
        .map(|c| c.to_digit(10).ok_or_else(|| SimpleError::new(format!("not a digit: {c}"))))
        .collect::<Result<_, _>>()?;

    let initial_max = *numbers.iter().max().unwrap();

    for n in initial_max + 1..=1_000_000 {
        numbers.push(n);
    }

    let start = numbers[0];
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    let mut circular_linked_hash = CircularLinkedHashMap::from(numbers);

    let mut current_cup = start;
    for _ in 0..10_000_000 {
        let mut removed = Vec::new();
        for _ in 0..3 {
            removed.push(circular_linked_hash.remove_after(current_cup));
        }

        let mut destination_cup = if current_cup == min {
            max
        } else {
            current_cup - 1
        };

        while removed.contains(&destination_cup) {
            destination_cup = if destination_cup == min {
                max
            } else {
                destination_cup - 1
            };
        }

        for removed_value in removed.into_iter().rev() {
            circular_linked_hash.insert_after(destination_cup, removed_value);
        }

        current_cup = circular_linked_hash.nexts[current_cup as usize] as u32;
    }

    let a = circular_linked_hash.nexts[1];
    let b = circular_linked_hash.nexts[a];

    Ok(a as u64 * b as u64)
}

pub fn solve(input: &str) -> Result<(String, u64), Box<dyn Error>> {
    let solution1 = solve_part_1(input, 100)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(String::from("92658374")), solve_part_1("389125467", 10));
        assert_eq!(Ok(String::from("67384529")), solve_part_1("389125467", 100))
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(149245887792), solve_part_2("389125467"));
    }
}