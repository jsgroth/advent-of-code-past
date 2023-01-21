//! Day 13: Mine Cart Madness
//!
//! <https://adventofcode.com/2018/day/13>

use crate::SimpleError;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::mem;
use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Void,
    HorizontalTrack,
    VerticalTrack,
    Intersection,
    RightCurve,
    LeftCurve,
}

impl Space {
    fn from_char(c: char) -> Result<Self, SimpleError> {
        match c {
            ' ' => Ok(Self::Void),
            '-' | '<' | '>' => Ok(Self::HorizontalTrack),
            '|' | '^' | 'v' => Ok(Self::VerticalTrack),
            '+' => Ok(Self::Intersection),
            '/' => Ok(Self::RightCurve),
            '\\' => Ok(Self::LeftCurve),
            _ => Err(SimpleError::new(format!("invalid space char: {c}"))),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self::new(
            (self.x as i32 + rhs.0) as usize,
            (self.y as i32 + rhs.1) as usize,
        )
    }
}

impl AddAssign<(i32, i32)> for Point {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        *self = *self + rhs;
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TurnDirection {
    Left,
    Straight,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct MineCart {
    position: Point,
    dx: i32,
    dy: i32,
    last_turn_direction: Option<TurnDirection>,
}

impl MineCart {
    const MINE_CART_CHARS: [char; 4] = ['^', '<', '>', 'v'];

    fn new(x: usize, y: usize, dx: i32, dy: i32) -> Self {
        Self {
            position: Point::new(x, y),
            dx,
            dy,
            last_turn_direction: None,
        }
    }

    fn tick(&mut self) {
        self.position += (self.dx, self.dy);
    }

    fn maybe_turn(&mut self, current_space: Space) {
        match current_space {
            Space::HorizontalTrack | Space::VerticalTrack => {}
            Space::Intersection => match self.last_turn_direction {
                None | Some(TurnDirection::Right) => {
                    self.turn_left();
                    self.last_turn_direction = Some(TurnDirection::Left);
                }
                Some(TurnDirection::Left) => {
                    self.last_turn_direction = Some(TurnDirection::Straight);
                }
                Some(TurnDirection::Straight) => {
                    self.turn_right();
                    self.last_turn_direction = Some(TurnDirection::Right);
                }
            },
            Space::RightCurve => {
                if self.dx != 0 {
                    self.turn_left();
                } else {
                    self.turn_right();
                }
            }
            Space::LeftCurve => {
                if self.dx != 0 {
                    self.turn_right();
                } else {
                    self.turn_left();
                }
            }
            Space::Void => panic!("mine cart fell into the void: {self:?}"),
        }
    }

    fn turn_left(&mut self) {
        self.dx = -self.dx;
        mem::swap(&mut self.dx, &mut self.dy);
    }

    fn turn_right(&mut self) {
        self.dy = -self.dy;
        mem::swap(&mut self.dx, &mut self.dy);
    }
}

fn solve_part_1(input: &str) -> Result<Point, SimpleError> {
    let (grid, mut mine_carts) = parse_input(input)?;

    loop {
        mine_carts.sort_by_key(|mine_cart| mine_cart.position);

        let mut mine_cart_positions: HashSet<_> = mine_carts
            .iter()
            .map(|mine_cart| mine_cart.position)
            .collect();
        for mine_cart in &mut mine_carts {
            mine_cart_positions.remove(&mine_cart.position);
            mine_cart.tick();

            if mine_cart_positions.contains(&mine_cart.position) {
                return Ok(mine_cart.position);
            }

            mine_cart_positions.insert(mine_cart.position);
            mine_cart.maybe_turn(grid[mine_cart.position.y][mine_cart.position.x]);
        }
    }
}

fn solve_part_2(input: &str) -> Result<Point, SimpleError> {
    let (grid, mut mine_carts) = parse_input(input)?;

    loop {
        mine_carts.sort_by_key(|mine_cart| mine_cart.position);

        let mut mine_cart_positions: HashMap<_, _> = mine_carts
            .iter()
            .enumerate()
            .map(|(i, mine_cart)| (mine_cart.position, i))
            .collect();

        let mut crashed_cart_indices = HashSet::new();
        for (i, mine_cart) in mine_carts.iter_mut().enumerate() {
            if crashed_cart_indices.contains(&i) {
                continue;
            }

            mine_cart_positions.remove(&mine_cart.position);
            mine_cart.tick();

            if let Some(other_cart_index) = mine_cart_positions.remove(&mine_cart.position) {
                crashed_cart_indices.insert(i);
                crashed_cart_indices.insert(other_cart_index);
                continue;
            }

            mine_cart_positions.insert(mine_cart.position, i);
            mine_cart.maybe_turn(grid[mine_cart.position.y][mine_cart.position.x]);
        }

        mine_carts = mine_carts
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| !crashed_cart_indices.contains(&i))
            .map(|(_, mine_cart)| mine_cart)
            .collect();

        if mine_carts.len() == 1 {
            return Ok(mine_carts[0].position);
        }
    }
}

fn parse_input(input: &str) -> Result<(Vec<Vec<Space>>, Vec<MineCart>), SimpleError> {
    let lines: Vec<_> = input.lines().collect();
    if lines.is_empty() {
        return Err(SimpleError::new(String::from("input has no lines")));
    }

    let rows = lines.len();
    let cols = lines[0].len();

    let mut grid = vec![vec![Space::Void; cols]; rows];
    let mut mine_carts = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = Space::from_char(c)?;
            if MineCart::MINE_CART_CHARS.contains(&c) {
                let (dx, dy) = match c {
                    '^' => (0, -1),
                    '<' => (-1, 0),
                    '>' => (1, 0),
                    'v' => (0, 1),
                    _ => panic!("should not be possible due to surrounding if check"),
                };

                let mine_cart = MineCart::new(j, i, dx, dy);
                mine_carts.push(mine_cart);
            }
        }
    }

    Ok((grid, mine_carts))
}

pub(crate) fn solve(input: &str) -> Result<(Point, Point), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample13.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample13-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(Point::new(7, 3)), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(Point::new(6, 4)), solve_part_2(SAMPLE_INPUT_2));
    }
}
