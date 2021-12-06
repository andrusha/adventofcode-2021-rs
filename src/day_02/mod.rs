use clap::Parser;

#[derive(Parser)]
pub struct Day2SubCmd {
    input_filename: String
}

pub fn main(args: Day2SubCmd) {
    println!(args.input_filename);
}

struct Command {

}