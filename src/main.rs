use anyhow::anyhow;
use anyhow::Result;
use clap::Parser;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod filemanip;

#[derive(Parser)]
struct Cli {
    /// The day of the challenge
    day: u8,

    /// The challenge number
    challenge: u8,

    /// The input file
    input: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match (args.day, args.challenge) {
        (1, 1) => {
            println!("{:#?}", day1::calculate_calibration_sum(&args.input, false))
        }
        (1, 2) => {
            println!("{:#?}", day1::calculate_calibration_sum(&args.input, true))
        }
        (2, 1) => {
            println!("{:#?}", day2::calculate_valid_game_sum(&args.input))
        }
        (2, 2) => {
            println!("{:#?}", day2::calculate_game_power_sum(&args.input))
        }
        (3, 1) => {
            println!("{:#?}", day3::calculate_schematic_sum(&args.input))
        }
        (3, 2) => {
            println!("{:#?}", day3::calculate_gear_ratio_sum(&args.input))
        }
        (4, 1) => {
            println!("{:#?}", day4::calculate_scratchcard_sum(&args.input))
        }
        (4, 2) => {
            println!("{:#?}", day4::calculate_scratchcard_count(&args.input))
        }
        (5, 1) => {
            println!("{:#?}", day5::calculate_lowest_location_number(&args.input))
        }
        (5, 2) => {
            println!("{:#?}", day5::calculate_lowest_location_number_range(&args.input))
        }
        (d, c) => {
            return Err(anyhow!("Invalid day / challenge combination: {} {}!", d, c));
        }
    }
    Ok(())
}
