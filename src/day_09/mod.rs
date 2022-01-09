use clap::Parser;
use thiserror::Error;

use crate::matrix::{Matrix, MatrixError, Pos};

#[derive(Parser)]
pub struct Day9SubCmd {
    test_input_filename: String,
    prod_input_filename: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum Day9Error {
    #[error(transparent)]
    MatrixError(#[from] MatrixError),
}

pub fn main(args: Day9SubCmd) -> Result<(), Day9Error> {
    process_matrix::<5, 10>(args.test_input_filename.as_str())?;
    println!();
    process_matrix::<100, 100>(args.prod_input_filename.as_str())?;

    Ok(())
}

fn process_matrix<const N: usize, const M: usize>(filename: &str) -> Result<(), Day9Error> {
    let matrix: Matrix<u8, N, M> = Matrix::from_file(filename)?;
    let minimas = find_local_minimas(&matrix);
    println!("Local minimas: {:?}", minimas);
    println!("Risk level for test matrix: {}", risk_level::<N, M>(&minimas, &matrix));

    let mut cavern_sizes = vec![];

    for pos in minimas {
        let cavern = matrix.fill(&pos, |x| if *x != 9 { Some(*x) } else { None });
        let cavern_size = cavern.find_indices(|f| f.is_some()).len();
        cavern_sizes.push(cavern_size);
        println!("Cavern of size: {} found", cavern_size);
    }

    cavern_sizes.sort();
    let three_basins = cavern_sizes[cavern_sizes.len() - 3..].to_vec();
    println!("Three largest basins: {:?}", &three_basins);
    println!("Basin metric: {}", three_basins.iter().fold(1, |acc, &y| acc * y));

    Ok(())
}

fn risk_level<const N: usize, const M: usize>(minima_coords: &[Pos<N, M>], matrix: &Matrix<u8, N, M>) -> u32 {
    minima_coords.iter().map(|pos| (*matrix.get(pos) as u32) + 1).sum()
}

fn find_local_minimas<T, const N: usize, const M: usize>(matrix: &Matrix<T, N, M>) -> Vec<Pos<N, M>>
    where T: PartialOrd + Copy {
    fn is_local_minima<T: PartialOrd + Copy>(x: &T, neighbours: &[&T]) -> bool {
        neighbours.iter().all(|&n| n > x)
    }

    let minimas_matrix = matrix.map_surroundings(is_local_minima);

    minimas_matrix.find_indices(|x| *x)
}
