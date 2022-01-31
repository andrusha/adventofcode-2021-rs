use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

use bitvec::macros::internal::funty::IsNumber;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Matrix<T, const N: usize, const M: usize> {
    m: [[T; M]; N],
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Pos<const N: usize, const M: usize> {
    i: usize,
    j: usize,
}

impl<const N: usize, const M: usize> Pos<N, M> {
    pub fn new(i: usize, j: usize) -> Pos<N, M> {
        if i >= N || j >= M {
            panic!("Position ({}, {}) is out of bounds of matrix dimensions ({}, {})", i, j, N, M);
        }

        Pos { i, j }
    }

    fn adjust(&self, diff: &Diff) -> Option<Pos<N, M>> {
        let i = self.i.checked_add_signed(diff.i);
        let j = self.j.checked_add_signed(diff.j);

        match (i, j) {
            (Some(i), Some(j)) if i < N && j < M => Some(Pos { i, j }),
            _ => None
        }
    }
}

// #[derive(Copy)]
struct Diff {
    i: isize,
    j: isize,
}

pub struct DiffIterator<'a, const N: usize, const M: usize, const D: usize> {
    i: usize,
    start: &'a Pos<N, M>,
    diffs: [Diff; D],
}

impl<'a, const N: usize, const M: usize, const D: usize> Iterator for DiffIterator<'a, N, M, D> {
    type Item = Pos<N, M>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.diffs.len() {
            let diff = &self.diffs[self.i];
            self.i += 1;

            let new_pos = self.start.adjust(diff);
            if new_pos.is_some() {
                return new_pos;
            }
        }

        None
    }
}

pub struct IndexIterator<const N: usize, const M: usize> {
    i: usize,
    j: usize,
}

impl<const N: usize, const M: usize> Iterator for IndexIterator<N, M> {
    type Item = Pos<N, M>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < N && self.j < M {
            let p = Pos { i: self.i, j: self.j };

            self.j += 1;
            if self.j >= N {
                self.j = 0;
                self.i += 1;
            }

            Some(p)
        } else {
            None
        }
    }
}

impl<T: Copy, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn new(fill: T) -> Matrix<T, N, M> {
        let m = [[fill; M]; N];
        Matrix { m }
    }

    pub fn index_iter(&self) -> IndexIterator<N, M> {
        IndexIterator { i: 0, j: 0 }
    }

    pub fn set(&mut self, pos: &Pos<N, M>, x: T) {
        self.m[pos.i][pos.j] = x;
    }

    pub fn get(&self, pos: &Pos<N, M>) -> &T {
        &self.m[pos.i][pos.j]
    }

    pub fn fill<F, B>(&self, start: &Pos<N, M>, f: F) -> Matrix<Option<B>, N, M>
        where F: Fn(&T) -> Option<B>, B: Default + Copy {
        let mut m = Matrix::new(None);

        let mut visit = vec![start.clone()];
        let mut visited = HashSet::new();

        while !visit.is_empty() {
            let cur = visit.pop().unwrap();
            if visited.contains(&cur) { continue; }
            visited.insert(cur.clone());

            let x = self.get(&cur);
            if let Some(b) = f(x) {
                visit.append(&mut self.direct_neighbours(&cur).collect());

                m.set(&cur, Some(b));
            }
        }

        m
    }

    pub fn direct_neighbours<'a>(&self, pos: &'a Pos<N, M>) -> DiffIterator<'a, N, M, 4> {
        let diffs = [
            Diff { i: 1, j: 0 },
            Diff { i: 0, j: 1 },
            Diff { i: -1, j: 0 },
            Diff { i: 0, j: -1 },
        ];

        DiffIterator { i: 0, start: pos, diffs }
    }

    pub fn all_neighbors<'a>(&self, pos: &'a Pos<N, M>) -> DiffIterator<'a, N, M, 8> {
        let diffs = [
            Diff { i: 0, j: -1 },
            Diff { i: 0, j: 1 },
            Diff { i: -1, j: 0 },
            Diff { i: 1, j: 0 },
            Diff { i: 1, j: 1 },
            Diff { i: -1, j: -1 },
            Diff { i: 1, j: -1 },
            Diff { i: -1, j: 1 },
        ];

        DiffIterator { i: 0, start: pos, diffs }
    }

    pub fn map_surroundings<F, B>(&self, f: F) -> Matrix<B, N, M>
        where F: Fn(&T, &[&T]) -> B, B: Default + Copy {
        let mut m = Matrix::new(B::default());

        for xpos in self.index_iter() {
            let x = self.get(&xpos);

            let neighbours: Vec<&T> = self.direct_neighbours(&xpos).map(|p| self.get(&p)).collect();
            m.set(&xpos, f(x, &neighbours));
        }

        m
    }

    pub fn map<F>(&mut self, f: F)
        where F: Fn(&T) -> T {
        for pos in self.index_iter() {
            self.set(&pos, f(self.get(&pos)));
        }
    }

    pub fn find_indices<F>(&self, f: F) -> Vec<Pos<N, M>>
        where F: Fn(&T) -> bool {
        let mut indices = vec![];

        for pos in self.index_iter() {
            if f(self.get(&pos)) {
                indices.push(pos);
            }
        }

        indices
    }
}

impl<T: Display + Copy, const N: usize, const M: usize> Display for Matrix<T, N, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for pos in self.index_iter() {
            if pos.i != 0 && pos.j == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", self.get(&pos))?;
        }

        Ok(())
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum MatrixError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("Unable to parse the number")]
    ParseIntError(#[from] std::num::ParseIntError),
}

impl<T: IsNumber, const N: usize, const M: usize> Matrix<T, N, M>
    where MatrixError: From<<T as FromStr>::Err> {
    pub fn from_file(filename: &str) -> Result<Matrix<T, N, M>, MatrixError> {
        let mut m = Matrix::new(T::default());
        let lines = io::BufReader::new(File::open(filename)?).lines();

        for (i, line) in lines.enumerate() {
            for (j, char) in line?.chars().enumerate() {
                m.set(&Pos::new(i, j), char.to_string().parse()?);
            }
        }

        Ok(m)
    }
}