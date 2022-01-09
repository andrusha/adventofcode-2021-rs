use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct SubCmd {
    input_filename: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum DayError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("Unparsable vertice")]
    WrongVerticeError,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
enum Vertice {
    Start,
    End,
    Single(String),
    Multi(String)
}

impl From<&str> for Vertice {
    fn from(s: &str) -> Self {
        if s == "start" {
            Vertice::Start
        } else if s == "end" {
            Vertice::End
        } else if s.to_uppercase() == s {
            Vertice::Multi(s.to_string())
        } else {
            Vertice::Single(s.to_string())
        }
    }
}

impl Display for Vertice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Vertice::Start => { write!(f, "start") }
            Vertice::End => { write!(f, "end") }
            Vertice::Single(s) => { write!(f, "{}", s) }
            Vertice::Multi(s) => { write!(f, "{}", s) }
        }
    }
}

pub fn main(args: SubCmd) -> Result<(), DayError> {
    let vs = read_vertices(&args.input_filename)?;
    println!("{:#?}", vs);

    let mut res = vec![];

    let mut paths = vec![(vec![Vertice::Start], false)];
    while let Some((longest, double)) = paths.pop() {
        let end = longest.last().unwrap();
        for cont in vs[end].iter() {
            match cont {
                Vertice::End => {
                    let mut r = longest.clone();
                    r.push(Vertice::End);
                    res.push(r);
                },
                Vertice::Start => {},
                Vertice::Single(_) => {
                    let cont_self = longest.contains(cont);

                    if !cont_self || (cont_self && !double) {
                        let mut e = longest.clone();
                        e.push(cont.clone());
                        paths.push((e, double || (cont_self && !double)));
                    }
                }
                Vertice::Multi(_) => {
                    // if !contains_a_tuple(&longest, (end, cont)) {
                        let mut e = longest.clone();
                        e.push(cont.clone());
                        paths.push((e, double));
                    // }
                }
            }
        }
    }

    for r in res.iter() {
        for x in r.iter() {
            print!("{},", x);
        }
        println!();
    }

    println!("Total paths: {}", res.len());

    Ok(())
}

fn contains_a_tuple<T: PartialEq>(xs: &[T], t: (&T, &T)) -> bool {
    if xs.len() < 2 { return false }

    let mut first = &xs[0];
    for second in &xs[1..] {
        if t.0 == first && t.1 == second { return true }
        first = second;
    }

    false
}

fn read_vertices(filename: &str) -> Result<HashMap<Vertice, HashSet<Vertice>>, DayError> {
    let mut hm = HashMap::new();

    let lines = io::BufReader::new(File::open(filename)?).lines();

    for edge in lines {
        if let Some((to, from)) = edge?.split_once('-') {
            let to: Vertice = to.into();
            let from: Vertice = from.into();

            hm.entry(to.clone()).or_insert(HashSet::new()).insert(from.clone());
            hm.entry(from).or_insert(HashSet::new()).insert(to);
        }
    }

    Ok(hm)
}
