use std::collections::HashSet;
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

    #[error("Unknown dimension specifier")]
    UnknownDimensionError,

    #[error(transparent)]
    ParsingError(#[from] nom::error::Error<String>),
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

impl TryFrom<(&str, &str)> for Point {
    type Error = DayError;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        Ok(Point { x: value.0.parse()?, y: value.1.parse()? })
    }
}

#[derive(Debug)]
pub enum Fold {
    X(usize),
    Y(usize),
}

impl TryFrom<(char, &str)> for Fold {
    type Error = DayError;

    fn try_from(value: (char, &str)) -> Result<Self, Self::Error> {
        match value.0 {
            'x' => Ok(Fold::X(value.1.parse()?)),
            'y' => Ok(Fold::Y(value.1.parse()?)),
            _ => Err(DayError::UnknownDimensionError)
        }
    }
}

pub fn main(args: SubCmd) -> Result<(), DayError> {
    let (points, folds) = reader::read_file(&args.input_filename)?;
    let mut points = HashSet::from_iter(points);

    for f in folds.iter() {
        points = fold(&points, f);
        if points.len() < 200 {
            viz(&points);
        }
        println!("Points after fold {:?}: {}", f, points.len());
        println!();
    }

    Ok(())
}

fn viz(points: &HashSet<Point>) {
    let max_x = points.iter().max_by_key(|p| p.x).unwrap().x;
    let max_y = points.iter().max_by_key(|p| p.y).unwrap().y;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn fold(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let (top, bottom) = split_on_fold(points, fold);
    let mirrored = mirror_on_fold(&bottom, fold);

    top.union(&mirrored).map(|p| p.to_owned()).collect()
}

fn mirror_on_fold(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let max_x = points.iter().max_by_key(|p| p.x).unwrap().x;
    let max_y = points.iter().max_by_key(|p| p.y).unwrap().y;

    points.iter().map(|p| {
        match fold {
            Fold::X(x) => Point { x: x - (p.x - (max_x - x)), y: p.y },
            Fold::Y(y) => Point { x: p.x, y: y - (p.y - (max_y - y)) }
        }
    }).collect()
}

fn split_on_fold(points: &HashSet<Point>, fold: &Fold) -> (HashSet<Point>, HashSet<Point>) {
    points.iter().partition(|p| {
        match fold {
            Fold::X(x) => p.x < *x,
            Fold::Y(y) => p.y < *y
        }
    })
}

mod reader {
    use nom::{Finish, IResult};
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, multispace1, one_of};
    use nom::combinator::map_res;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair};

    use crate::day_13::DayError;

    use super::{Fold, Point};

    pub fn read_file(fname: &str) -> Result<(Vec<Point>, Vec<Fold>), DayError> {
        Ok(parse_input(&std::fs::read_to_string(fname)?)
            .map(|(_, r)| r)
            .map_err(|e| e.to_owned())
            .finish()?)
    }

    fn parse_input(input: &str) -> IResult<&str, (Vec<Point>, Vec<Fold>)> {
        separated_pair(points, multispace1, folds)(input)
    }

    fn points(input: &str) -> IResult<&str, Vec<Point>> {
        separated_list1(multispace1, point)(input)
    }

    fn point(input: &str) -> IResult<&str, Point> {
        map_res(
            separated_pair(digit1, tag(","), digit1),
            Point::try_from,
        )(input)
    }

    fn folds(input: &str) -> IResult<&str, Vec<Fold>> {
        separated_list1(multispace1, fold)(input)
    }

    fn fold(input: &str) -> IResult<&str, Fold> {
        preceded(
            tag("fold along "),
            fold_value,
        )(input)
    }

    fn fold_value(input: &str) -> IResult<&str, Fold> {
        map_res(
            separated_pair(
                one_of("xy"),
                tag("="),
                digit1,
            ),
            Fold::try_from,
        )(input)
    }
}