use clap::Parser;
use thiserror::Error;

use crate::day_01::Day1SubCmd;
use crate::day_02::Day2SubCmd;
use crate::day_03::Day3SubCmd;
use crate::day_04::Day4SubCmd;
use crate::day_05::Day5SubCmd;
use crate::day_06::Day6SubCmd;
use crate::day_07::Day7SubCmd;
use crate::day_08::Day8SubCmd;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

#[derive(Parser)]
#[clap(version = "0.1", author = "Andrew Korzhuev <korzhuev@andrusha.me>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Day1(Day1SubCmd),
    Day2(Day2SubCmd),
    Day3(Day3SubCmd),
    Day4(Day4SubCmd),
    Day5(Day5SubCmd),
    Day6(Day6SubCmd),
    Day7(Day7SubCmd),
    Day8(Day8SubCmd),
}

#[derive(Error, Debug)]
pub enum AdventError {
    #[error(transparent)]
    Day01Error(#[from] day_01::Day1Error),

    #[error(transparent)]
    Day02Error(#[from] day_02::Day2Error),

    #[error(transparent)]
    Day03Error(#[from] day_03::Day3Error),

    #[error(transparent)]
    Day04Error(#[from] day_04::Day4Error),

    #[error(transparent)]
    Day05Error(#[from] day_05::Day5Error),

    #[error(transparent)]
    Day06Error(#[from] day_06::Day6Error),

    #[error(transparent)]
    Day07Error(#[from] day_07::Day7Error),

    #[error(transparent)]
    Day08Error(#[from] day_08::Day8Error),
}


fn main() -> Result<(), AdventError> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Day1(args) => day_01::main(args)?,
        SubCommand::Day2(args) => day_02::main(args)?,
        SubCommand::Day3(args) => day_03::main(args)?,
        SubCommand::Day4(args) => day_04::main(args)?,
        SubCommand::Day5(args) => day_05::main(args)?,
        SubCommand::Day6(args) => day_06::main(args)?,
        SubCommand::Day7(args) => day_07::main(args)?,
        SubCommand::Day8(args) => day_08::main(args)?,
    }

    Ok(())
}