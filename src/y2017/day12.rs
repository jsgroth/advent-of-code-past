//! Day 12: Digital Plumber
//! https://adventofcode.com/2017/day/12

use std::collections::HashSet;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug)]
struct DisjointSetNode {
    parent_index: usize,
    size: usize,
}

#[derive(Debug)]
struct DisjointSet {
    nodes: Vec<DisjointSetNode>,
}

impl DisjointSet {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    fn ensure_node_exists(&mut self, index: usize) {
        while self.nodes.len() <= index {
            self.nodes.push(DisjointSetNode {
                parent_index: self.nodes.len(),
                size: 1,
            });
        }
    }

    fn find(&mut self, index: usize) -> usize {
        if self.nodes[index].parent_index != index {
            let root_index = self.find(self.nodes[index].parent_index);
            self.nodes[index].parent_index = root_index;
        }
        self.nodes[index].parent_index
    }

    fn union(&mut self, i: usize, j: usize) {
        self.ensure_node_exists(i);
        self.ensure_node_exists(j);

        let i_root = self.find(i);
        let j_root = self.find(j);
        if i_root != j_root {
            if self.nodes[i_root].size >= self.nodes[j_root].size {
                self.nodes[j_root].parent_index = i_root;
                self.nodes[i_root].size += self.nodes[j_root].size;
            } else {
                self.nodes[i_root].parent_index = j_root;
                self.nodes[j_root].size += self.nodes[i_root].size;
            }
        }
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let input = parse_input(input)?;

    let mut disjoint_set = DisjointSet::new();
    for (left_node, right_nodes) in input {
        for right_node in right_nodes {
            disjoint_set.union(left_node, right_node);
        }
    }

    let zero_set = disjoint_set.find(0);
    Ok(disjoint_set.nodes[zero_set].size)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let input = parse_input(input)?;

    let mut disjoint_set = DisjointSet::new();
    for (left_node, right_nodes) in input {
        for right_node in right_nodes {
            disjoint_set.union(left_node, right_node);
        }
    }

    let distinct_groups: HashSet<_> = (0..disjoint_set.nodes.len())
        .map(|i| disjoint_set.find(i))
        .collect();

    Ok(distinct_groups.len())
}

fn parse_input(input: &str) -> Result<Vec<(usize, Vec<usize>)>, SimpleError> {
    input.lines().map(|line| {
        let (l, r) = line.split_once(" <-> ").ok_or(
            SimpleError::new(format!("invalid line, missing <->: {line}"))
        )?;

        let l: usize = l.parse()?;
        let r = r.split(", ")
            .map(|n| n.parse::<usize>().map_err(SimpleError::from))
            .collect::<Result<Vec<_>, _>>()?;

        Ok((l, r))
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample12.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(6), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(2), solve_part_2(SAMPLE_INPUT));
    }
}