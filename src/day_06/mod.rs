use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

use clap::Parser;
use thiserror::Error;

type Fish = i64;

const FISH_BIRTH_DAYS: i64 = 8;
const FISH_SPAWN_DAYS: i64 = 6;

#[derive(Parser)]
pub struct Day6SubCmd {
    input_filename: String,
}

#[derive(Error, Debug)]
pub enum Day6Error {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error("Unable to parse the number")]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub fn main(args: Day6SubCmd) -> Result<(), Day6Error> {
    let mut genesis_fishes = String::new();
    File::open(args.input_filename)?.read_to_string(&mut genesis_fishes)?;

    let genesis_fishes: Vec<Fish> = genesis_fishes.split(',').map(|d| d.parse()).flatten().collect();
    println!("Initial state: {:?}", genesis_fishes);

    let mut memo = HashMap::new();

    for days in 1..=256 {
        let mut total_spawn: Fish = genesis_fishes.len() as Fish;
        for &f in genesis_fishes.iter() {
            total_spawn += fishes_generation(f, days, &mut memo);
        }

        println!("Total fishes spawned on Day {}: {}", days, total_spawn);
    }

    Ok(())
}

fn fishes_generation(age: Fish, days_left: Fish, memo: &mut HashMap<(Fish, Fish), Fish>) -> Fish {
    if age >= days_left { return 0; }
    if let Some(&result) = memo.get(&(age, days_left)) { return result; }

    let lifetime = days_left - age - 1;
    let siblings = lifetime / (FISH_SPAWN_DAYS + 1);
    // firstborn accounted separately
    let family = 1 + siblings;

    let mut generation = family;
    for i in 0..family {
        let sibling_age = lifetime - i * (FISH_SPAWN_DAYS + 1);
        generation += fishes_generation(FISH_BIRTH_DAYS, sibling_age, memo);
    }

    memo.insert((age, days_left), generation);
    generation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_days() {
        // no spawn
        assert_eq!(fishes_generation(0, 0, &mut HashMap::new()), 0);
        assert_eq!(fishes_generation(1, 0, &mut HashMap::new()), 0);
    }

    #[test]
    fn test_first_day() {
        // i: 1
        // 1: 0
        assert_eq!(fishes_generation(1, 1, &mut HashMap::new()), 0);

        // i: 2
        // 1: 1
        assert_eq!(fishes_generation(2, 1, &mut HashMap::new()), 0);

        // i: 0
        // 1: 6, 8
        assert_eq!(fishes_generation(0, 1, &mut HashMap::new()), 1);
    }

    #[test]
    fn test_one_layer_spawn() {
        // i: 1
        // 1: 0
        // 2: 6, 8
        assert_eq!(fishes_generation(1, 2, &mut HashMap::new()), 1);

        // i: 2
        // 1: 1
        // 2: 0
        // 3: 6, 8
        assert_eq!(fishes_generation(2, 3, &mut HashMap::new()), 1);

        // i: 3
        // 1: 2
        // 2: 1
        // 3: 0
        // 4: 6, 8
        // 5: 5, 7
        assert_eq!(fishes_generation(3, 5, &mut HashMap::new()), 1);

        // i: 6
        // 1: 5
        // 2: 4
        // 3: 3
        // 4: 2
        // 5: 1
        // 6: 0
        // 7: 6, 8
        // 8: 5, 7
        assert_eq!(fishes_generation(6, 8, &mut HashMap::new()), 1);

        // i: 8
        // 1: 7
        // 2: 6
        // 3: 5
        // 4: 4
        // 5: 3
        // 6: 2
        // 7: 1
        // 8: 0, 8
        // 9: 6, 7
        assert_eq!(fishes_generation(8, 9, &mut HashMap::new()), 1);
    }

    #[test]
    fn test_two_layer_spawn() {
        // i: 0
        // 1: 6, 8
        // 2: 5, 7
        // 3: 4, 6
        // 4: 3, 5
        // 5: 2, 4
        // 6: 1, 3
        // 7: 0, 2
        // 8: 6, 1, 8
        // 9: 5, 0, 7
        assert_eq!(fishes_generation(0, 9, &mut HashMap::new()), 2);

        //  i: 3
        //  1: 2
        //  2: 1
        //  3: 0
        //  4: 6, 8
        //  5: 5, 7
        //  6: 4, 6
        //  7: 3, 5
        //  8: 2, 4
        //  9: 1, 3
        // 10: 0, 2
        // 11: 6, 1, 8
        // 12: 5, 0, 7
        // 13: 4, 6, 6, 8
        // 14: 3, 5, 5, 7
        // 15: 2, 4, 4, 6
        // 16: 1, 3, 3, 5
        // 17: 0, 2, 2, 4
        // 18: 6, 1, 1, 3, 8
        assert_eq!(fishes_generation(3, 18, &mut HashMap::new()), 4);

        //  i: 0
        //  1: 6, 8
        //  2: 5, 7
        //  3: 4, 6
        //  4: 3, 5
        //  5: 2, 4
        //  6: 1, 3
        //  7: 0, 2
        //  8: 6, 1, 8
        //  9: 5, 0, 7
        // 10: 4, 6, 6, 8
        assert_eq!(fishes_generation(0, 10, &mut HashMap::new()), 3);
    }

    #[test]
    fn test_three_layer_spawn() {
        //  i: 0
        //  1: 6, 8
        //  2: 5, 7
        //  3: 4, 6
        //  4: 3, 5
        //  5: 2, 4
        //  6: 1, 3
        //  7: 0, 2
        //  8: 6, 1, 8
        //  9: 5, 0, 7
        // 10: 4, 6, 6, 8
        // 11: 3, 5, 5, 7
        // 12: 2, 4, 4, 6
        // 13: 1, 3, 3, 5
        // 14: 0, 2, 2, 4
        // 15: 6, 1, 1, 3, 8
        // 16: 5, 0, 0, 2, 8
        // 17: 4, 6, 6, 1, 7, 8, 8
        assert_eq!(fishes_generation(0, 17, &mut HashMap::new()), 6);
    }

    #[test]
    fn test_four_layer_spawn() {
        //  i: 0
        //  1: 6, 8
        //  2: 5, 7
        //  3: 4, 6
        //  4: 3, 5
        //  5: 2, 4
        //  6: 1, 3
        //  7: 0, 2
        //  8: 6, 1, 8
        //  9: 5, 0, 7
        // 10: 4, 6, 6, 8
        // 11: 3, 5, 5, 7
        // 12: 2, 4, 4, 6
        // 13: 1, 3, 3, 5
        // 14: 0, 2, 2, 4
        // 15: 6, 1, 1, 3, 8
        // 16: 5, 0, 0, 2, 8
        // 17: 4, 6, 6, 1, 7, 8, 8
        // 18: 3, 5, 5, 0, 6, 7, 7
        // 19: 2, 4, 4, 6, 5, 6, 6, 8
        assert_eq!(fishes_generation(0, 19, &mut HashMap::new()), 7);
    }
}
