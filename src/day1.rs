use anyhow::{Context, Result};

use crate::filemanip::read_lines;

const WORDS_TO_VALUE: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn to_values(input: &str, include_words: bool) -> Vec<u64> {
    let mut values = vec![];
    for i in 0..input.len() {
        if let Some(value) = input.chars().nth(i).unwrap().to_digit(10) {
            values.push(value as u64);
        } else if include_words {
            let splice = &input[i..];
            let value = WORDS_TO_VALUE
                .iter()
                .enumerate()
                .filter_map(|(i, word)| splice.starts_with(word).then_some((i + 1) as u64))
                .next();
            if let Some(value) = value {
                values.push(value);
            }
        }
    }
    values
}

pub fn calculate_calibration_sum(file: &std::path::Path, include_words: bool) -> Result<u64> {
    let mut sum = 0u64;
    for line in read_lines(file)
        .context(format!("Failed to read file {:#?}", file))?
        .flatten()
    {
        let nums = to_values(&line, include_words);
        let first = nums
            .first()
            .context(format!("No numbers in line {:#}", &line))?;
        let last = nums
            .last()
            .expect("We just checked that nums contains values.");
        sum += (first * 10 + last) as u64;
    }
    Ok(sum)
}
