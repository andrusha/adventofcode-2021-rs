use clap::Parser;

use crate::day_01::Day1SubCmd;
use crate::day_02::Day2SubCmd;

pub mod day_01;
pub mod day_02;

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
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Day1(day_1) => day_01::main(day_1),
        SubCommand::Day2(day_2) => day_02::main(day_2)
    }
}