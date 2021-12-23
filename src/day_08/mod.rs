use std::collections::HashSet;
use std::io;

use clap::Parser as ClapParser;
use thiserror::Error;

type Signal = HashSet<Wire>;

#[derive(ClapParser)]
pub struct Day8SubCmd {
    input_filename: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum Day8Error {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error("Unknown wire type")]
    ParseWireError,

    #[error(transparent)]
    ParsingError(#[from] nom::error::Error<String>),
}

pub fn main(args: Day8SubCmd) -> Result<(), Day8Error> {
    let entries = reader::entries(args.input_filename.as_str())?;

    let mut known_digits = 0;
    let permutations = permutations(&[0, 1, 2, 3, 4, 5, 6]);
    println!("Total permutations: {}", permutations.len());
    let cyphers = all_cyphers(permutations);
    println!("Total cyphers: {}", cyphers.len());

    let mut sum_digits = 0;

    for entry in entries.iter() {
        println!("{:?}", entry);

        if let Some(cypher) = find_cypher(entry, &cyphers) {
            let decyphered = entry.decrypt(cypher);
            sum_digits += decyphered.output_num();

            println!("{:?}", cypher);
            println!("{:?}", decyphered);
            println!("Signal: {}, Digits: {}", decyphered.signal_num(), decyphered.output_num());
            println!();
        }

        known_digits += entry.outputs.iter().flat_map(signal_to_digit).count();
    }

    println!("Known simple digits counter: {}", known_digits);
    println!("Sum of output digits: {}", sum_digits);

    Ok(())
}

#[derive(Debug)]
struct Cypher {
    signals: Vec<Signal>,
}

impl Cypher {
    fn decrypt(&self, signals: &[Signal]) -> Vec<usize> {
        let mut result = vec![];

        for signal in signals {
            let i = self.signals.iter()
                .position(|s| *signal == *s)
                .expect("Unknown signal");
            result.push(i);
        }

        result
    }
}

/**
Implements Heap's algorithm https://en.wikipedia.org/wiki/Heap%27s_algorithm
Using swaps without recursion to minimize number of moves
 **/
fn permutations<T: Copy + Default>(items: &[T]) -> Vec<Vec<T>> {
    let mut output = vec![];
    // stack is an encoding of the stack state
    // stack[k] encodes the for-loop counter for when recurse(i - 1, items) is called
    let mut stack = vec![0; items.len()];

    let mut inner = vec![T::default(); items.len()];
    inner.copy_from_slice(items);
    output.push(inner.clone());

    let mut i = 0; // stack pointer
    while i < items.len() {
        if stack[i] < i {
            if i % 2 == 0 {
                inner.swap(0, i);
            } else {
                inner.swap(stack[i], i);
            }
            output.push(inner.clone());

            // swap at the end of the loop, simulate the increment of the counter
            stack[i] += 1;
            // recursion reaching the base case
            i = 0;
        } else {
            // calling recurse(i+1, A) has ended as the for-loop terminated
            // reset the state and simulate popping the stack by incrementing the pointer
            stack[i] = 0;
            i += 1;
        }
    }

    output
}

/**
Generate all possible ways signal could be encrypted with wires mapping
 **/
fn all_cyphers(permutations: Vec<Vec<usize>>) -> Vec<Cypher> {
    fn signal(indices: &[usize], mapping: &[usize]) -> Signal {
        let mut signal: Signal = HashSet::new();
        let wires = Wire::all();

        for &i in indices {
            signal.insert(wires[mapping[i]]);
        }

        signal
    }

    let mut cyphers = vec![];

    for mapping in permutations {
        let signals = vec![
            signal(&[0, 1, 2, 4, 5, 6], &mapping),    // 0
            signal(&[2, 5], &mapping),                // 1
            signal(&[0, 2, 3, 4, 6], &mapping),       // 2
            signal(&[0, 2, 3, 5, 6], &mapping),       // 3
            signal(&[1, 2, 3, 5], &mapping),          // 4
            signal(&[0, 1, 3, 5, 6], &mapping),       // 5
            signal(&[0, 1, 3, 4, 5, 6], &mapping),    // 6
            signal(&[0, 2, 5], &mapping),             // 7
            signal(&[0, 1, 2, 3, 4, 5, 6], &mapping), // 8
            signal(&[0, 1, 2, 3, 5, 6], &mapping),    // 9
        ];
        cyphers.push(Cypher { signals });
    }

    cyphers
}

fn find_cypher<'a>(entry: &Entry, cyphers: &'a [Cypher]) -> Option<&'a Cypher> {
    for cypher in cyphers {
        if entry.signals.iter().all(|s| cypher.signals.contains(s)) {
            return Some(cypher);
        }
    }

    None
}

