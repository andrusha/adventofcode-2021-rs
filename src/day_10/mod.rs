use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::BufRead;

use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct Day10SubCmd {
    input_filename: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum Day10Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("Incorrect brace sequence encountered")]
    BalancingError,
}

pub fn main(args: Day10SubCmd) -> Result<(), Day10Error> {
    let lines = io::BufReader::new(File::open(args.input_filename)?).lines();

    let mut score = 0;
    let mut autocomplete_scores: Vec<i64> = vec![];

    for line in lines {
        let mut incomplete = false;
        let mut balancer = Balancer::default();

        for char in line?.chars() {
            if balancer.process(char).is_err() {
                incomplete = true;
                match Brace::from(char) {
                    Brace::Round => score += 3,
                    Brace::Square => score += 57,
                    Brace::Curly => score += 1197,
                    Brace::Angled => score += 25137,
                }
            }
        }

        if !incomplete && !balancer.braces.is_empty() {
            autocomplete_scores.push(balancer.braces.iter().rev().map(|b| match b {
                Brace::Round => 1,
                Brace::Square => 2,
                Brace::Curly => 3,
                Brace::Angled => 4,
            }).fold(0, |acc, s| acc * 5 + s));
        }
    }

    println!("Total score of corrupted lines: {}", score);

    autocomplete_scores.sort();
    println!("Middle score {}", autocomplete_scores[autocomplete_scores.len()/2]);

    Ok(())
}

#[derive(Default)]
struct Balancer {
    braces: VecDeque<Brace>
}

#[derive(PartialEq)]
enum Brace {
    Round,
    Square,
    Curly,
    Angled
}

impl From<char> for Brace {
    fn from(c: char) -> Self {
        match c {
            '(' | ')' => Brace::Round,
            '[' | ']' => Brace::Square,
            '{' | '}' => Brace::Curly,
            '<' | '>' => Brace::Angled,
            _ => panic!("Unknown brace type")
        }
    }
}


impl Balancer {
    fn process(&mut self, brace: char) -> Result<(), Day10Error> {
        let kind = Brace::from(brace);

        match brace {
            '(' | '[' | '{' | '<' => {
                self.braces.push_back(kind);
                Ok(())
            },
            ')' | ']' | '}' | '>' => {
                match self.braces.pop_back() {
                    Some(c) if c == kind => Ok(()),
                    _ => Err(Day10Error::BalancingError)
                }
            },
            _ => panic!("Unknown brace symbol")
        }
    }
}