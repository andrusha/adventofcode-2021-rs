use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct SubCmd {
    input_filename: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum DayError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("Unable to parse the number")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Incorrec file format")]
    ParsingError
}

struct Point {
    x: usize,
    y: usize
}

enum Fold {
    X(usize),
    Y(usize)
}

pub fn main(args: SubCmd) -> Result<(), DayError> {
    let (points, folds) = read_prob(&args.input_filename)?;

    Ok(())
}

fn read_prob(fname: &str) -> Result<(Vec<Point>, Vec<Fold>), DayError> {
    if let Some((points, folds)) = std::fs::read_to_string(fname)?.split_once("\n\n") {
        return Ok((parse_points(points)?, parse_folds(folds)?))
    }

    Err(DayError::ParsingError)
}

fn parse_points(inp: &str) -> Result<Vec<Point>, DayError> {
    Ok(inp
        .lines()
        .flat_map(|l| l.split_once(','))
        .map(|(x, y)| (x.parse()?, y.parse()?))
        .map(|(x, y)| Point { x, y })
        .collect()
    )
}

fn parse_folds(inp: &str) -> Result<Vec<Fold>, DayError> {
    Ok(
        inp.
            lines()
            .map(|l| {
                let pref_len = "fold along x=".len();

                if l.starts_with("fold along x=") {
                    Fold::X(l[pref_len..].parse()?)
                } else {
                    Fold::Y(l[pref_len..].parse()?)
                }
            })
            .collect()
    )
}