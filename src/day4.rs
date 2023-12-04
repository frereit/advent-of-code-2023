use anyhow::{Context, Error, Result};

use crate::filemanip::read_lines;

pub fn calculate_scratchcard_sum(file: &std::path::Path) -> Result<u64> {
    let mut sum = 0u64;
    for line in read_lines(file)
        .context(format!("Failed to read file {:#?}", file))?
        .flatten()
    {
        let (_card, info) = line.split_once(": ").ok_or(Error::msg("Malformed input"))?;
        let (winning_numbers, my_numbers) = info
            .split_once(" | ")
            .ok_or(Error::msg("Malformed input"))?;
        let winning_numbers = winning_numbers
            .split(' ')
            .flat_map(|s| s.trim().parse::<u64>())
            .collect::<Vec<_>>();
        let my_winning_numbers_count = my_numbers
            .split(' ')
            .flat_map(|s| s.trim().parse::<u64>())
            .filter(|x| winning_numbers.contains(x))
            .count();
        if my_winning_numbers_count != 0 {
            sum += 2u64.pow((my_winning_numbers_count - 1) as u32);
        }
    }
    Ok(sum)
}

pub fn calculate_scratchcard_count(file: &std::path::Path) -> Result<u64> {
    let original_cards = read_lines(file)
        .context(format!("Failed to read file {:#?}", file))?
        .flatten()
        .collect::<Vec<_>>();
    let mut unprocessed_cards: Vec<(usize, &String)> = original_cards
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &String)>>();
    let mut count = 0;
    while !unprocessed_cards.is_empty() {
        count += 1;
        let (line_idx, line) = unprocessed_cards
            .pop()
            .expect("While condition states this i not empty.");
        let (_card, info) = line.split_once(": ").ok_or(Error::msg("Malformed input"))?;
        let (winning_numbers, my_numbers) = info
            .split_once(" | ")
            .ok_or(Error::msg("Malformed input"))?;
        let winning_numbers = winning_numbers
            .split(' ')
            .flat_map(|s| s.trim().parse::<u64>())
            .collect::<Vec<_>>();
        let my_winning_numbers_count = my_numbers
            .split(' ')
            .flat_map(|s| s.trim().parse::<u64>())
            .filter(|x| winning_numbers.contains(x))
            .count();
        if my_winning_numbers_count != 0 {
            for i in 1..=my_winning_numbers_count {
                if let Some(other_line) = original_cards.get(line_idx + i) {
                    unprocessed_cards.push((line_idx + i, other_line));
                }
            }
        }
    }
    Ok(count as u64)
}
