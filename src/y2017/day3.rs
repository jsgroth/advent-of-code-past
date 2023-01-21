//! Day 3: Spiral Memory
//!
//! <https://adventofcode.com/2017/day/3>

use crate::SimpleError;
use std::collections::HashMap;
use std::error::Error;
use std::mem;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn all_adjacent_points(&self) -> impl Iterator<Item = Self> + '_ {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .map(|(dx, dy)| Point::new(self.x + dx, self.y + dy))
    }
}

fn solve_part_1(input: &str) -> Result<i32, SimpleError> {
    let target: i32 = crate::read_single_line(input)?.parse()?;

    if target == 1 {
        return Ok(1);
    }

    let mut current_bound = 1;
    let mut current_multiple = 0;
    while current_bound <= target {
        current_multiple += 1;
        current_bound += 8 * current_multiple;
    }

    let relative_pos =
        (target - (current_bound - 8 * current_multiple + 1)) % (2 * current_multiple);
    let horizontal_pos = (relative_pos - (current_multiple - 1)).abs();
    Ok(horizontal_pos + current_multiple)
}

fn solve_part_2(input: &str) -> Result<i32, SimpleError> {
    let target_value: i32 = crate::read_single_line(input)?.parse()?;

    let mut generated_values = HashMap::new();
    generated_values.insert(Point::new(0, 0), 1);

    let mut x = 0_i32;
    let mut y = 0_i32;
    let mut side_len = 0;
    loop {
        side_len += 2;
        x += 1;
        y -= 1;

        let mut dx = 0;
        let mut dy = 1;
        for _ in 0..4 {
            for _ in 0..side_len {
                x += dx;
                y += dy;

                let point = Point::new(x, y);

                let mut value = 0;
                for p in point.all_adjacent_points() {
                    if let Some(&prev_value) = generated_values.get(&p) {
                        value += prev_value;
                    }
                }

                if value > target_value {
                    return Ok(value);
                }

                generated_values.insert(point, value);
            }

            dy = -dy;
            mem::swap(&mut dx, &mut dy);
        }
    }
}

pub fn solve(input: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(1), solve_part_1("1"));
        assert_eq!(Ok(3), solve_part_1("12"));
        assert_eq!(Ok(2), solve_part_1("23"));
        assert_eq!(Ok(31), solve_part_1("1024"));
    }
}
