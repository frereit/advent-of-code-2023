use anyhow::{Context, Error, Result};
use std::str::FromStr;

use crate::filemanip::read_lines;

#[derive(Debug, PartialEq, Eq, Default)]
struct Cubes {
    red: u64,
    green: u64,
    blue: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCubesError;

impl FromStr for Cubes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = Cubes::default();
        for entry in s.split(',') {
            let (number, kind) = entry
                .trim()
                .split_once(' ')
                .and_then(|(number, kind)| Some((number.parse::<u64>().ok()?, kind.trim())))
                .ok_or(Error::msg("Failed to parse cube list"))?;
            match kind {
                "red" => cubes.red = number,
                "green" => cubes.green = number,
                "blue" => cubes.blue = number,
                _ => return Err(Error::msg("Invalid color specified")),
            }
        }
        Ok(cubes)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u64,
    rounds: Vec<Cubes>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rounds) = s
            .strip_prefix("Game ")
            .and_then(|s| s.split_once(':'))
            .and_then(|(id, rounds)| Some((id.parse::<u64>().ok()?, rounds.trim())))
            .ok_or(Error::msg("Failed to parse game structure"))?;

        let rounds = rounds
            .split(';')
            .map(|round| round.parse::<Cubes>())
            .collect::<Result<Vec<Cubes>>>()?;

        Ok(Game { id, rounds })
    }
}

pub fn calculate_valid_game_sum(file: &std::path::Path) -> Result<u64> {
    let mut sum = 0u64;
    let max_cubes = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    for line in read_lines(file)
        .context(format!("Failed to read file {:#?}", file))?
        .flatten()
    {
        let game = line.parse::<Game>()?;
        if game.rounds.iter().all(|cubes| {
            cubes.red <= max_cubes.red
                && cubes.green <= max_cubes.green
                && cubes.blue <= max_cubes.blue
        }) {
            sum += game.id;
        }
    }
    Ok(sum)
}
