use std::cmp::{min, Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::usize;
use clap::{Parser, ArgEnum};
use thiserror::Error;
use crate::matrix::{If, Matrix, MatrixError, Pos, True};

#[derive(ArgEnum, Clone)]
enum MatrixSize {
    S10x10,
    S100x100
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

fn solve<const N: usize, const M: usize>(fname: &str) -> Result<(), DayError>
    where If<{N <= M}>: True {
    let input_matrix: Matrix<u8, N, N> = Matrix::from_file(fname)?;
    let mut matrix: Matrix<u8, M, M> = Matrix::new(0);
    if N == M {
        for pos in input_matrix.index_iter() {
            matrix.set(&pos.to(), *input_matrix.get(&pos));
        }
    } else if N < M {
        for i in 0..M {
            let imul = (i / N) as u8;
            for j in 0..M {
                let jmul = (j / N) as u8;
                let v = *input_matrix.get(&Pos::new(i % N, j % N));
                let v = v + imul + jmul;
                matrix.set(&Pos::new(i, j), v % 10 + v / 10 );
            }
        }
    } else {
        panic!("Don't know how to shrink the cave")
    }

    let end_pos = Pos::new(M - 1, M - 1);
    // println!("Intial matrix:\n{}\n", matrix);

    let dist = distances(&matrix, &end_pos);
    println!("Risk: {}", dist.get(&end_pos));
    // println!("Distances matrix:\n{}\n", dist);
    // let paths = shortest_paths(&dist, &end_pos);
    // println!("Total shortest paths: {}", paths.len());
    //
    // let risks: Vec<usize> = paths
    //     .iter()
    //     .map(|path| {
    //         path
    //             .iter()
    //             .map(|pos| *matrix.get(pos) as usize)
    //             .sum()
    //     })
    //     .collect();
    //
    // let min_risk = risks.iter().min().unwrap();
    // println!("Minimal risk: {}", min_risk);

    Ok(())
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct WeightedPos<const N: usize> {
    pos: Pos<N, N>,
    weight: usize
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

    let mut visited = HashSet::with_capacity(N*N);
    let mut front = BinaryHeap::with_capacity(N*N);
    front.push(Reverse(WeightedPos { pos: initial_pos, weight: 0 }));

    while let Some(Reverse(WeightedPos { pos: cur, weight: cur_dist})) = front.pop() {
        if visited.contains(&cur) {
            continue
        } else if &cur == finish_pos {
            break
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

fn shortest_paths<const N: usize>(distances: &Matrix<usize, N, N>, end_pos: &Pos<N, N>) -> Vec<VecDeque<Pos<N, N>>> {
    let mut result = vec![];
    let mut paths = vec![];
    let mut path = VecDeque::with_capacity(N*N);
    path.push_back(*end_pos);
    paths.push(path);

    let start_pos = Pos::new(0, 0);

    while let Some(path) = paths.pop() {
        let cur = path.back().unwrap();

        let min_dist =
            distances
                .direct_neighbours(cur)
                .map(|p| distances.get(&p))
                .min()
                .unwrap();

        let possible_paths =
            distances
                .direct_neighbours(cur)
                .filter(|p| distances.get(p) == min_dist);

        for pos in possible_paths {
            let mut new_path = path.clone();
            if pos == start_pos {
                result.push(new_path);
            } else {
                new_path.push_back(pos);
                paths.push(new_path);
            }
        }
    }

    result
}