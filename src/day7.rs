use std::{cmp, collections::HashMap, str::FromStr};

use anyhow::{Context, Error, Result};

use crate::filemanip::read_lines;

const CARDS_PART_ONE: &[char] = &[
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CARDS_PART_TWO: &[char] = &[
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<u8>,
    r#type: u8,
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 5 {
            return Err(Error::msg("Invalid string length supplied"));
        }
        let cards = s
            .chars()
            .map(|c| CARDS_PART_TWO.iter().position(|x| *x == c).unwrap() as u8)
            .collect::<Vec<_>>();
        let mut counts = HashMap::new();
        for card in &cards {
            if !counts.contains_key(card) {
                counts.insert(*card, cards.iter().filter(|x| *x == card).count());
            }
        }
        let r#type;
        if counts.len() == 1 {
            r#type = 6; // Five of a kind
        } else if counts.len() == 2 && counts.iter().filter(|(_, v)| **v == 4).count() == 1 {
            r#type = 5; // Four of a kind
        } else if counts.len() == 2 {
            r#type = 4; // Full house
        } else if counts.len() == 3 && counts.iter().filter(|(_, v)| **v == 3).count() == 1 {
            r#type = 3; // Three of a kind
        } else if counts.len() == 3 {
            r#type = 2; // Two pair
        } else if counts.len() == 4 {
            r#type = 1; // One pair
        } else {
            r#type = 0; // High card
        }
        Ok(Hand { cards, r#type })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self == other {
            cmp::Ordering::Equal
        } else if self.r#type > other.r#type {
            cmp::Ordering::Greater
        } else if self.r#type < other.r#type {
            cmp::Ordering::Less
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

fn replace_jokers(hand: &str) -> Hand {
    let mut real_hand = hand.parse::<Hand>().unwrap();
    let mut max_types = vec![];
    for sub in CARDS_PART_TWO {
        max_types.push(hand.replace('J', &sub.to_string()).parse::<Hand>().unwrap());
    }
    let max_type = max_types.iter().map(|hand| hand.r#type).max().unwrap();
    real_hand.r#type = max_type;
    real_hand
}

pub fn calculate_winning_hands(file: &std::path::Path, jokers: bool) -> Result<u64> {
    let mut cards = read_lines(file)
        .context(format!("Failed to read file {:#?}", file))?
        .flatten()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let parsed_hand = if jokers {
                replace_jokers(hand)
            } else {
                hand.parse().unwrap()
            };
            (parsed_hand, bid.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();
    cards.sort_by(|a, b| a.0.cmp(&b.0));
    dbg!(&cards);
    Ok(cards
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum())
}
