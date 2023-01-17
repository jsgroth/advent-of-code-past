//! Day 23: Crab Cups
//! https://adventofcode.com/2020/day/23

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use crate::SimpleError;

#[derive(Debug, Clone)]
struct Node<T> {
    val: T,
    next: Weak<RefCell<Node<T>>>,
}

#[derive(Debug, Clone)]
struct CircularLinkedHashMap<T: Eq + Hash> {
    nodes: HashMap<T, Rc<RefCell<Node<T>>>>,
}

impl<T: Debug + Eq + Hash + Clone> CircularLinkedHashMap<T> {
    fn contains_key(&self, value: &T) -> bool {
        self.nodes.contains_key(value)
    }

    fn get(&self, value: &T) -> Option<Rc<RefCell<Node<T>>>> {
        self.nodes.get(value).map(|rc| Rc::clone(rc))
    }

    fn remove_after(&mut self, value: &T) -> Option<T> {
        match self.nodes.get(value) {
            Some(node) => {
                let next = node.borrow().next.upgrade().unwrap();
                node.borrow_mut().next = Weak::clone(&next.borrow().next);

                let next_value = next.borrow().val.clone();
                self.nodes.remove(&next_value);
                Some(next_value)
            }
            None => None
        }
    }

    fn insert_after(&mut self, value: &T, new_value: T) {
        if let Some(node) = self.nodes.get(value) {
            let new_node = Rc::new(RefCell::new(Node {
                val: new_value.clone(),
                next: Weak::clone(&node.borrow().next),
            }));

            node.borrow_mut().next = Rc::downgrade(&new_node);
            self.nodes.insert(new_value, new_node);
        }
    }
}

impl<T: Debug + Eq + Hash + Clone, I: IntoIterator<Item = T>> From<I> for CircularLinkedHashMap<T> {
    fn from(value: I) -> Self {
        let mut iter = value.into_iter().peekable();
        if iter.peek().is_none() {
            return Self { nodes: HashMap::new() };
        }

        let first = iter.next().unwrap();
        let head = Rc::new_cyclic(|me| {
            RefCell::new(Node {
                val: first.clone(),
                next: Weak::clone(me),
            })
        });

        let mut nodes = HashMap::new();
        nodes.insert(first, Rc::clone(&head));

        let mut tail = Rc::clone(&head);
        while let Some(value) = iter.next() {
            let node = Rc::new(RefCell::new(Node {
                val: value.clone(),
                next: Weak::clone(&tail.borrow().next),
            }));

            nodes.insert(value.clone(), Rc::clone(&node));
            tail.borrow_mut().next = Rc::downgrade(&node);

            tail = node;
        }

        Self { nodes }
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
            removed.push(circular_linked_hash.remove_after(&current_cup).unwrap());
        }

        let mut destination_cup = if current_cup == min {
            max
        } else {
            current_cup - 1
        };

        while !circular_linked_hash.contains_key(&destination_cup) {
            destination_cup = if destination_cup == min {
                max
            } else {
                destination_cup - 1
            };
        }

        for removed_value in removed.into_iter().rev() {
            circular_linked_hash.insert_after(&destination_cup, removed_value);
        }

        let current_node = circular_linked_hash.get(&current_cup).unwrap();
        let next_node = current_node.borrow().next.upgrade().unwrap();
        current_cup = next_node.borrow().val;
    }

    let node_after_1 = circular_linked_hash.get(&1).unwrap().borrow().next.upgrade().unwrap();
    let a = node_after_1.borrow().val as u64;
    let b = node_after_1.borrow().next.upgrade().unwrap().borrow().val as u64;

    println!("{a} {b}");

    Ok(a * b)
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