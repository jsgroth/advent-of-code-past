//! Day 3: No Matter How You Slice It
//! https://adventofcode.com/2018/day/3

use std::error::Error;
use crate::SimpleError;

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let rectangles = parse_input(input)?;

    let square = compute_tile_counts(&rectangles);

    let overlap_tiles = square.iter().map(|row| {
        row.iter().filter(|&&count| count > 1).count()
    })
        .sum();

    Ok(overlap_tiles)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let rectangles = parse_input(input)?;

    let square = compute_tile_counts(&rectangles);

    for (i, &rectangle) in rectangles.iter().enumerate() {
        let mut overlaps = false;
        'outer: for col in square.iter().skip(rectangle.x).take(rectangle.width) {
            for &count in col.iter().skip(rectangle.y).take(rectangle.height) {
                if count > 1 {
                    overlaps = true;
                    break 'outer;
                }
            }
        }

        if !overlaps {
            return Ok(i + 1);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn compute_tile_counts(rectangles: &Vec<Rectangle>) -> Vec<Vec<i32>> {
    let mut square = vec![vec![0; 1000]; 1000];
    for &rectangle in rectangles {
        for col in square.iter_mut().skip(rectangle.x).take(rectangle.width) {
            for count in col.iter_mut().skip(rectangle.y).take(rectangle.height) {
                *count += 1;
            }
        }
    }

    square
}

fn parse_input(input: &str) -> Result<Vec<Rectangle>, SimpleError> {
    input.lines().map(|line| {
        let (_, line) = line.split_once(" @ ").ok_or_else(
            || SimpleError::new(format!("line has no ' @ ': {line}"))
        )?;

        let (position, lengths) = line.split_once(": ").ok_or_else(
            || SimpleError::new(format!("line has no ': ': {line}"))
        )?;

        let (x, y) = position.split_once(',').ok_or_else(
            || SimpleError::new(format!("position part of line has no ',': {line}"))
        )?;

        let (w, h) = lengths.split_once('x').ok_or_else(
            || SimpleError::new(format!("lengths part of line has no 'x': {line}"))
        )?;

        Ok(Rectangle {
            x: x.parse()?,
            y: y.parse()?,
            width: w.parse()?,
            height: h.parse()?,
        })
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}