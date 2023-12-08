use std::collections::{HashMap, VecDeque};

use anyhow::{Context, Result};

use crate::filemanip::read_lines;

pub fn calculate_zzz_steps(file: &std::path::Path) -> Result<u64> {
    let mut lines = read_lines(file)
        .context(format!("Failed to read file {:#?}", file))?
        .flatten()
        .collect::<VecDeque<_>>();
    let instructions = lines.pop_front().unwrap();
    let mut instructions = instructions.chars().cycle();
    let _blank = lines.pop_front().unwrap();
    let mut nodes = HashMap::new();
    for line in lines {
        let (node_name, node_steps) = line.trim().split_once(" = ").unwrap();
        let (left, right) = node_steps.trim().split_once(", ").unwrap();
        let left = left.replace("(", "");
        let right = right.replace(")", "");
        nodes.insert(node_name.to_owned(), (left, right));
    }

    let mut current = "AAA".to_owned();
    let mut count = 0u64;
    while current != "ZZZ" {
        let instr = instructions.next().unwrap();
        current = match instr {
            'L' => nodes.get(&current).unwrap().0.to_owned(),
            'R' => nodes.get(&current).unwrap().1.to_owned(),
            _ => panic!("Malformed instruction"),
        };
        count += 1;
    }
    Ok(count)
}
