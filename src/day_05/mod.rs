use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
use std::str::FromStr;

use clap::Parser;
use thiserror::Error;

use crate::day_05::Day5Error::{LineParsingError, PointParsingError};

#[derive(Parser)]
pub struct Day5SubCmd {
    input_filename: String,
}

#[derive(Error, Debug)]
pub enum Day5Error {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error("Unable to parse the number")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Wrong point format")]
    PointParsingError(),

    #[error("Wrong line format")]
    LineParsingError(),
}

pub fn main(args: Day5SubCmd) -> Result<(), Day5Error> {
    let lines = read_lines(&args.input_filename)?;

    let mut intersections: HashSet<Point> = HashSet::new();

    for (i, l1) in lines.iter().enumerate() {
        for l2 in lines[i+1..].iter() {
            for p in l1.points().intersection(&l2.points()) {
                // println!("{:?} & {:?} = {:?}", l1, l2, p);
                intersections.insert(p.clone());
            }
        }
    }
    println!("Intersection points: {:?}", intersections.len());

    Ok(())
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl FromStr for Point {
    type Err = Day5Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(PointParsingError())?;
        let (x, y) = (x.parse()?, y.parse()?);

        Ok(Point { x, y })
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point
}

fn directional_range(a: i32, b: i32) -> Box<dyn Iterator<Item = i32>> {
    if a > b {
        Box::new((b ..= a).rev())
    } else {
        Box::new(a ..= b)
    }
}

impl Line {
    fn points(&self) -> HashSet<Point> {
        let xs = directional_range(self.start.x, self.end.x);
        let ys = directional_range(self.start.y, self.end.y);

        if self.is_vertical() {
            iter::repeat(self.start.x)
                .zip(ys)
                .map(|(x, y)| Point { x, y })
                .collect()
        } else if self.is_horizontal() {
            xs
                .zip(iter::repeat(self.start.y))
                .map(|(x, y)| Point { x, y })
                .collect()
        } else if self.is_diagonal() {
            xs
                .zip(ys)
                .map(|(x, y)| Point { x, y })
                .collect()
        } else {
            println!("Non-diagonal line: {:?}", self);
            HashSet::new()
        }
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_diagonal(&self) -> bool {
        (self.start.x - self.end.x).abs() == (self.start.y - self.end.y).abs()
    }
}

impl FromStr for Line {
    type Err = Day5Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").ok_or(LineParsingError())?;
        let (start, end) = (start.parse()?, end.parse()?);

        Ok(Line { start, end })
    }
}

fn read_lines(filename: &str) -> Result<Vec<Line>, Day5Error> {
    let file = File::open(filename)?;

    let lines = io::BufReader::new(file)
        .lines()
        .map(|l| l?.parse())
        .flatten()
        .collect();

    Ok(lines)
}
