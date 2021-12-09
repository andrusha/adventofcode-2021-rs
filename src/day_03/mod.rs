use std::fs::File;
use std::io::{self, BufRead};

use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;
use clap::Parser;
use thiserror::Error;
use bitvec::prelude::*;

#[derive(Parser)]
pub struct Day3SubCmd {
    input_filename: String,
}

pub fn main(args: Day3SubCmd) {
    match read_lines(args.input_filename.as_str()) {
        Ok(bits) => {
            let gamma: BitVec<Msb0> = bits.iter().map(|b| b.count_ones() > b.len() / 2).collect();
            let epsilon: BitVec<Msb0> = !gamma.clone();

            let gamma_v = gamma.load::<u32>();
            let epsilon_v = epsilon.load::<u32>();

            println!("Gamma: {}, {}", gamma_v, gamma);
            println!("Epsilon: {}, {}", epsilon_v, epsilon);
            println!("Power consumption: {}", gamma_v * epsilon_v);

            let oxygen_rating: BitVec<Msb0> = progressive_filter(&bits, true);
            let oxygen_rating_v = oxygen_rating.load::<u32>();

            let co2_rating: BitVec<Msb0> = progressive_filter(&bits, false);
            let co2_rating_v = co2_rating.load::<u32>();

            println!("Oxygen rating: {}, {}", oxygen_rating_v, oxygen_rating);
            println!("CO2 rating: {}, {}", co2_rating_v, co2_rating);
            println!("Support rating: {}", oxygen_rating_v * co2_rating_v);
        }
        Err(e) => {
            eprintln!("Error reading file: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn progressive_filter(bits: &[BitVec<>], most_common: bool) -> BitVec<Msb0> {
    let mut row_mask: BitVec<> = BitVec::repeat(true, bits[0].len());

    for col in bits.iter() {
        let intersect = row_mask.clone() & col.clone();
        let intersect_zeros = intersect.count_zeros() - row_mask.count_zeros();
        let common_ones = intersect.count_ones() >= intersect_zeros;
        let common_zeros = intersect.count_ones() < intersect_zeros;

        if (common_ones && most_common) || (common_zeros && !most_common) {
            row_mask = intersect;
        } else {
            row_mask &= !intersect;
        }

        if row_mask.count_ones() == 1 {
            break;
        }
    }

    let index = row_mask.leading_zeros();

    bits.iter().map(|b| b[index]).collect()
}

#[derive(Error, Debug)]
enum InputError {
    #[error(transparent)]
    IOError(#[from] io::Error),
}

fn read_lines(filename: &str) -> Result<Vec<BitVec>, InputError> {
    parse_lines(read_lines_buf(filename)?)
}

fn parse_lines(lines: io::Lines<io::BufReader<File>>) -> Result<Vec<BitVec>, InputError> {
    let mut res: Vec<BitVec> = vec![];

    for line in lines {
        for (i, ch) in line?.char_indices() {
            let b: bool = ch == '1';
            if res.len() < i + 1 {
                res.push(BitVec::new());
            }

            res[i].push(b);
        }
    }

    Ok(res)
}

fn read_lines_buf(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}