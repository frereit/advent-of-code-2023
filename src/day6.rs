use anyhow::{Context, Result};

use crate::filemanip::read_lines;

fn parse_line(line: &str, kerning: bool) -> Vec<u64> {
    line.split_once(": ")
        .map(|(_, times)| {
            if kerning {
                times.trim().replace(' ', "")
            } else {
                times.trim().to_owned()
            }
        })
        .unwrap()
        .split(' ')
        .flat_map(|x| x.parse::<u64>())
        .collect::<Vec<_>>()
}

fn hold_time_to_distance(hold_time: u64, max_time: u64) -> u64 {
    if hold_time == 0 || hold_time >= max_time {
        return 0;
    }
    let travel_time = max_time - hold_time;
    travel_time * hold_time
}

pub fn calculate_margin_of_error(file: &std::path::Path, kerning: bool) -> Result<u64> {
    let mut product = 1u64;
    let data = read_lines(file)
        .context(format!("Failed to read file {:#?}", file))?
        .flatten()
        .collect::<Vec<_>>();
    let times = parse_line(data.get(0).unwrap(), kerning);
    let distances = parse_line(data.get(1).unwrap(), kerning);
    for (time, min_distance) in times.iter().zip(distances.iter()) {
        let mut count = 0;
        for hold_duration in 1..*time {
            let distance = hold_time_to_distance(hold_duration, *time);
            if distance > *min_distance {
                count += 1;
            }
        }
        product *= count;
    }
    Ok(product)
}
