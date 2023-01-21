//! Day 17: Spinlock
//!
//! <https://adventofcode.com/2017/day/17>

use crate::SimpleError;
use std::cell::RefCell;
use std::error::Error;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct BufferNode {
    val: u32,
    next: Weak<RefCell<BufferNode>>,
}

impl BufferNode {
    fn unwrap_next(&self) -> Rc<RefCell<BufferNode>> {
        match self.next.upgrade() {
            Some(rc) => rc,
            None => panic!("node with value {} has an invalid next reference", self.val),
        }
    }
}

#[derive(Debug)]
struct CircularBuffer {
    nodes: Vec<Rc<RefCell<BufferNode>>>,
    head: Rc<RefCell<BufferNode>>,
}

impl CircularBuffer {
    fn new() -> Self {
        let head = Rc::new_cyclic(|me| {
            RefCell::new(BufferNode {
                val: 0,
                next: Weak::clone(me),
            })
        });
        let nodes = vec![Rc::clone(&head)];
        Self { nodes, head }
    }

    fn insert_new_node(
        &mut self,
        node: &Rc<RefCell<BufferNode>>,
        new_val: u32,
    ) -> Rc<RefCell<BufferNode>> {
        let new_node = Rc::new(RefCell::new(BufferNode {
            val: new_val,
            next: Weak::clone(&node.borrow().next),
        }));
        node.borrow_mut().next = Rc::downgrade(&new_node);
        self.nodes.push(Rc::clone(&new_node));

        new_node
    }
}

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let steps_per_turn: usize = crate::read_single_line(input)?.parse()?;

    let mut circular_buffer = CircularBuffer::new();
    let last_node_inserted =
        populate_circular_buffer(&mut circular_buffer, steps_per_turn, 1..=2017);

    let next_after_2017 = last_node_inserted.borrow().unwrap_next().borrow().val;
    Ok(next_after_2017)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let steps_per_turn: usize = crate::read_single_line(input)?.parse()?;

    let mut value_after_zero = 1;
    let mut current_pos = 1;
    for i in 2..=50_000_000 {
        current_pos = ((current_pos + steps_per_turn) % i) + 1;
        if current_pos == 1 {
            value_after_zero = i;
        }
    }

    Ok(value_after_zero as u32)
}

fn populate_circular_buffer(
    circular_buffer: &mut CircularBuffer,
    steps_per_turn: usize,
    values: impl Iterator<Item = u32>,
) -> Rc<RefCell<BufferNode>> {
    let mut current_node = Rc::clone(&circular_buffer.head);
    for i in values {
        for _ in 0..steps_per_turn {
            let next_node = current_node.borrow().unwrap_next();
            current_node = next_node;
        }

        current_node = circular_buffer.insert_new_node(&current_node, i);
    }

    current_node
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(638), solve_part_1("3"));
    }
}