fn signal_to_digit(signal: &Signal) -> Option<usize> {
    match signal.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Wire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Wire {
    fn all() -> [Wire; 7] {
        [
            Wire::A,
            Wire::B,
            Wire::C,
            Wire::D,
            Wire::E,
            Wire::F,
            Wire::G,
        ]
    }
}

impl TryFrom<char> for Wire {
    type Error = Day8Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Wire::A),
            'b' => Ok(Wire::B),
            'c' => Ok(Wire::C),
            'd' => Ok(Wire::D),
            'e' => Ok(Wire::E),
            'f' => Ok(Wire::F),
            'g' => Ok(Wire::G),
            _ => Err(Day8Error::ParseWireError)
        }
    }
}

#[derive(Default, Debug)]
pub struct Entry {
    signals: Vec<Signal>,
    outputs: Vec<Signal>,
}

#[derive(Debug)]
struct DecryptedEntry {
    signals: Vec<usize>,
    outputs: Vec<usize>,
}

fn dec_vec_to_int(vec: &[usize]) -> i64 {
    let mut x = 0;

    for (i, &n) in vec.iter().enumerate() {
        let exp = (vec.len() - i - 1) as u32;
        x += 10_i64.pow(exp) * n as i64;
    }

    x
}

impl DecryptedEntry {
    fn signal_num(&self) -> i64 {
        dec_vec_to_int(&self.signals)
    }

    fn output_num(&self) -> i64 {
        dec_vec_to_int(&self.outputs)
    }
}

impl Entry {
    fn decrypt(&self, cypher: &Cypher) -> DecryptedEntry {
        DecryptedEntry {
            signals: cypher.decrypt(&self.signals),
            outputs: cypher.decrypt(&self.outputs),
        }
    }
}

mod reader {
    use std::fs::File;
    use std::io;
    use std::io::BufRead;

    use nom::{Finish, IResult};
    use nom::bytes::complete::tag;
    use nom::character::complete::{one_of, space1};
    use nom::combinator::{map, map_res};
    use nom::multi::{many1, separated_list0};
    use nom::sequence::separated_pair;

    use super::{Day8Error, Entry, Signal, Wire};

    pub fn entries(filename: &str) -> Result<Vec<Entry>, Day8Error> {
        let mut entries = vec![];

        for line in io::BufReader::new(File::open(filename)?).lines() {
            let (_, parsed) = entry(line?.as_str()).map_err(|e| e.to_owned()).finish()?;
            entries.push(parsed);
        }

        Ok(entries)
    }

    fn entry(input: &str) -> IResult<&str, Entry> {
        map(
            separated_pair(signals, tag(" | "), signals),
            |(signals, outputs)| Entry { signals, outputs },
        )(input)
    }

    fn signals(input: &str) -> IResult<&str, Vec<Signal>> {
        separated_list0(space1, signal)(input)
    }

    fn signal(input: &str) -> IResult<&str, Signal> {
        map(many1(wire), |ws| ws.into_iter().collect())(input)
    }

    fn wire(input: &str) -> IResult<&str, Wire> {
        map_res(one_of("abcdefg"), |c| c.try_into())(input)
    }
}