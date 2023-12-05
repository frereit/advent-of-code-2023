use std::{collections::VecDeque, str::FromStr};

use anyhow::{Context, Error, Result};

use crate::filemanip::read_lines;

#[derive(Debug, Clone, PartialEq)]
struct MapEntry {
    destination_range_start: u64,
    source_range_start: u64,
    range_len: u64,
}

impl MapEntry {
    pub fn try_map(&self, source: u64) -> Option<u64> {
        if source >= self.source_range_start && source < self.source_range_start + self.range_len {
            Some(self.destination_range_start + (source - self.source_range_start))
        } else {
            None
        }
    }
}

impl FromStr for MapEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let destination_range_start: u64 = parts
            .next()
            .ok_or(Error::msg("No destination range start"))?
            .parse()?;
        let source_range_start: u64 = parts
            .next()
            .ok_or(Error::msg("No source range start"))?
            .parse()?;
        let range_len: u64 = parts.next().ok_or(Error::msg("No range length"))?.parse()?;
        Ok(MapEntry {
            destination_range_start,
            source_range_start,
            range_len,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    pub fn map(&self, source: u64) -> u64 {
        self.entries
            .iter()
            .filter_map(|entry| entry.try_map(source))
            .next()
            .unwrap_or(source)
    }

    fn consume_map(input: &mut VecDeque<String>) -> Result<Map> {
        let mut entries: Vec<MapEntry> = vec![];
        input.pop_front().expect("Required name for map");
        while input.front().is_some_and(|line| !line.is_empty()) {
            let line = input.pop_front().expect("while condition");
            entries.push(line.parse()?);
        }
        input.pop_front();
        Ok(Map { entries })
    }
}

pub fn calculate_lowest_location_number(file: &std::path::Path) -> Result<u64> {
    let mut raw_data = read_lines(file)
        .context(format!("Failed to read file {:#?}", file))?
        .flatten()
        .collect::<VecDeque<_>>();
    let seeds = raw_data
        .pop_front()
        .ok_or(Error::msg("No seeds found."))?
        .split_once(": ")
        .ok_or(Error::msg("Malformed input"))?
        .1
        .split(' ')
        .flat_map(|s| s.parse::<u64>())
        .collect::<Vec<_>>();
    raw_data.pop_front();

    let mut maps = vec![];
    for _ in 0..7 {
        maps.push(Map::consume_map(&mut raw_data)?);
    }

    let mut min_location = u64::MAX;
    for seed in seeds {
        let location = maps.iter().fold(seed, |a, map| map.map(a));
        if location < min_location {
            min_location = location;
        }
    }

    Ok(min_location)
}

pub fn calculate_lowest_location_number_range(file: &std::path::Path) -> Result<u64> {
    let mut raw_data = read_lines(file)
        .context(format!("Failed to read file {:#?}", file))?
        .flatten()
        .collect::<VecDeque<_>>();
    let raw_seeds = raw_data
        .pop_front()
        .ok_or(Error::msg("No seeds found."))?
        .split_once(": ")
        .ok_or(Error::msg("Malformed input"))?
        .1
        .split(' ')
        .flat_map(|s| s.parse::<u64>())
        .collect::<Vec<_>>();
    dbg!("Raw seeds constructed!");
    let seeds = raw_seeds
        .chunks_exact(2)
        .flat_map(|seed_range| {
            let start = *seed_range.first().unwrap();
            let len = *seed_range.get(1).unwrap();
            start..start + len
        });
    dbg!("seed iterator constructed!");
    raw_data.pop_front();

    let mut maps = vec![];
    for _ in 0..7 {
        maps.push(Map::consume_map(&mut raw_data)?);
    }

    let mut min_location = u64::MAX;
    // Note: With the given input, this iterates ~2 billion seeds.
    // At ~10 million a seconnd, this took roughly 200 seconds on my machine.
    for seed in seeds {
        let location = maps.iter().fold(seed, |a, map| map.map(a));
        if location < min_location {
            min_location = location;
        }
    }

    Ok(min_location)
}
