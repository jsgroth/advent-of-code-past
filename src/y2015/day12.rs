//! Day 12: JSAbacusFramework.io
//!
//! <https://adventofcode.com/2015/day/12>

use crate::SimpleError;
use std::collections::HashMap;
use std::error::Error;
use std::iter::Peekable;

#[derive(Debug)]
enum JsonValue {
    Number(i32),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    fn sum_all_numbers<P>(&self, predicate: P) -> i32
    where
        P: Copy + Fn(&JsonValue) -> bool,
    {
        if !predicate(self) {
            return 0;
        }

        match self {
            Self::Number(n) => *n,
            Self::String(_) => 0,
            Self::Array(array) => array
                .iter()
                .map(|json_value| json_value.sum_all_numbers(predicate))
                .sum(),
            Self::Object(object) => object
                .values()
                .map(|json_value| json_value.sum_all_numbers(predicate))
                .sum(),
        }
    }
}

fn solve_part_1(input: &str) -> Result<i32, SimpleError> {
    let root_value = parse_input(input)?;

    Ok(root_value.sum_all_numbers(|_| true))
}

fn solve_part_2(input: &str) -> Result<i32, SimpleError> {
    let root_value = parse_input(input)?;

    Ok(root_value.sum_all_numbers(|json_value| match json_value {
        JsonValue::Object(object) => !contains_red_value(object),
        _ => true,
    }))
}

fn contains_red_value(object: &HashMap<String, JsonValue>) -> bool {
    object.values().any(|json_value| match json_value {
        JsonValue::String(s) => s == "red",
        _ => false,
    })
}

fn parse_input(input: &str) -> Result<JsonValue, SimpleError> {
    let line = crate::read_single_line(input)?;

    parse_json_value(&mut line.chars().peekable())
}

fn parse_json_value<I>(iter: &mut Peekable<I>) -> Result<JsonValue, SimpleError>
where
    I: Iterator<Item = char>,
{
    if iter.peek().is_none() {
        return Err(SimpleError::new(String::from("unexpected end of stream")));
    }

    match iter.peek().unwrap() {
        '{' => parse_object(iter),
        '[' => parse_array(iter),
        '"' => parse_string(iter),
        '-' => parse_int(iter),
        _c @ '0'..='9' => parse_int(iter),
        _ => Err(SimpleError::new(format!(
            "invalid character at start of value: {}",
            iter.peek().unwrap()
        ))),
    }
}

fn parse_object<I>(iter: &mut Peekable<I>) -> Result<JsonValue, SimpleError>
where
    I: Iterator<Item = char>,
{
    iter.next();

    skip_spaces(iter);

    let mut object: HashMap<String, JsonValue> = HashMap::new();
    loop {
        if iter.peek() == Some(&'}') {
            iter.next();
            break;
        }

        let key = match parse_string(iter) {
            Ok(JsonValue::String(key)) => key,
            _ => return Err(SimpleError::new(String::from("unable to parse object key"))),
        };

        skip_spaces(iter);

        if iter.next() != Some(':') {
            return Err(SimpleError::new(format!(
                "missing colon after object key '{key}'"
            )));
        }

        skip_spaces(iter);

        let value = parse_json_value(iter)?;
        object.insert(key, value);

        skip_spaces(iter);

        if iter.peek() == Some(&',') {
            iter.next();
        }
    }

    Ok(JsonValue::Object(object))
}

fn parse_array<I>(iter: &mut Peekable<I>) -> Result<JsonValue, SimpleError>
where
    I: Iterator<Item = char>,
{
    iter.next();

    skip_spaces(iter);

    let mut array: Vec<JsonValue> = Vec::new();
    loop {
        if iter.peek() == Some(&']') {
            iter.next();
            break;
        }

        array.push(parse_json_value(iter)?);

        skip_spaces(iter);

        if iter.peek() == Some(&',') {
            iter.next();
        }
    }

    Ok(JsonValue::Array(array))
}

fn parse_string<I>(iter: &mut Peekable<I>) -> Result<JsonValue, SimpleError>
where
    I: Iterator<Item = char>,
{
    iter.next();

    let mut s = String::new();
    for c in iter.by_ref() {
        if c == '"' {
            break;
        }

        s.push(c);
    }

    Ok(JsonValue::String(s))
}

fn parse_int<I>(iter: &mut Peekable<I>) -> Result<JsonValue, SimpleError>
where
    I: Iterator<Item = char>,
{
    let mut s = String::new();
    while let Some(&c) = iter.peek() {
        if c != '-' && !('0'..='9').contains(&c) {
            break;
        }

        s.push(iter.next().unwrap());
    }

    Ok(JsonValue::Number(s.parse()?))
}

fn skip_spaces<I>(iter: &mut Peekable<I>)
where
    I: Iterator<Item = char>,
{
    while iter.peek() == Some(&' ') {
        iter.next();
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
        assert_eq!(Ok(6), solve_part_1("[1,2,3]"));
        assert_eq!(Ok(6), solve_part_1(r#"{"a":2,"b":4}"#));
        assert_eq!(Ok(3), solve_part_1("[[[3]]]"));
        assert_eq!(Ok(3), solve_part_1(r#"{"a":{"b":4},"c":-1}"#));
        assert_eq!(Ok(0), solve_part_1(r#"{"a":[-1,1]}"#));
        assert_eq!(Ok(0), solve_part_1(r#"[-1,{"a":1}]"#));
        assert_eq!(Ok(0), solve_part_1("[]"));
        assert_eq!(Ok(0), solve_part_1("{}"));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(6), solve_part_2("[1,2,3]"));
        assert_eq!(Ok(4), solve_part_2(r#"[1,{"c":"red","b":2},3]"#));
        assert_eq!(Ok(0), solve_part_2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#));
        assert_eq!(Ok(6), solve_part_2(r#"[1,"red",5]"#));
    }
}
