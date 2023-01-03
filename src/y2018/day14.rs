//! Day 14: Chocolate Charts
//! https://adventofcode.com/2018/day/14

use std::cell::RefCell;
use std::error::Error;
use std::rc::{Rc, Weak};
use crate::SimpleError;

#[derive(Debug)]
struct ListNode {
    val: u32,
    next: Weak<RefCell<ListNode>>,
}

impl ListNode {
    fn unwrap_next(&self) -> Rc<RefCell<ListNode>> {
        self.next.upgrade().expect("every node should have a valid next pointer")
    }
}

#[derive(Debug)]
struct CircularList {
    nodes: Vec<Rc<RefCell<ListNode>>>,
}

impl CircularList {
    fn new(initial_value: u32) -> Self {
        let head = Rc::new_cyclic(|me| {
            RefCell::new(ListNode {
                val: initial_value,
                next: Weak::clone(me),
            })
        });
        let nodes = vec![head];

        Self { nodes }
    }

    fn head(&self) -> Rc<RefCell<ListNode>> {
        Rc::clone(&self.nodes[0])
    }

    fn tail(&self) -> Rc<RefCell<ListNode>> {
        Rc::clone(self.nodes.last().unwrap())
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn push(&mut self, value: u32) {
        let new_node = Rc::new(RefCell::new(ListNode {
            val: value,
            next: Rc::downgrade(&self.nodes[0]),
        }));

        let tail = self.nodes.last().unwrap();
        tail.borrow_mut().next = Rc::downgrade(&new_node);
        self.nodes.push(new_node);
    }
}

const INITIAL_STATE: [u32; 2] = [3, 7];

fn solve_part_1(input: &str) -> Result<String, SimpleError> {
    let num_recipes: usize = crate::read_single_line(input)?.parse()?;

    let mut circular_list = CircularList::new(INITIAL_STATE[0]);
    circular_list.push(INITIAL_STATE[1]);

    let mut elf0_node = circular_list.head();
    let mut elf1_node = circular_list.tail();
    while circular_list.len() < num_recipes + 10 {
        let new_recipe = elf0_node.borrow().val + elf1_node.borrow().val;
        if new_recipe > 9 {
            circular_list.push(1);
            circular_list.push(new_recipe % 10);
        } else {
            circular_list.push(new_recipe);
        }

        let elf0_val = elf0_node.borrow().val;
        for _ in 0..=elf0_val {
            let next = elf0_node.borrow().unwrap_next();
            elf0_node = next;
        }

        let elf1_val = elf1_node.borrow().val;
        for _ in 0..=elf1_val {
            let next = elf1_node.borrow().unwrap_next();
            elf1_node = next;
        }
    }

    let mut current_node = circular_list.head();
    for _ in 0..num_recipes {
        let next = current_node.borrow().unwrap_next();
        current_node = next;
    }

    let mut result = String::new();
    for _ in 0..10 {
        result.push(char::from_digit(current_node.borrow().val, 10).unwrap());

        let next = current_node.borrow().unwrap_next();
        current_node = next;
    }

    Ok(result)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let target_sequence: Vec<_> = crate::read_single_line(input)?.chars()
        .map(|c| c.to_digit(10).ok_or(SimpleError::new(format!("not a digit: {c}"))))
        .collect::<Result<_, _>>()?;

    let mut circular_list = CircularList::new(INITIAL_STATE[0]);
    circular_list.push(INITIAL_STATE[1]);

    let mut elf0_node = circular_list.head();
    let mut elf1_node = circular_list.tail();
    let mut target_index = 0;
    loop {
        let new_recipe = elf0_node.borrow().val + elf1_node.borrow().val;
        let new_values = if new_recipe > 9 {
            vec![1, new_recipe % 10]
        } else {
            vec![new_recipe]
        };

        for (i, &new_value) in new_values.iter().enumerate() {
            if new_value == target_sequence[target_index] {
                target_index += 1;
            } else {
                target_index = if new_value == target_sequence[0] {
                    1
                } else {
                    0
                };
            }

            if target_index == target_sequence.len() {
                return Ok(circular_list.len() + (i + 1) - target_sequence.len());
            }
        }

        for &new_value in &new_values {
            circular_list.push(new_value);
        }

        let elf0_val = elf0_node.borrow().val;
        for _ in 0..=elf0_val {
            let next = elf0_node.borrow().unwrap_next();
            elf0_node = next;
        }

        let elf1_val = elf1_node.borrow().val;
        for _ in 0..=elf1_val {
            let next = elf1_node.borrow().unwrap_next();
            elf1_node = next;
        }
    }
}

pub fn solve(input: &str) -> Result<(String, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(String::from("5158916779")), solve_part_1("9"));
        assert_eq!(Ok(String::from("0124515891")), solve_part_1("5"));
        assert_eq!(Ok(String::from("9251071085")), solve_part_1("18"));
        assert_eq!(Ok(String::from("5941429882")), solve_part_1("2018"));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(9), solve_part_2("51589"));
        assert_eq!(Ok(5), solve_part_2("01245"));
        assert_eq!(Ok(18), solve_part_2("92510"));
        assert_eq!(Ok(2018), solve_part_2("59414"));
    }
}