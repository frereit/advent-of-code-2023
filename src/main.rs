use anyhow::anyhow;
use anyhow::Result;
use clap::Parser;

mod day1;
mod filemanip;

#[derive(Parser)]
struct Cli {
    /// The challenge number
    challenge: u8,

    /// The input file
    input: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.challenge {
        1 => {
            println!("{:#?}", day1::calculate_calibration_sum(&args.input, false))
        }
        2 => {
            println!("{:#?}", day1::calculate_calibration_sum(&args.input, true))
        }
        _ => {
            return Err(anyhow!("Invalid challenge number: {}!", args.challenge));
        }
    }
    Ok(())
}
