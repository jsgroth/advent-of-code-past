pub mod y2015;
pub mod y2016;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::io::Read;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
struct SimpleError {
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

impl From<ParseIntError> for SimpleError {
    fn from(err: ParseIntError) -> Self {
        Self { msg: err.to_string() }
    }
}

pub fn run_solution<T1, T2, F>(solution: F) -> Result<(), Box<dyn Error>>
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