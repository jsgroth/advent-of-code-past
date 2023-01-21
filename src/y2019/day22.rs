//! Day 22: Slam Shuffle
//! https://adventofcode.com/2019/day/22

use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shuffle {
    DealIntoNewStack,
    Cut(usize),
    ReverseCut(usize),
    DealWithIncrement(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct LinearFunction {
    coefficient: i64,
    intercept: i64,
}

impl LinearFunction {
    fn identity() -> Self {
        Self { coefficient: 1, intercept: 0 }
    }

    fn compose(&self, other: Self, modulo: i64) -> Self {
        let coefficient = bigint_multiply(self.coefficient, other.coefficient, modulo);
        let intercept = (bigint_multiply(self.coefficient, other.intercept, modulo) + self.intercept) % modulo;
        Self { coefficient, intercept }
    }
}

fn bigint_multiply(a: i64, b: i64, modulo: i64) -> i64 {
    ((a as i128 * b as i128) % (modulo as i128)) as i64
}

impl Shuffle {
    fn from_line(line: &str) -> Result<Self, SimpleError> {
        let split: Vec<_> = line.split(' ').collect();
        match split.as_slice() {
            ["deal", "into", "new", "stack"] => Ok(Self::DealIntoNewStack),
            ["cut", n] => {
                let n: i32 = n.parse()?;
                if n > 0 {
                    Ok(Self::Cut(n as usize))
                } else {
                    Ok(Self::ReverseCut(-n as usize))
                }
            }
            ["deal", "with", "increment", n] => {
                let n: usize = n.parse()?;
                Ok(Self::DealWithIncrement(n))
            }
            _ => Err(SimpleError::new(format!("invalid shuffle line: {line}")))
        }
    }
}

const PART_1_DECK_SIZE: i64 = 10007;

fn solve_part_1(input: &str) -> Result<i64, SimpleError> {
    let shuffles = parse_input(input)?;

    let shuffle_function = reduce_to_function(&shuffles, PART_1_DECK_SIZE);

    let pos_2019 = (shuffle_function.coefficient * 2019 + shuffle_function.intercept).rem_euclid(PART_1_DECK_SIZE);

    Ok(pos_2019)
}

const PART_2_DECK_SIZE: i64 = 119315717514047;
const PART_2_SHUFFLES: u64 = 101741582076661;

fn solve_part_2(input: &str) -> Result<i64, SimpleError> {
    let shuffles = parse_input(input)?;

    let shuffle_function = reduce_to_function(&shuffles, PART_2_DECK_SIZE);

    let function = compose_function_n_times(shuffle_function, PART_2_SHUFFLES, PART_2_DECK_SIZE);

    // At this point we have a linear congruence of this form:
    //     ax + b ≡ 2020 (mod N)
    // Where a is the shuffle function coefficient, b is the shuffle function intercept, N is the
    // deck size, and x is unknown. Through basic properties of modular arithmetic, we can rewrite
    // this as:
    //     ax ≡ 2020 - b (mod N)
    // We want to solve for some x such that 0 <= x < N.
    // The deck size N is a prime number which means that a and N are by definition coprime. Thus
    // there exists a unique modular multiplicative inverse of a mod N such that:
    //     a * a_-1 ≡ 1 (mod N)
    // We can solve for a_-1 by using the extended Euclidean algorithm to solve for z in Bézout's
    // equation (the value of w is the modular multiplicative inverse of N mod a, which we don't
    // care about here):
    //     az + Nw = 1
    // With a_-1 known, we can finally solve for x:
    //     x ≡ a_-1 * (2020 - b) (mod N)
    // From which we can easily compute the single value of x such that 0 <= x < N.
    // Isn't modular arithmetic fun?

    let target = (2020 - function.intercept) % PART_2_DECK_SIZE;

    let coefficient_modular_inverse = modular_inverse(function.coefficient, PART_2_DECK_SIZE);

    let number_in_2020 = bigint_multiply(coefficient_modular_inverse, target, PART_2_DECK_SIZE)
        .rem_euclid(PART_2_DECK_SIZE);

    Ok(number_in_2020)
}

fn modular_inverse(a: i64, b: i64) -> i64 {
    let (mut prev_r, mut prev_s) = (a, 1);
    let (mut r, mut s) = (b, 0);

    while prev_r % r != 0 {
        let q = prev_r / r;

        let next_r = prev_r - q * r;
        let next_s = prev_s - q * s;

        prev_r = r;
        prev_s = s;

        r = next_r;
        s = next_s;
    }

    s
}

// Reduce the list of shuffles to a function of the form f(x) = ax + b
fn reduce_to_function(shuffles: &[Shuffle], deck_size: i64) -> LinearFunction {
    let mut coefficient = 1;
    let mut intercept = 0;
    for &shuffle in shuffles {
        match shuffle {
            Shuffle::DealIntoNewStack => {
                coefficient = -coefficient;
                intercept = deck_size - 1 - intercept;
            }
            Shuffle::Cut(n) => {
                intercept = (intercept - n as i64) % deck_size;
            }
            Shuffle::ReverseCut(n) => {
                intercept = (intercept + n as i64) % deck_size;
            }
            Shuffle::DealWithIncrement(n) => {
                coefficient = (coefficient * n as i64) % deck_size;
                intercept = (intercept * n as i64) % deck_size;
            }
        }
    }

    LinearFunction { coefficient, intercept }
}

fn compose_function_n_times(function: LinearFunction, n: u64, modulo: i64) -> LinearFunction {
    let mut functions = vec![function];
    while 2_u64.pow(functions.len() as u32 - 1) < n {
        let last = functions.last().copied().unwrap();
        functions.push(last.compose(last, modulo));
    }

    let mut function = LinearFunction::identity();
    let mut current_shuffles = 0;
    for i in (0..functions.len()).rev() {
        let function_shuffles = 2_u64.pow(i as u32);
        if current_shuffles + function_shuffles <= n {
            function = function.compose(functions[i], modulo);
            current_shuffles += function_shuffles;
        }
    }

    function
}

fn parse_input(input: &str) -> Result<Vec<Shuffle>, SimpleError> {
    input.lines().map(Shuffle::from_line).collect()
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}