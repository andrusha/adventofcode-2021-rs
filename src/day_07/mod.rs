// use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::Range;
// use bitvec::macros::internal::funty::{IsInteger, IsNumber, IsSigned};

use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct Day7SubCmd {
    input_filename: String,
}

#[derive(Error, Debug)]
pub enum Day7Error {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error("Unable to parse the number")]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub fn main(args: Day7SubCmd) -> Result<(), Day7Error> {
    let positions = read_positions(args.input_filename.as_str())?;
    println!("Crabs horizontal positions: {:?}", positions);

    let r = *positions.iter().min().unwrap() ..*positions.iter().max().unwrap();

    let (naive_min, naive_min_d) = find_min(&r, |x| distance(&positions, x));
    println!("Naive crab position: {}, distance: {}", naive_min, naive_min_d);

    let (factored_min, factored_min_d) = find_min(&r, |x| factor_distance(&positions, x));
    println!("Factored crab position: {}, factored distance: {}", factored_min, factored_min_d);

    // let b = bisect(&r, |l, r| distance(&positions, l).cmp(&distance(&positions, r)));
    // println!("Bisected position: {}, distance: {}", b, distance(&positions, b));
    //
    // match median(&positions) {
    //     Some(Median::Odd(m)) => {
    //         println!("Median / most efficient position is: {}", m);
    //         let ds = distances(&positions, m);
    //         println!("Distances: {:?}", ds);
    //         let fuel: i32 = ds.iter().sum();
    //         println!("Total fuel requirement: {}", fuel);
    //     }
    //     Some(Median::Even(l, r)) => {
    //         println!("Two median values found: {}, {}", l, r);
    //         let dl = distances(&positions, l);
    //         println!("Distances to the {} median: {:?}", l, dl);
    //         let dr = distances(&positions, r);
    //         println!("Distances to the {} median: {:?}", r, dr);
    //
    //         let fl : i32 = dl.iter().sum();
    //         let fr : i32 = dr.iter().sum();
    //         println!("Fuel requirements for {}: {}, {}: {}", l, fl, r, fr);
    //     }
    //     None => { println!("Positions are empty") }
    // }

    Ok(())
}

fn read_positions(filename: &str) -> Result<Vec<i32>, Day7Error> {
    let mut input = String::new();
    File::open(filename)?.read_to_string(&mut input)?;

    let mut res = vec![];

    // Do not swallow possible errors
    for s in input.split(',') {
        res.push(s.trim().parse()?);
    }

    Ok(res)
}

fn distance(xs: &Vec<i32>, e: i32) -> i32 {
    xs.iter().map(|&x| (e - x).abs()).sum()
}

fn factor_distance(xs: &Vec<i32>, e: i32) -> i32 {
    xs.iter().map(|&x| (e - x).abs()).map(|d| d*(d+1)/2).sum()
}

fn find_min<F>(r: &Range<i32>, f: F) -> (i32, i32)
    where F: Fn(i32) -> i32 {
    let mut min = r.start;
    let mut min_d = f(min);
    for x in r.clone() {
        let d = f(x);
        if d < min_d {
            min = x;
            min_d = d;
        }
    }

    (min, min_d)
}

// fn midvalue(r: &Range<i32>) -> i32 {
//     r.start + (r.end - r.start) / 2
// }
//
// fn bisect<F>(r: &Range<i32>, f: F) -> i32
//     where F: Fn(i32, i32) -> Ordering {
//     if r.end - r.start == 1 { return r.start }
//
//     let mid = midvalue(r);
//     let left = r.start .. mid;
//     let right = mid .. r.end + 1;
//     println!("{:?} {} {:?}", left, mid, right);
//
//     match f(r.start, mid) {
//         Ordering::Less => {
//             bisect(&left, f)
//         }
//         Ordering::Equal => {
//             mid
//         }
//         Ordering::Greater => {
//             bisect(&right, f)
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests_bisect {
//     use super::bisect;
//
//     #[test]
//     fn test_min() {
//         let xs: Vec<i32> = vec![];
//         assert_eq!(bisect(&(0..10), |l, r| l.cmp(&r)), 0);
//     }
//
//     #[test]
//     fn test_max() {
//         let xs: Vec<i32> = vec![];
//         assert_eq!(bisect(&(0..10), |l, r| l.cmp(&r).reverse()), 9);
//     }
//
//     #[test]
//     fn test_mid() {
//         let xs: Vec<i32> = vec![];
//         assert_eq!(bisect(&(0..10), |l, r| (r-l).cmp(&4)), 5);
//     }
// }
//
// #[derive(Debug, Eq, PartialEq)]
// enum Median<T> {
//     Odd(T),
//     Even(T, T)
// }
//
// fn median<T: Copy + Ord>(xs: &Vec<T>) -> Option<Median<T>> {
//     if xs.is_empty() { return None }
//
//     let mut sorted = xs.clone();
//     sorted.sort();
//
//     let i = sorted.len() / 2;
//     if sorted.len() % 2 == 0 {
//         // -1 because we index from 0
//         Some(Median::Even(sorted[i - 1], sorted[i]))
//     } else {
//         Some(Median::Odd(sorted[i]))
//     }
// }

// fn distances<T: IsSigned + IsNumber>(xs: &Vec<T>, e: T) -> Vec<T> {
//     xs.iter().map(|&x| (e - x).abs()).collect()
// }

// #[cfg(test)]
// mod tests_median {
//     use super::{median, Median};
//
//     #[test]
//     fn test_empty() {
//         let xs: Vec<i32> = vec![];
//         assert_eq!(median(&xs), None);
//     }
//
//     #[test]
//     fn test_odd() {
//         let xs = vec![6, 8, 21, 4, 9, 20, 42];
//         assert_eq!(median(&xs), Some(Median::Odd(9)));
//     }
//
//     #[test]
//     fn test_even() {
//         let xs = vec![6, 8, 21, 4, 9, 20];
//         assert_eq!(median(&xs), Some(Median::Even(8, 9)));
//     }
// }
