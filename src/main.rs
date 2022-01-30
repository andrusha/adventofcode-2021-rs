#![feature(mixed_integer_ops)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

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
use crate::day_09::Day9SubCmd;
use crate::day_10::Day10SubCmd;

mod matrix;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;

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
    Day9(Day9SubCmd),
    Day10(Day10SubCmd),
    Day11(day_11::SubCmd),
    Day12(day_12::SubCmd),
    Day13(day_13::SubCmd),
    Day14(day_14::SubCmd),
    Day15(day_15::SubCmd),
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

    #[error(transparent)]
    Day09Error(#[from] day_09::Day9Error),

    #[error(transparent)]
    Day10Error(#[from] day_10::Day10Error),

    #[error(transparent)]
    Day11Error(#[from] day_11::DayError),

    #[error(transparent)]
    Day12Error(#[from] day_12::DayError),

    #[error(transparent)]
    Day13Error(#[from] day_13::DayError),

    #[error(transparent)]
    Day14Error(#[from] day_14::DayError),

    #[error(transparent)]
    Day15Error(#[from] day_15::DayError),
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
        SubCommand::Day9(args) => day_09::main(args)?,
        SubCommand::Day10(args) => day_10::main(args)?,
        SubCommand::Day11(args) => day_11::main(args)?,
        SubCommand::Day12(args) => day_12::main(args)?,
        SubCommand::Day13(args) => day_13::main(args)?,
        SubCommand::Day14(args) => day_14::main(args)?,
        SubCommand::Day15(args) => day_15::main(args)?,
    }

    Ok(())
}