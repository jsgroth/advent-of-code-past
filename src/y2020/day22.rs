//! Day 22: Crab Combat
//!
//! <https://adventofcode.com/2020/day/22>

use crate::SimpleError;
use std::collections::{HashSet, VecDeque};
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Winner {
    Player1,
    Player2,
}

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let (p1_deck, p2_deck) = parse_input(input)?;

    let mut p1_deck = VecDeque::from(p1_deck);
    let mut p2_deck = VecDeque::from(p2_deck);

    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        let p1_card = p1_deck.pop_front().unwrap();
        let p2_card = p2_deck.pop_front().unwrap();

        if p1_card > p2_card {
            p1_deck.push_back(p1_card);
            p1_deck.push_back(p2_card);
        } else {
            p2_deck.push_back(p2_card);
            p2_deck.push_back(p1_card);
        }
    }

    let winning_deck = if !p1_deck.is_empty() {
        p1_deck
    } else {
        p2_deck
    };

    let score = score_deck(&winning_deck);

    Ok(score)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let (p1_deck, p2_deck) = parse_input(input)?;

    let (winner, p1_deck, p2_deck) =
        play_recursive_combat(VecDeque::from(p1_deck), VecDeque::from(p2_deck));

    let winning_deck = match winner {
        Winner::Player1 => p1_deck,
        Winner::Player2 => p2_deck,
    };

    let score = score_deck(&winning_deck);

    Ok(score)
}

fn play_recursive_combat(
    mut p1_deck: VecDeque<u32>,
    mut p2_deck: VecDeque<u32>,
) -> (Winner, VecDeque<u32>, VecDeque<u32>) {
    let mut previous_states = HashSet::new();

    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        if !previous_states.insert((p1_deck.clone(), p2_deck.clone())) {
            // Infinite loop
            return (Winner::Player1, p1_deck, p2_deck);
        }

        let p1_card = p1_deck.pop_front().unwrap();
        let p2_card = p2_deck.pop_front().unwrap();

        let sub_game_winner = if p1_card <= p1_deck.len() as u32 && p2_card <= p2_deck.len() as u32
        {
            let p1_sub_deck: VecDeque<_> = p1_deck.iter().copied().take(p1_card as usize).collect();
            let p2_sub_deck: VecDeque<_> = p2_deck.iter().copied().take(p2_card as usize).collect();

            let (sub_game_winner, _, _) = play_recursive_combat(p1_sub_deck, p2_sub_deck);
            sub_game_winner
        } else if p1_card > p2_card {
            Winner::Player1
        } else {
            Winner::Player2
        };

        match sub_game_winner {
            Winner::Player1 => {
                p1_deck.push_back(p1_card);
                p1_deck.push_back(p2_card);
            }
            Winner::Player2 => {
                p2_deck.push_back(p2_card);
                p2_deck.push_back(p1_card);
            }
        }
    }

    if !p1_deck.is_empty() {
        (Winner::Player1, p1_deck, p2_deck)
    } else {
        (Winner::Player2, p1_deck, p2_deck)
    }
}

fn score_deck(deck: &VecDeque<u32>) -> u32 {
    deck.iter()
        .rev()
        .copied()
        .enumerate()
        .map(|(i, card)| (i as u32 + 1) * card)
        .sum()
}

fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<u32>), SimpleError> {
    let mut lines = input.lines();

    let p1_deck = lines
        .by_ref()
        .skip(1)
        .take_while(|s| !s.is_empty())
        .map(|line| line.parse::<u32>())
        .collect::<Result<_, _>>()?;

    let p2_deck = lines
        .skip(1)
        .map(|line| line.parse::<u32>())
        .collect::<Result<_, _>>()?;

    Ok((p1_deck, p2_deck))
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample22.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(306), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(291), solve_part_2(SAMPLE_INPUT));
    }
}
