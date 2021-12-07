use std::{io, process};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct Day2SubCmd {
    input_filename: String,
}

pub fn main(args: Day2SubCmd) {
    match read_lines(args.input_filename) {
        Ok(commands) => {
            let pos = commands.iter().fold(Position::default(), |p, c| p.execute_command(c));
            println!("Resulting position {:?}, multiply {}", pos, pos.horizontal * pos.depth);
        }
        Err(e) => {
            eprintln!("Error reading file: {:?}", e);
            process::exit(1);
        }
    }
}

#[derive(Debug, Default)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn execute_command(&self, cmd: &Command) -> Position {
        match cmd.direction {
            Direction::Forward =>
                Position {
                    horizontal: self.horizontal + cmd.magnitude,
                    depth: self.depth + self.aim * cmd.magnitude,
                    aim: self.aim,
                },
            Direction::Up =>
                Position {
                    horizontal: self.horizontal,
                    depth: self.depth,
                    aim: self.aim - cmd.magnitude,
                },
            Direction::Down =>
                Position {
                    horizontal: self.horizontal,
                    depth: self.depth,
                    aim: self.aim + cmd.magnitude,
                },
        }
    }
}

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(ParseCommandError::ParseDirectionError)
        }
    }
}

#[derive(Error, Debug)]
enum ParseCommandError {
    #[error("Unable to parse the number")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Command must have format '<direction> <magnitude>'")]
    UnsplittableCommandError,

    #[error("Unable to parse direction")]
    ParseDirectionError,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    magnitude: i32,
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((d, m)) => {
                Ok(Command {
                    direction: d.parse()?,
                    magnitude: m.parse()?,
                })
            }
            None => Err(ParseCommandError::UnsplittableCommandError)
        }
    }
}

#[derive(Error, Debug)]
enum InputError {
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    ParseCommandError(#[from] ParseCommandError),
}

fn read_lines<P>(filename: P) -> Result<Vec<Command>, InputError>
    where P: AsRef<Path>, {
    parse_lines(read_lines_buf(filename)?)
}

fn parse_lines<F>(lines: io::Lines<io::BufReader<File>>) -> Result<Vec<F>, InputError>
    where
        F: FromStr,
        InputError: From<<F as FromStr>::Err>, {
    lines.map(|line| Ok(line?.parse()?)).collect()
}

fn read_lines_buf<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}