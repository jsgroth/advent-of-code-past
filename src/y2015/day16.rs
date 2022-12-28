//! Day 16: Aunt Sue
//! https://adventofcode.com/2015/day/16

use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Compound {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl Compound {
    fn from_str(s: &str) -> Result<Self, SimpleError> {
        let compound = match s {
            "children" => Self::Children,
            "cats" => Self::Cats,
            "samoyeds" => Self::Samoyeds,
            "pomeranians" => Self::Pomeranians,
            "akitas" => Self::Akitas,
            "vizslas" => Self::Vizslas,
            "goldfish" => Self::Goldfish,
            "trees" => Self::Trees,
            "cars" => Self::Cars,
            "perfumes" => Self::Perfumes,
            _ => return Err(SimpleError::new(format!("invalid compound string: {s}"))),
        };
        Ok(compound)
    }
}

struct AuntSue {
    known_compounds: HashMap<Compound, u32>,
}

const TARGET_VALUES: [(Compound, u32); 10] = [
    (Compound::Children, 3),
    (Compound::Cats, 7),
    (Compound::Samoyeds, 2),
    (Compound::Pomeranians, 3),
    (Compound::Akitas, 0),
    (Compound::Vizslas, 0),
    (Compound::Goldfish, 5),
    (Compound::Trees, 3),
    (Compound::Cars, 2),
    (Compound::Perfumes, 1),
];

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let aunt_sues = parse_input(input)?;

    let target_values: HashMap<_, _> = TARGET_VALUES.iter().copied().collect();

    for (i, aunt_sue) in aunt_sues.iter().enumerate() {
        if aunt_sue.known_compounds.iter().all(|(compound, number)| {
            target_values.get(compound) == Some(number)
        }) {
            return Ok(i + 1);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let aunt_sues = parse_input(input)?;

    let target_values: HashMap<_, _> = TARGET_VALUES.iter().copied().collect();

    for (i, aunt_sue) in aunt_sues.iter().enumerate() {
        if aunt_sue.known_compounds.iter().all(|(compound, &number)| {
            match compound {
                Compound::Cats | Compound::Trees => {
                    number > target_values.get(compound).copied().unwrap()
                }
                Compound::Pomeranians | Compound::Goldfish => {
                    number < target_values.get(compound).copied().unwrap()
                }
                compound => number == target_values.get(compound).copied().unwrap(),
            }
        }) {
            return Ok(i + 1);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn parse_input(input: &str) -> Result<Vec<AuntSue>, SimpleError> {
    input.lines().map(|line| {
        let split: Vec<_> = line.split(' ').skip(2).collect();

        let mut known_compounds: HashMap<Compound, u32> = HashMap::new();
        for chunk in split.chunks(2) {
            let compound = chunk[0];
            let compound = &compound[..compound.len() - 1];
            let compound = Compound::from_str(compound)?;

            let mut number = chunk[1];
            if number.chars().last() == Some(',') {
                number = &number[..number.len() - 1];
            }
            let number: u32 = number.parse()?;

            known_compounds.insert(compound, number);
        }

        Ok(AuntSue { known_compounds })
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}