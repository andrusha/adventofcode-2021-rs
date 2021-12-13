use std::fs::File;
use std::io::{self, Read};
use std::str::FromStr;

use clap::Parser;
use thiserror::Error;

const BOARD_SIZE: usize = 5;

#[derive(Parser)]
pub struct Day4SubCmd {
    input_filename: String,
}

pub fn main(args: Day4SubCmd) {
    match read_lines(args.input_filename.as_str()) {
        Ok((guesses, mut boards)) => {
            println!("Guesses: {:?}", guesses);
            for (i, b) in boards.iter().enumerate() {
                println!("Board {}:\n{:?}", i, b);
            }

            let mut winning_boards: Vec<bool> = vec![false; boards.len()];

            for guess in guesses.guesses {
                for (i, b) in boards.iter_mut().enumerate() {
                    b.mark_number(guess);

                    if b.is_winning() && !winning_boards[i] {
                        winning_boards[i] = true;
                        println!("Winning board: {:?}", b);
                        let sum_unmarked = b.sum_unmarked();
                        println!("Sum unmarked: {}, Winning number: {}, Score: {}", sum_unmarked, guess, sum_unmarked * guess);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {:?}", e);
            std::process::exit(1);
        }
    }
}

#[derive(Default, Debug)]
struct BingoBoard {
    board: [[usize; BOARD_SIZE]; BOARD_SIZE],
    marks: [[bool; BOARD_SIZE]; BOARD_SIZE],
}

impl BingoBoard {
    fn mark_number(&mut self, num: usize) {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j] == num {
                    self.marks[i][j] = true;
                }
            }
        }
    }

    fn sum_unmarked(&self) -> usize {
        let mut sum = 0;
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if !self.marks[i][j] {
                    sum += self.board[i][j];
                }
            }
        }

        sum
    }

    fn is_winning(&self) -> bool {
        for i in 0..BOARD_SIZE {
            let mut is_winning_col = true;
            let mut is_winning_row = true;

            for j in 0..BOARD_SIZE {
                is_winning_col = is_winning_col && self.marks[i][j];
                is_winning_row = is_winning_row && self.marks[j][i];
            }

            if is_winning_col || is_winning_row {
                return true;
            }
        }

        false
    }
}

#[derive(Error, Debug)]
enum ParseBingoBoardError {
    #[error("Unable to parse the number")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Not enough rows to fill the board")]
    TooFewRowsError,

    #[error("Too many columns to fit the board")]
    TooManyColumnsError,

    #[error("Too many rows to fit the board")]
    TooManyRowsError
}

impl BingoBoard {
    fn from_lines(lines: &[&str]) -> Result<Self, ParseBingoBoardError> {
        if lines.len() < BOARD_SIZE { return Err(ParseBingoBoardError::TooFewRowsError) }
        else if lines.len() > BOARD_SIZE { return Err(ParseBingoBoardError::TooManyRowsError) }

        let mut board = BingoBoard::default();

        for (i, row) in lines.iter().enumerate() {
            for (j, col) in row.split_whitespace().enumerate() {
                if j >= BOARD_SIZE { return Err(ParseBingoBoardError::TooManyColumnsError) }

                board.board[i][j] = col.parse()?;
            }
        }

        Ok(board)
    }
}

#[derive(Debug)]
struct Guesses {
    guesses: Vec<usize>
}

impl FromStr for Guesses {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guesses = vec![];
        for n in s.split(',') {
            guesses.push(n.parse()?);
        }

        Ok(Guesses { guesses })
    }
}

#[derive(Error, Debug)]
enum InputError {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error(transparent)]
    ParseBingoBoardError(#[from] ParseBingoBoardError),

    #[error("Unable to parse the number")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("File is incorrectly     formatted")]
    FileParsingError
}

fn read_lines(filename: &str) -> Result<(Guesses, Vec<BingoBoard>), InputError> {
    let mut lines: String = String::new();
    File::open(filename)?.read_to_string(&mut lines)?;
    let lines: Vec<&str> = lines.lines().collect();

    if lines.len() < 2 {
        return Err(InputError::FileParsingError)
    }

    let guesses: Guesses = lines[0].parse()?;
    let mut boards: Vec<BingoBoard> = vec![];

    // skip separator line, assume each board has separator at the end as well
    for chunks in lines[1..].chunks(BOARD_SIZE + 1) {
        let chunks = &chunks[1..];
        boards.push(BingoBoard::from_lines(chunks)?);
    }

    Ok((guesses, boards))
}
