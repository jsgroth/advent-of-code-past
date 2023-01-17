//! Day 23: Crab Cups
//! https://adventofcode.com/2020/day/23

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Debug;
use crate::SimpleError;

#[derive(Debug, Clone)]
struct Node {
    val: u32,
    next: usize,
}

#[derive(Debug, Clone)]
struct CircularLinkedHashMap {
    nodes: Vec<Node>,
    label_to_index: Vec<usize>,
}

impl CircularLinkedHashMap {
    fn get(&self, label: u32) -> &Node {
        &self.nodes[self.label_to_index[label as usize]]
    }

    fn remove_after(&mut self, label: u32) -> u32 {
        let index = self.label_to_index[label as usize];
        let next_index = self.nodes[index].next;
        let next_value = self.nodes[next_index].val;

        self.nodes[index].next = self.nodes[next_index].next;

        next_value
    }

    fn insert_after(&mut self, label: u32, new_value: u32) {
        let index = self.label_to_index[label as usize];
        let next_index = self.nodes[index].next;

        let new_value_index = self.label_to_index[new_value as usize];
        self.nodes[new_value_index].next = next_index;
        self.nodes[index].next = new_value_index;
    }
}

impl From<Vec<u32>> for CircularLinkedHashMap {
    fn from(value: Vec<u32>) -> Self {
        if value.is_empty() {
            return Self { nodes: Vec::new(), label_to_index: Vec::new() };
        }

        let max_value = *value.iter().max().unwrap();

        let mut nodes = Vec::with_capacity(value.len());
        let mut label_to_index = vec![usize::MAX; max_value as usize + 1];
        for number in value {
            label_to_index[number as usize] = nodes.len();
            nodes.push(Node { val: number, next: 0 });

            if nodes.len() > 1 {
                let nodes_len = nodes.len();
                nodes[nodes_len - 2].next = nodes_len - 1;
            }
        }

        Self { nodes, label_to_index }
    }
}

fn solve_part_1(input: &str, moves: usize) -> Result<String, SimpleError> {
    let mut numbers: VecDeque<_> = crate::read_single_line(input)?.chars()
        .map(|c| c.to_digit(10).ok_or(SimpleError::new(format!("not a digit: {c}"))))
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
        .map(|c| c.to_digit(10).ok_or(SimpleError::new(format!("not a digit: {c}"))))
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

        let current_node = circular_linked_hash.get(current_cup);
        current_cup = circular_linked_hash.nodes[current_node.next].val;
    }

    let one_node = circular_linked_hash.get(1);
    let a_node = &circular_linked_hash.nodes[one_node.next];
    let b_node = &circular_linked_hash.nodes[a_node.next];

    Ok(a_node.val as u64 * b_node.val as u64)
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