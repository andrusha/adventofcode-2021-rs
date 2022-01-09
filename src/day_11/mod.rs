use clap::{Parser, ArgEnum};
use thiserror::Error;

use crate::matrix::{Matrix, MatrixError};

#[derive(ArgEnum, Clone)]
enum MatrixSize {
    S5x5,
    S10x10,
}

#[derive(Parser)]
pub struct SubCmd {
    #[clap(arg_enum)]
    matrix_size: MatrixSize,
    input_filename: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum DayError {
    #[error(transparent)]
    MatrixError(#[from] MatrixError),
}

pub fn main(args: SubCmd) -> Result<(), DayError> {
    match args.matrix_size {
        MatrixSize::S5x5 => {
            let mut matrix: Matrix<u8, 5, 5> = Matrix::from_file(&args.input_filename)?;
            simulate(&mut matrix);
        }
        MatrixSize::S10x10 => {
            let mut matrix: Matrix<u8, 10, 10> = Matrix::from_file(&args.input_filename)?;
            simulate(&mut matrix);
        }
    }

    Ok(())
}

fn simulate<const N: usize, const M: usize>(m: &mut Matrix<u8, N, M>) {
    println!("Init:\n{}\n", m);

    let mut total_flashes = 0;

    for step in 1..=1000 {
        m.map(|x| x + 1);
        explode_octopussies(m);

        let flashes = m.find_indices(|x| *x == 0).len();
        total_flashes += flashes;

        println!("Step {}, {}/{} flashes:\n{}\n", step, flashes, total_flashes, m);

        if flashes == N*M {
            println!("First full sync on step {}", step);
            break;
        }
    }
}

fn explode_octopussies<const N: usize, const M: usize>(m: &mut Matrix<u8, N, M>) {
    let mut visit = m.find_indices(|&x| x > 9);

    while let Some(cur) = visit.pop() {
        if *m.get(&cur) == 0 { continue }
        m.set(&cur, 0);

        for npos in m.all_neighbors(&cur) {
            let v = *m.get(&npos);
            if v == 0 { continue }

            let nextval = v + 1;
            m.set(&npos, nextval);
            if nextval > 9 {
                visit.push(npos);
            }
        }
    }
}