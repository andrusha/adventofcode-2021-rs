use std::cmp::{min, Ordering, Reverse};
use std::collections::BinaryHeap;
use std::usize;

use clap::{ArgEnum, Parser};
use hashbrown::HashSet;
use thiserror::Error;

use crate::matrix::{Matrix, MatrixError, Pos};

#[derive(ArgEnum, Clone)]
enum MatrixSize {
    S10x10,
    S100x100,
}

#[derive(Parser)]
pub struct SubCmd {
    #[clap(arg_enum)]
    matrix_size: MatrixSize,
    #[clap(long, parse(from_flag))]
    large_cave: bool,
    input_filename: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum DayError {
    #[error(transparent)]
    MatrixError(#[from] MatrixError),
}

pub fn main(args: SubCmd) -> Result<(), DayError> {
    match (args.matrix_size, args.large_cave) {
        (MatrixSize::S10x10, false) => {
            solve::<10, 10>(&args.input_filename)?;
        }
        (MatrixSize::S10x10, true) => {
            solve::<10, 50>(&args.input_filename)?;
        }
        (MatrixSize::S100x100, false) => {
            solve::<100, 100>(&args.input_filename)?;
        }
        (MatrixSize::S100x100, true) => {
            solve::<100, 500>(&args.input_filename)?;
        }
    }

    Ok(())
}

fn solve<const N: usize, const M: usize>(fname: &str) -> Result<(), DayError> {
    let matrix: Matrix<u8, N, N> = Matrix::from_file(fname)?;
    let matrix: Matrix<u8, M, M> = extend_matrix(&matrix);
    let end_pos = Pos::new(M - 1, M - 1);

    let dist = distances(&matrix, &end_pos);
    println!("Minimal risk: {}", dist.get(&end_pos));

    Ok(())
}

fn extend_matrix<const N: usize, const M: usize>(m: &Matrix<u8, N, N>) -> Matrix<u8, M, M> {
    let mut r: Matrix<u8, M, M> = Matrix::new(0);
    for i in 0..M {
        let imul = (i / N) as u8;
        for j in 0..M {
            let jmul = (j / N) as u8;
            let v = *m.get(&Pos::new(i % N, j % N));
            let v = v + imul + jmul;
            r.set(&Pos::new(i, j), v % 10 + v / 10);
        }
    }

    r
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct WeightedPos<const N: usize> {
    pos: Pos<N, N>,
    weight: usize,
}

impl<const N: usize> PartialOrd for WeightedPos<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.weight {
            w if w > other.weight => Some(Ordering::Greater),
            w if w == other.weight => Some(Ordering::Equal),
            w if w < other.weight => Some(Ordering::Less),
            _ => None
        }
    }
}

impl<const N: usize> Ord for WeightedPos<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/**
Implements Dijkstra algorithm imagining the matrix as a set of connected nodes with
the front moving towards the bottom-right corner
 **/
fn distances<const N: usize>(m: &Matrix<u8, N, N>, finish_pos: &Pos<N, N>) -> Matrix<usize, N, N> {
    let initial_pos = Pos::new(0, 0);

    let mut distances = Matrix::new(usize::MAX);
    distances.set(&initial_pos, 0);

    let mut visited = HashSet::with_capacity(N * N);
    let mut front = BinaryHeap::with_capacity(N * N);
    front.push(Reverse(WeightedPos { pos: initial_pos, weight: 0 }));

    while let Some(Reverse(WeightedPos { pos: cur, weight: cur_dist })) = front.pop() {
        if visited.contains(&cur) {
            continue;
        } else if &cur == finish_pos {
            break;
        }
        visited.insert(cur);

        for pos in m.direct_neighbours(&cur) {
            let min_dist =
                min(
                    *distances.get(&pos),
                    *m.get(&pos) as usize + cur_dist);
            distances.set(&pos, min_dist);

            front.push(Reverse(WeightedPos { pos, weight: min_dist }));
        }
    }

    distances
}
