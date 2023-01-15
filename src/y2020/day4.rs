//! Day 4: Passport Processing
//! https://adventofcode.com/2020/day/4

use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum PassportFieldType {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}

impl PassportFieldType {
    const ALL: [Self; 8] = [
        Self::BirthYear,
        Self::IssueYear,
        Self::ExpirationYear,
        Self::Height,
        Self::HairColor,
        Self::EyeColor,
        Self::PassportId,
        Self::CountryId,
    ];

    fn validate(&self, value: &str) -> bool {
        match self {
            Self::BirthYear => {
                value.parse::<u32>().map(|year| year >= 1920 && year <= 2002).unwrap_or(false)
            }
            Self::IssueYear => {
                value.parse::<u32>().map(|year| year >= 2010 && year <= 2020).unwrap_or(false)
            }
            Self::ExpirationYear => {
                value.parse::<u32>().map(|year| year >= 2020 && year <= 2030).unwrap_or(false)
            }
            Self::Height => {
                if value.ends_with("cm") {
                    value[..value.len() - 2].parse::<u32>().map(|cm| cm >= 150 && cm <= 193).unwrap_or(false)
                } else if value.ends_with("in") {
                    value[..value.len() - 2].parse::<u32>().map(|inches| inches >= 59 && inches <= 76).unwrap_or(false)
                } else {
                    false
                }
            }
            Self::HairColor => {
                let chars: Vec<_> = value.chars().collect();
                chars.len() == 7 && chars[0] == '#' &&
                    chars[1..].iter().all(|&c| c.is_digit(10) || ('a'..='f').contains(&c))
            }
            Self::EyeColor => {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value)
            }
            Self::PassportId => {
                value.len() == 9 && value.chars().all(|c| c.is_digit(10))
            }
            Self::CountryId => true,
        }
    }
}

impl FromStr for PassportFieldType {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "byr" => Ok(Self::BirthYear),
            "iyr" => Ok(Self::IssueYear),
            "eyr" => Ok(Self::ExpirationYear),
            "hgt" => Ok(Self::Height),
            "hcl" => Ok(Self::HairColor),
            "ecl" => Ok(Self::EyeColor),
            "pid" => Ok(Self::PassportId),
            "cid" => Ok(Self::CountryId),
            _ => Err(SimpleError::new(format!("invalid passport field: {s}")))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PassportField {
    field_type: PassportFieldType,
    field_value: String,
}

impl FromStr for PassportField {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (field_name, field_value) = s.split_once(':').ok_or(
            SimpleError::new(format!("passport field does not contain a ':': {s}"))
        )?;

        let field_type = PassportFieldType::from_str(field_name)?;
        let field_value = String::from(field_value);

        Ok(Self { field_type, field_value })
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let passports = parse_input(input)?;

    let valid_count = passports.iter()
        .filter(|&passport| has_all_required_fields(passport))
        .count();

    Ok(valid_count)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let passports = parse_input(input)?;

    let valid_count = passports.iter()
        .filter(|&passport| {
            has_all_required_fields(passport) &&
                passport.iter().all(|field| field.field_type.validate(&field.field_value))
        })
        .count();

    Ok(valid_count)
}

fn has_all_required_fields(passport: &[PassportField]) -> bool {
    let fields_present: HashSet<_> = passport.iter()
        .map(|field| field.field_type)
        .collect();

    if fields_present.len() == PassportFieldType::ALL.len() {
        true
    } else if fields_present.len() == PassportFieldType::ALL.len() - 1 {
        !fields_present.contains(&PassportFieldType::CountryId)
    } else {
        false
    }
}

fn parse_input(input: &str) -> Result<Vec<Vec<PassportField>>, SimpleError> {
    let lines: Vec<_> = input.lines().collect();

    let mut passports = Vec::new();
    for line_group in lines.split(|s| s.is_empty()) {
        let mut passport_fields = Vec::new();
        for &line in line_group {
            for passport_field in line.split(' ') {
                let passport_field = PassportField::from_str(passport_field)?;
                passport_fields.push(passport_field);
            }
        }
        passports.push(passport_fields);
    }

    Ok(passports)
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}