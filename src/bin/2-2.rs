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

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Outcome::*;
        Ok(match s {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => Err("hand is not in range".to_string())?,
        })
    }
}

impl Hand {
    fn win_hand(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn lose_hand(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn other_from_outcome(&self, outcome: Outcome) -> Hand {
        match outcome {
            Outcome::Win => self.win_hand(),
            Outcome::Lose => self.lose_hand(),
            Outcome::Draw => *self,
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
        .map(|(a, b)| (a.parse::<Hand>().unwrap(), b.parse::<Outcome>().unwrap()))
        .map(|(a, b)| a.other_from_outcome(b).score() + b.score())
        .sum();

    println!("result = {result}");

    Ok(())
}
