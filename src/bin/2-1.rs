use anyhow::Result;
use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

impl Hand {
    fn fight(&self, other: &Hand) -> Outcome {
        let h1 = *self as i32;
        let h2 = *other as i32;
        let result = (h1 - h2).rem_euclid(3);
        use Outcome::*;
        match result {
            0 => Draw,
            1 => Lose,
            2 => Win,
            _ => unreachable!(),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Hand::*;
        Ok(match s {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => Err("hand is not in range".to_string())?,
        })
    }
}

fn main() -> Result<()> {
    let contents = read_to_string("inputs/2.txt")?;

    let result: u32 = contents
        .split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|row| row.split_once(' '))
        .map(|(a, b)| (a.parse::<Hand>().unwrap(), b.parse::<Hand>().unwrap()))
        .map(|(a, b)| a.fight(&b).score() + b.score())
        .sum();

    println!("result = {result}");

    Ok(())
}
