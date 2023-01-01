#![forbid(unsafe_code)]

pub mod y2015;
pub mod y2016;
pub mod y2017;

use std::char::ParseCharError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::io::Read;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[derive(Debug, PartialEq, Eq)]
pub struct SimpleError {
    msg: String,
}

impl SimpleError {
    fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl Display for SimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for SimpleError {}

trait ErrorWrapper : Error {}

impl ErrorWrapper for ParseIntError {}

impl ErrorWrapper for ParseCharError {}

impl ErrorWrapper for FromUtf8Error {}

impl<T: ErrorWrapper> From<T> for SimpleError {
    fn from(t: T) -> Self {
        Self { msg: t.to_string() }
    }
}

fn run_solution<T1, T2, F>(solution: F) -> Result<(), Box<dyn Error>>
where
    T1: Display,
    T2: Display,
    F: Fn(&str) -> Result<(T1, T2), Box<dyn Error>>,
{
    let input = read_input()?;

    let (solution1, solution2) = solution(&input)?;
    println!("{solution1}");
    println!("{solution2}");

    Ok(())
}

// Read input from stdin
fn read_input() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    Ok(s)
}

fn read_single_line(input: &str) -> Result<&str, SimpleError> {
    match input.lines().next() {
        Some(line) => Ok(line),
        None => Err(SimpleError::new(String::from("input is empty, expected a single line"))),
    }
}