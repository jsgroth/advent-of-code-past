//! Day 15: Beverage Bandits
//! https://adventofcode.com/2018/day/15

use std::cell::RefCell;
use std::cmp;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::error::Error;
use std::rc::Rc;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    // In reading order
    const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn adjacent_points(&self) -> impl Iterator<Item = Self> + '_ {
        Self::DIRECTIONS.iter().map(|&(dx, dy)| {
            Self::new((self.x as i32 + dx) as usize, (self.y as i32 + dy) as usize)
        })
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y)
            .then(self.x.cmp(&other.x))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum WarriorRace {
    Goblin,
    Elf,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Warrior {
    race: WarriorRace,
    position: Point,
    attack_power: i32,
    hit_points: i32,
}

impl Warrior {
    fn new(agent_type: WarriorRace, x: usize, y: usize) -> Self {
        Self {
            race: agent_type,
            position: Point::new(x, y),
            attack_power: 3,
            hit_points: 200,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Space {
    Empty,
    Wall,
    Warrior(Rc<RefCell<Warrior>>),
}

impl Clone for Space {
    fn clone(&self) -> Self {
        match self {
            Space::Empty => Space::Empty,
            Space::Wall => Space::Wall,
            Space::Warrior(warrior) => {
                Space::Warrior(Rc::new(RefCell::new(warrior.borrow().clone())))
            }
        }
    }
}

#[derive(Debug)]
struct SimulationInput {
    map: Vec<Vec<Space>>,
    goblins: Vec<Rc<RefCell<Warrior>>>,
    elves: Vec<Rc<RefCell<Warrior>>>,
}

impl Clone for SimulationInput {
    fn clone(&self) -> Self {
        let map = self.map.clone();

        let mut goblins = Vec::with_capacity(self.goblins.len());
        let mut elves = Vec::with_capacity(self.elves.len());
        for row in &map {
            for space in row {
                if let Space::Warrior(warrior) = space {
                    match warrior.borrow().race {
                        WarriorRace::Goblin => {
                            goblins.push(Rc::clone(warrior));
                        }
                        WarriorRace::Elf => {
                            elves.push(Rc::clone(warrior));
                        }
                    }
                }
            }
        }

        Self { map, goblins, elves }
    }
}

fn solve_part_1(input: &str) -> Result<i32, SimpleError> {
    let simulation_input = parse_input(input)?;

    let (final_full_round, goblins, elves) = run_combat_simulation(simulation_input);

    if goblins.is_empty() {
        return Ok(compute_score(final_full_round, &elves));
    }

    if elves.is_empty() {
        return Ok(compute_score(final_full_round, &goblins));
    }

    panic!("combat simulation left both goblins and elves remaining");
}

fn solve_part_2(input: &str) -> Result<i32, SimpleError> {
    let simulation_input = parse_input(input)?;

    for elf_attack_power in 4.. {
        for elf in &simulation_input.elves {
            elf.borrow_mut().attack_power = elf_attack_power;
        }

        let (final_full_round, _, end_elves) = run_combat_simulation(simulation_input.clone());

        if end_elves.len() == simulation_input.elves.len() {
            return Ok(compute_score(final_full_round, &end_elves));
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn run_combat_simulation(input: SimulationInput) -> (i32, Vec<Rc<RefCell<Warrior>>>, Vec<Rc<RefCell<Warrior>>>) {
    let SimulationInput {
        mut map,
        mut goblins,
        mut elves,
    } = input;

    for round in 1.. {
        let mut all_warriors: Vec<_> = goblins.iter()
            .map(|goblin| Rc::clone(goblin))
            .chain(
                elves.iter().map(|elf| Rc::clone(elf))
            )
            .collect();
        all_warriors.sort_by_key(|warrior| warrior.borrow().position);

        for warrior in &all_warriors {
            // Check if killed earlier in the round
            if warrior.borrow().hit_points <= 0 {
                continue;
            }

            // Check for win state
            let warrior_race = warrior.borrow().race;
            if (warrior_race == WarriorRace::Goblin && elves.is_empty()) ||
                (warrior_race == WarriorRace::Elf && goblins.is_empty()) {
                return (round - 1, goblins, elves);
            }

            // Move if not adjacent to an enemy and a target is reachable
            if let Some(move_target) = find_move_target(&map, warrior) {
                let warrior_position = warrior.borrow().position;
                if move_target != warrior_position {
                    map[warrior_position.y][warrior_position.x] = Space::Empty;
                    map[move_target.y][move_target.x] = Space::Warrior(Rc::clone(warrior));

                    warrior.borrow_mut().position = move_target;
                }
            } else {
                // No target reachable, skip attack check
                continue;
            }

            // Attack if adjacent to an enemy
            let warrior = warrior.borrow();
            if let Some(attack_target) = find_attack_target(&map, &warrior) {
                let other_warrior = match &map[attack_target.y][attack_target.x] {
                    Space::Warrior(w) => Rc::clone(w),
                    _ => panic!("attack target does not contain a warrior")
                };

                other_warrior.borrow_mut().hit_points -= warrior.attack_power;

                // If enemy is dead, remove them from the simulation
                let other_warrior = other_warrior.borrow();
                if other_warrior.hit_points <= 0 {
                    map[attack_target.y][attack_target.x] = Space::Empty;

                    match other_warrior.race {
                        WarriorRace::Goblin => {
                            goblins = goblins.into_iter()
                                .filter(|goblin| goblin.borrow().position != attack_target)
                                .collect();
                        }
                        WarriorRace::Elf => {
                            elves = elves.into_iter()
                                .filter(|elf| elf.borrow().position != attack_target)
                                .collect();
                        }
                    }
                }
            }
        }
    }

    panic!("simulation did not terminate organically");
}

fn find_move_target(
    map: &Vec<Vec<Space>>,
    warrior: &Rc<RefCell<Warrior>>,
) -> Option<Point> {
    let warrior = warrior.borrow();

    let (destination, destination_distance) = match find_closest_square_in_range(map, &warrior) {
        Some((a, b)) => (a, b),
        None => {
            // No target reachable
            return None;
        }
    };

    let warrior_position = warrior.position;

    if destination_distance == 0 {
        // Already adjacent to a target
        return Some(warrior_position);
    }

    if destination_distance == 1 {
        // Destination square is adjacent, just move into it
        return Some(destination);
    }

    for potential_first_move in warrior_position.adjacent_points() {
        if let Space::Empty = &map[potential_first_move.y][potential_first_move.x] {
            if find_distance_to(map, potential_first_move, destination) == Some(destination_distance - 1) {
                return Some(potential_first_move);
            }
        }
    }

    panic!("there should be an ideal first move if no early return");
}

fn find_closest_square_in_range(
    map: &Vec<Vec<Space>>,
    warrior: &Warrior,
) -> Option<(Point, usize)> {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    visited[warrior.position.y][warrior.position.x] = true;

    let mut queue = VecDeque::new();
    queue.push_back((warrior.position, 0));

    let mut closest_square: Option<(Point, usize)> = None;
    while !queue.is_empty() {
        let (position, distance) = queue.pop_front().unwrap();

        if closest_square.is_some() && distance > closest_square.unwrap().1 {
            break;
        }

        for adjacent_point in position.adjacent_points() {
            match &map[adjacent_point.y][adjacent_point.x] {
                Space::Wall => {},
                Space::Empty => {
                    if !visited[adjacent_point.y][adjacent_point.x] && closest_square.is_none() {
                        visited[adjacent_point.y][adjacent_point.x] = true;
                        queue.push_back((adjacent_point, distance + 1));
                    }
                }
                Space::Warrior(other_warrior) => {
                    if warrior.race != other_warrior.borrow().race {
                        closest_square = match closest_square {
                            Some((existing_point, _)) => {
                                Some((cmp::min(existing_point, position), distance))
                            }
                            None => Some((position, distance))
                        }
                    }
                }
            }
        }
    }

    closest_square
}

fn find_distance_to(
    map: &Vec<Vec<Space>>,
    position: Point,
    target_position: Point,
) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((position, 0));

    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    visited[position.y][position.x] = true;

    while !queue.is_empty() {
        let (position, distance) = queue.pop_front().unwrap();

        for adjacent_point in position.adjacent_points() {
            if let Space::Empty = map[adjacent_point.y][adjacent_point.x] {
                if adjacent_point == target_position {
                    return Some(distance + 1);
                }

                if !visited[adjacent_point.y][adjacent_point.x] {
                    visited[adjacent_point.y][adjacent_point.x] = true;
                    queue.push_back((adjacent_point, distance + 1));
                }
            }
        }
    }

    None
}

fn find_attack_target(map: &Vec<Vec<Space>>, warrior: &Warrior) -> Option<Point> {
    let mut target_hp = i32::MAX;
    let mut target_location: Option<Point> = None;
    for point in warrior.position.adjacent_points() {
        if let Space::Warrior(other_warrior) = &map[point.y][point.x] {
            let other_warrior = other_warrior.borrow();
            if other_warrior.race != warrior.race && other_warrior.hit_points < target_hp {
                target_hp = other_warrior.hit_points;
                target_location = Some(point);
            }
        }
    }

    target_location
}

fn compute_score(final_full_round: i32, remaining_warriors: &Vec<Rc<RefCell<Warrior>>>) -> i32 {
    let total_hit_points: i32 = remaining_warriors.iter()
        .map(|warrior| warrior.borrow().hit_points)
        .sum();

    final_full_round * total_hit_points
}

fn parse_input(input: &str) -> Result<SimulationInput, SimpleError> {
    let lines: Vec<_> = input.lines().collect();
    if lines.is_empty() {
        return Err(SimpleError::new(String::from("input has no lines")));
    }

    let mut map = vec![vec![Space::Empty; lines[0].len()]; lines.len()];
    let mut goblins = Vec::new();
    let mut elves = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    map[i][j] = Space::Empty;
                }
                '#' => {
                    map[i][j] = Space::Wall;
                }
                'G' => {
                    let goblin = Rc::new(RefCell::new(Warrior::new(
                        WarriorRace::Goblin, j, i
                    )));
                    map[i][j] = Space::Warrior(Rc::clone(&goblin));
                    goblins.push(goblin);
                }
                'E' => {
                    let elf = Rc::new(RefCell::new(Warrior::new(
                        WarriorRace::Elf, j, i
                    )));
                    map[i][j] = Space::Warrior(Rc::clone(&elf));
                    elves.push(elf);
                }
                _ => return Err(SimpleError::new(format!("unexpected char: {c}")))
            }
        }
    }

    Ok(SimulationInput { map, goblins, elves })
}

pub fn solve(input: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = include_str!("sample_input/sample15.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample15-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("sample_input/sample15-3.txt");
    const SAMPLE_INPUT_4: &str = include_str!("sample_input/sample15-4.txt");
    const SAMPLE_INPUT_5: &str = include_str!("sample_input/sample15-5.txt");
    const SAMPLE_INPUT_6: &str = include_str!("sample_input/sample15-6.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(27730), solve_part_1(SAMPLE_INPUT_1));
        assert_eq!(Ok(36334), solve_part_1(SAMPLE_INPUT_2));
        assert_eq!(Ok(39514), solve_part_1(SAMPLE_INPUT_3));
        assert_eq!(Ok(27755), solve_part_1(SAMPLE_INPUT_4));
        assert_eq!(Ok(28944), solve_part_1(SAMPLE_INPUT_5));
        assert_eq!(Ok(18740), solve_part_1(SAMPLE_INPUT_6));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(4988), solve_part_2(SAMPLE_INPUT_1));
        assert_eq!(Ok(31284), solve_part_2(SAMPLE_INPUT_3));
        assert_eq!(Ok(3478), solve_part_2(SAMPLE_INPUT_4));
        assert_eq!(Ok(6474), solve_part_2(SAMPLE_INPUT_5));
        assert_eq!(Ok(1140), solve_part_2(SAMPLE_INPUT_6));
    }
}