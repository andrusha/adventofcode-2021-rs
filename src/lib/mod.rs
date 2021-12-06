use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;


#[derive(Debug)]
pub enum InputError {
    IOError(io::Error),
    ParseIntError(std::num::ParseIntError),
}

impl From<io::Error> for InputError {
    fn from(e: io::Error) -> InputError {
        return InputError::IOError(e);
    }
}

impl From<std::num::ParseIntError> for InputError {
    fn from(e: std::num::ParseIntError) -> InputError {
        return InputError::ParseIntError(e);
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputError::IOError(io_error) => {
                io_error.fmt(f)
            }
            InputError::ParseIntError(parse_int_error) => {
                parse_int_error.fmt(f)
            }
        }
    }
}

pub fn read_num_lines<P>(filename: P) -> Result<Vec<i32>, InputError>
    where P: AsRef<Path>, {
    parse_lines(read_lines(filename)?)
}

fn parse_lines<F>(lines: io::Lines<io::BufReader<File>>) -> Result<Vec<F>, InputError>
    where
        F: FromStr,
        InputError: From<<F as FromStr>::Err>, {
    lines.map(|line| Ok(line?.parse()?)).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}