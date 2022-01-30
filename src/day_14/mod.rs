use std::collections::HashMap;
use clap::Parser;
use thiserror::Error;

type Insertion = ((char, char), char);

#[derive(Parser)]
pub struct SubCmd {
    input_filename: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum DayError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ParsingError(#[from] nom::error::Error<String>),
}

pub fn main(args: SubCmd) -> Result<(), DayError> {
    let (tmpl, ins) = reader::read_file(&args.input_filename)?;
    let pairs: Vec<(char, char)> = tmpl.chars().zip(tmpl.chars().skip(1)).collect();
    let ins: HashMap<(char, char), char> = HashMap::from_iter(ins);

    let mut char_tally = HashMap::new();
    for c in tmpl.chars() {
        char_tally.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut tally = HashMap::new();
    for p in pairs {
        tally.entry(p).and_modify(|v| *v += 1).or_insert(1);
    }

    for s in 1..=40 {
        let (t, ct) = step(&tally, &char_tally, &ins);
        tally = t;
        char_tally = ct;
        // let cnt = char_count(&tally);
        let common = char_tally.iter().max_by_key(|(_, f)| *f).unwrap();
        let rare = char_tally.iter().min_by_key(|(_, f)| *f).unwrap();

        println!("Step {}, most common {}, least common {}, diff = {}", s, fmt_pair(common), fmt_pair(rare), common.1 - rare.1);
    }


    Ok(())
}

fn fmt_pair(p: (&char, &usize)) -> String {
    format!("({}, {})", p.0.to_string(), p.1)
}

fn step(tally: &HashMap<(char, char), usize>, char_tally: &HashMap<char, usize>, ins: &HashMap<(char, char), char>) -> (HashMap<(char, char), usize>, HashMap<char, usize>) {
    let mut res = HashMap::new();
    let mut char_res = char_tally.clone();

    for (p, &freq) in tally.iter() {
       if ins.contains_key(p) {
           let mid = ins[p];

           res
               .entry((p.0, mid))
               .and_modify(|v| *v += freq )
               .or_insert(freq);

           res
               .entry((mid, p.1))
               .and_modify(|v| *v += freq )
               .or_insert(freq);

           char_res
               .entry(mid)
               .and_modify(|v| *v += freq )
               .or_insert(freq);
       } else {
           res
               .entry(*p)
               .and_modify(|v| *v += freq )
               .or_insert(0);
       }
    }

    (res, char_res)
}

mod reader {
    use nom::{Finish, IResult};
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, anychar, multispace1};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{pair, separated_pair};

    use crate::day_14::{DayError, Insertion};

    pub fn read_file(fname: &str) -> Result<(String, Vec<Insertion>), DayError> {
        Ok(parse_input(&std::fs::read_to_string(fname)?)
            .map(|(_, r)| r)
            .map_err(|e| e.to_owned())
            .finish()?)
    }

    fn template(input: &str) -> IResult<&str, String> {
        map(alpha1, |s: &str| s.to_string())(input)
    }

    fn insertion(input: &str) -> IResult<&str, Insertion> {
        separated_pair(pair(anychar, anychar), tag(" -> "), anychar)(input)
    }

    fn insertions(input: &str) -> IResult<&str, Vec<Insertion>> {
        separated_list1(multispace1, insertion)(input)
    }

    fn parse_input(input: &str) -> IResult<&str, (String, Vec<Insertion>)> {
        separated_pair(template, multispace1, insertions)(input)
    }
}