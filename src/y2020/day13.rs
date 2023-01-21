//! Day 13: Shuttle Search
//! https://adventofcode.com/2020/day/13

use std::error::Error;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<u64, SimpleError> {
    let earliest_departure: u64 = crate::read_single_line(input)?.parse()?;

    let second_line = input.lines().nth(1).ok_or_else(
        || SimpleError::new(String::from("input should have 2 lines"))
    )?;

    let buses: Vec<_> = second_line.split(',')
        .filter(|&s| s != "x")
        .map(|s| s.parse::<u64>())
        .collect::<Result<_, _>>()?;

    for t in earliest_departure.. {
        for &bus in &buses {
            if t % bus == 0 {
                return Ok(bus * (t - earliest_departure));
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn solve_part_2(input: &str) -> Result<i128, SimpleError> {
    let second_line = input.lines().nth(1).ok_or_else(
        || SimpleError::new(String::from("input should have 2 lines"))
    )?;

    let buses_with_indices: Vec<_> = second_line.split(',').enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| s.parse::<i64>().map(|n| (i as i64, n)))
        .collect::<Result<_, _>>()?;

    // To find the solution, we treat the list of buses as a system of linear congruences of the
    // form:
    //     x + index ≡ 0 (mod bus_id)
    // Or equivalently:
    //     x ≡ bus_id - index (mod bus_id)
    // Rewrite the list as (a, N) pairs such that x ≡ a (mod N) and 0 <= a < N
    let linear_congruences = buses_with_indices.into_iter()
        .map(|(index, bus)| ((bus - index) as i128, bus as i128));

    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    //
    // Given a pair of linear congruences:
    //     x ≡ a (mod m)
    //     x ≡ b (mod n)
    // If m and n are coprime then there exists a solution x, and furthermore x is unique (mod mn).
    // In other words, there exists exactly one solution in the range 0 <= x < mn.
    //
    // All solutions satisfy the following congruence:
    //     x ≡ a * m_mod_n_inv * m + b * n_mod_m_inv * m (mod mn)
    // Where m_mod_n_inv is the modular multiplicative inverse of m (mod n), and n_mod_m_inv is the
    // modular multiplicative inverse of n (mod m). These inverses can be computed by using the
    // extended Euclidean algorithm to solve Bézout's equation (mx + ny = 1) for x and y.
    //
    // This can be generalized to a set of N linear congruences as long as each pair of divisors is
    // coprime. If m, n, and p are pairwise coprime, then mn and p are also coprime, and thus we
    // can apply the theorem by reducing over the system of congruences.
    let (solution, _) = linear_congruences
        .reduce(|(a, m), (b, n)| {
            let (m_mod_n_inv, n_mod_m_inv) = extended_euclidean_algorithm(m, n);

            let remainder = (b * m_mod_n_inv * m + a * n_mod_m_inv * n).rem_euclid(m * n);

            (remainder, m * n)
        })
        .unwrap();

    Ok(solution)
}

fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, i128) {
    let (mut prev_r, mut prev_s, mut prev_t) = (a, 1, 0);
    let (mut r, mut s, mut t) = (b, 0, 1);

    while prev_r % r != 0 {
        let q = prev_r / r;

        let next_r = prev_r - q * r;
        let next_s = prev_s - q * s;
        let next_t = prev_t - q * t;

        prev_r = r;
        prev_s = s;
        prev_t = t;

        r = next_r;
        s = next_s;
        t = next_t;
    }

    (s, t)
}

pub fn solve(input: &str) -> Result<(u64, i128), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample13.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(295), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(1068781), solve_part_2(SAMPLE_INPUT));
        assert_eq!(Ok(3417), solve_part_2("\n17,x,13,19"));
        assert_eq!(Ok(754018), solve_part_2("\n67,7,59,61"));
        assert_eq!(Ok(779210), solve_part_2("\n67,x,7,59,61"));
        assert_eq!(Ok(1261476), solve_part_2("\n67,7,x,59,61"));
        assert_eq!(Ok(1202161486), solve_part_2("\n1789,37,47,1889"));
    }
}