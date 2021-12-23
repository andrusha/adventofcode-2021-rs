use std::collections::HashSet;
use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct Day9SubCmd {
    test_input_filename: String,
    prod_input_filename: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum Day9Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("Unable to parse the number")]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub fn main(args: Day9SubCmd) -> Result<(), Day9Error> {
    process_matrix::<5, 10>(args.test_input_filename.as_str())?;
    println!();
    process_matrix::<100, 100>(args.prod_input_filename.as_str())?;

    Ok(())
}

fn process_matrix<const N: usize, const M: usize>(filename: &str) -> Result<(), Day9Error> {
    let matrix = reader::matrix::<N, M>(filename)?;
    let minimas = find_local_minimas(&matrix);
    println!("Local minimas: {:?}", minimas);
    println!("Risk level for test matrix: {}", risk_level::<N, M>(&minimas, &matrix));

    let mut cavern_sizes = vec![];

    for (i, j) in minimas {
        let cavern = matrix.fill(i, j, |x| if *x != 9 { Some(*x) } else { None });
        let cavern_size = cavern.find_indices(|f| f.is_some()).len();
        cavern_sizes.push(cavern_size);
        println!("Cavern of size: {} found", cavern_size);
    }

    cavern_sizes.sort();
    let three_basins = cavern_sizes[cavern_sizes.len() - 3 ..].to_vec();
    println!("Three largest basins: {:?}", &three_basins);
    println!("Basin metric: {}", three_basins.iter().fold(1, |acc, &y| acc * y));

    Ok(())
}

fn risk_level<const N: usize, const M: usize>(minima_coords: &[(usize, usize)], matrix: &Matrix<u8, N, M>) -> u32 {
    minima_coords.iter().map(|(i, j)| (*matrix.get(*i, *j).expect("Incorrect matrix sizing") as u32) + 1).sum()
}

fn find_local_minimas<T: PartialOrd + Copy, const N: usize, const M: usize>(matrix: &Matrix<T, N, M>) -> Vec<(usize, usize)> {
    fn is_local_minima<T: PartialOrd + Copy>(x: &T, up: Option<&T>, down: Option<&T>, left: Option<&T>, right: Option<&T>) -> bool {
        fn minima_compare<T: PartialOrd + Copy>(x: &T, y: Option<&T>) -> bool {
            y.map_or_else(|| true, |y| y > x)
        }

        minima_compare(x, up) && minima_compare(x, down) && minima_compare(x, left) && minima_compare(x, right)
    }

    let minimas_matrix = matrix.map_surroundings(is_local_minima);

    minimas_matrix.find_indices(|x| *x)
}

#[derive(Debug)]
pub struct Matrix<T, const N: usize, const M: usize> {
    m: [[T; M]; N],
}

struct MatrixIndexIterator<const N: usize, const M: usize> {
    i: usize,
    j: usize,
}

impl<const N: usize, const M: usize> Iterator for MatrixIndexIterator<N, M> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < N && self.j < M {
            let t = (self.i, self.j);

            self.i += 1;
            if self.i >= N {
                self.i = 0;
                self.j += 1;
            }

            Some(t)
        } else {
            None
        }
    }
}

impl<T, const N: usize, const M: usize> Matrix<T, N, M> {
    fn index_iter(&self) -> MatrixIndexIterator<N, M> {
        MatrixIndexIterator { i: 0, j: 0 }
    }

    fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i >= N || j >= M {
            None
        } else {
            Some(&self.m[i][j])
        }
    }

    fn unsafe_get(&self, i: usize, j: usize) -> &T { &self.get(i, j).expect("Index out of bounds") }

    fn fill<F, B>(&self, i: usize, j: usize, f: F) -> Matrix<Option<B>, N, M>
    where F: Fn(&T) -> Option<B>, B: Default + Copy {
        let mut m = [[None; M]; N];

        let mut visit = vec![(i, j)];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        while !visit.is_empty() {
            let cur = visit.pop().unwrap();
            if visited.contains(&cur) { continue }
            visited.insert(cur);

            let x = self.unsafe_get(cur.0, cur.1);
            if let Some(b) = f(x) {
                if cur.0 > 0 { visit.push((cur.0 - 1, cur.1)) }
                if cur.0 < N - 1 { visit.push((cur.0 + 1, cur.1)) }
                if cur.1 > 0 { visit.push((cur.0, cur.1 - 1)) }
                if cur.1 < M - 1 { visit.push((cur.0, cur.1 + 1)) }

                m[cur.0][cur.1] = Some(b);
            }
        }

        Matrix { m }
    }

    fn map_surroundings<F, B>(&self, f: F) -> Matrix<B, N, M>
    where F: Fn(&T, Option<&T>, Option<&T>, Option<&T>, Option<&T>) -> B, B: Default + Copy {
        let mut m = [[B::default(); M]; N];

        for (i, j) in self.index_iter() {
            let x = self.unsafe_get(i, j);
            let up = if j > 0 { self.get(i, j - 1) } else { None };
            let down = self.get(i, j + 1);
            let left = if i > 0 { self.get(i - 1, j) } else { None };
            let right = self.get(i + 1, j);

            m[i][j] = f(x, up, down, left, right);
        }

        Matrix { m }
    }

    fn find_indices<F>(&self, f: F) -> Vec<(usize, usize)> where F: Fn(&T) -> bool {
        let mut indices = vec![];

        for (i, j) in self.index_iter() {
            if f(self.unsafe_get(i, j)) {
                indices.push((i, j));
            }
        }

        indices
    }
}

mod reader {
    use std::fs::File;
    use std::io;
    use std::io::BufRead;

    use super::{Day9Error, Matrix};

    pub fn matrix<const N: usize, const M: usize>(filename: &str) -> Result<Matrix<u8, N, M>, Day9Error> {
        let mut m = [[0; M]; N];
        let lines = io::BufReader::new(File::open(filename)?).lines();

        for (i, line) in lines.enumerate() {
            for (j, char) in line?.chars().enumerate() {
                m[i][j] = char.to_string().parse()?;
            }
        }

        Ok(Matrix { m })
    }
}