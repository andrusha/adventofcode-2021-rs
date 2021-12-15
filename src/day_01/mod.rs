use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;

use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct Day1SubCmd {
    input_filename: String,
    #[clap(long, default_value = "1")]
    window_width: usize,
    #[clap(long, default_value = "1")]
    window_offset: usize,
}

pub fn main(args: Day1SubCmd) -> Result<(), Day1Error> {
    let numbers = read_lines(args.input_filename)?;
    let window_sums = window_map(numbers, args.window_width, args.window_offset, |w| w.iter().sum());
    for ws in window_sums.iter() {
        println!("{}", ws);
    }
    let increases = count_increases(window_sums);
    println!("{} measurements that are larger than previous measurement", increases);

    Ok(())
}

struct IncCounter {
    increases: i32,
    prev_number: Option<i32>,
}

impl IncCounter {
    fn process_number(&self, num: i32) -> IncCounter {
        if self.prev_number.is_some() {
            if num > self.prev_number.unwrap() {
                IncCounter { increases: self.increases + 1, prev_number: Option::Some(num) }
            } else {
                IncCounter { increases: self.increases, prev_number: Option::Some(num) }
            }
        } else {
            IncCounter { increases: self.increases, prev_number: Option::Some(num) }
        }
    }

    fn empty() -> IncCounter {
        IncCounter { increases: 0, prev_number: Option::None }
    }
}

fn count_increases(nums: Vec<i32>) -> i32 {
    nums.iter().fold(IncCounter::empty(), |acc, &x| acc.process_number(x)).increases
}

struct WindowIndexer {
    cur_index: usize,
    width: usize,
    offset: usize,
    max_index: usize,
}

impl Iterator for WindowIndexer {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let end_index = self.cur_index + self.width;
        let range = self.cur_index..end_index;

        self.cur_index += self.offset;

        if end_index <= self.max_index {
            Option::Some(range)
        } else {
            Option::None
        }
    }
}

impl WindowIndexer {
    fn for_vec(width: usize, offset: usize, max_index: usize) -> WindowIndexer {
        WindowIndexer {
            cur_index: 0,
            width,
            offset,
            max_index,
        }
    }
}

fn window_map<A, B, F>(xs: Vec<A>, width: usize, offset: usize, f: F) -> Vec<B>
    where F: Fn(&[A]) -> B, {
    let slice = xs.as_slice();

    WindowIndexer
    ::for_vec(width, offset, xs.len())
        .map(|r| f(&slice[r]))
        .collect()
}

#[derive(Error, Debug)]
pub enum Day1Error {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}

fn read_lines<P>(filename: P) -> Result<Vec<i32>, Day1Error>
    where P: AsRef<Path>, {
    parse_lines(read_lines_buf(filename)?)
}

fn parse_lines<F>(lines: io::Lines<io::BufReader<File>>) -> Result<Vec<F>, Day1Error>
    where
        F: FromStr,
        Day1Error: From<<F as FromStr>::Err>, {
    lines.map(|line| Ok(line?.parse()?)).collect()
}

fn read_lines_buf<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}