use std::{cmp, collections::HashMap};

use anyhow::Result;

use crate::filemanip::read_lines;

pub fn calculate_schematic_sum(file: &std::path::Path) -> Result<u64> {
    let mut sum = 0u64;
    let lines = read_lines(file)?.collect::<Result<Vec<_>, _>>()?;
    for (row_idx, row) in lines.iter().enumerate() {
        for (col_idx, c) in row.chars().enumerate() {
            // Check if a number STARTS at col_idx
            if c.is_ascii_digit()
                && (col_idx == 0
                    || row
                        .chars()
                        .nth(col_idx - 1)
                        .is_some_and(|c| !c.is_ascii_digit()))
            {
                let mut value = 0u64;
                let mut view_idx = col_idx;
                while row
                    .chars()
                    .nth(view_idx)
                    .is_some_and(|c| c.is_ascii_digit())
                {
                    value = value * 10
                        + row
                            .chars()
                            .nth(view_idx)
                            .expect("while condition")
                            .to_digit(10)
                            .expect("while condition") as u64;
                    view_idx += 1;
                }

                let mut symbol_found = false;
                for row_offset in -1i32..=1 {
                    if let Some(comp_row) = lines.get((row_idx as i32 + row_offset) as usize) {
                        for col in cmp::max(col_idx, 1) - 1..=view_idx {
                            if let Some(comp_c) = comp_row.chars().nth(col) {
                                if !comp_c.is_ascii_digit() && comp_c != '.' {
                                    symbol_found = true;
                                }
                            }
                        }
                    }
                }
                if symbol_found {
                    sum += value;
                }
            }
        }
    }
    Ok(sum)
}

pub fn calculate_gear_ratio_sum(file: &std::path::Path) -> Result<u64> {
    let mut star_adjacent = HashMap::new();
    let lines = read_lines(file)?.collect::<Result<Vec<_>, _>>()?;
    for (row_idx, row) in lines.iter().enumerate() {
        for (col_idx, c) in row.chars().enumerate() {
            // Check if a number STARTS at col_idx
            if c.is_ascii_digit()
                && (col_idx == 0
                    || row
                        .chars()
                        .nth(col_idx - 1)
                        .is_some_and(|c| !c.is_ascii_digit()))
            {
                let mut value = 0u64;
                let mut view_idx = col_idx;
                while row
                    .chars()
                    .nth(view_idx)
                    .is_some_and(|c| c.is_ascii_digit())
                {
                    value = value * 10
                        + row
                            .chars()
                            .nth(view_idx)
                            .expect("while condition")
                            .to_digit(10)
                            .expect("while condition") as u64;
                    view_idx += 1;
                }

                for row_offset in -1i32..=1 {
                    let row = (row_idx as i32 + row_offset) as usize;
                    if let Some(comp_row) = lines.get(row) {
                        for col in cmp::max(col_idx, 1) - 1..=view_idx {
                            if let Some(comp_c) = comp_row.chars().nth(col) {
                                if comp_c == '*' {
                                    let entry = star_adjacent
                                        .entry((row, col))
                                        .or_insert_with(Vec::<u64>::new);
                                    entry.push(value);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let gears = star_adjacent
        .iter()
        .filter(|(_pos, numbers)| numbers.len() == 2)
        .map(|(_key, numbers)| numbers.iter().product::<u64>())
        .sum::<u64>();
    Ok(gears)
}
