/// This is an informal Slots simulation based on 6 random numbers in the range of 0-9
/// In the absence of slot machine symbols, the numbers are used for poker-like scoring.
/// https://exercism.io/my/solutions/bf6b1195dc0e4b7885049db0c2605c09
///

extern crate wasm_bindgen;
extern crate js_sys;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use itertools::join;
// use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, HashSet};
use std::iter;

const SLOT_COUNT: usize = 6;

/// A Play is a grouping of nums that translate to a score. When numbers are generated
/// for Slots,
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Play {
    /// A sequence of numbers like a straight (e.g. 5-6-7)
    NumSequence(Vec<u32>),
    /// A set of matching numbers (e.g. pair, 3 of a kind)
    NumMatch(Vec<u32>),
    /// The highest number not used in other Ranks
    HighNum(u32),
}

// impl Play {
//     fn increment(&self) {
//         match self {
//             Play::NumSequence(seq) => {

//             }
//             Play::NumMatch(matches) => {

//             }
//             Play::HighNum(value) => {

//             }
//         }
//     }
// }

#[wasm_bindgen]
#[derive(Clone)]
pub struct Slots {
    nums: Vec<u32>,
    plays: HashSet<Play>,
}

impl Default for Slots {
    fn default() -> Self {
        Slots {
            nums: Vec::with_capacity(SLOT_COUNT),
            plays: HashSet::new(),
        }
    }
}

#[wasm_bindgen]
impl Slots {
    #[wasm_bindgen(constructor)]
    pub fn new(count: usize) -> Self {
        Slots {
            nums: Vec::with_capacity(count),
            plays: HashSet::new(),
        }
    }

    pub fn deal(&mut self) {
        self.nums.clear();
        let mut rng = self.get_rng();
        for _ in 0..SLOT_COUNT {
            let x = rng.gen_range(0, 10);
            self.nums.push(x as u32);
        }
    }

    /// Copied from: https://github.com/stevenpack/bob-ross-lipsum-rust/blob/master/src/phrases.rs
    /// See also: https://blog.cloudflare.com/cloudflare-workers-as-a-serverless-rust-platform/
    fn get_rng(&self) -> SmallRng {
        use js_sys::Date;
        use rand::SeedableRng;

        //from Javascript
        // let ticks = Date::now();

        // Hack because of unit test error:
        // cannot call wasm-bindgen imported functions on non-wasm targets
        let ticks: f64 = 1234567890.234;

        //convert the number to byte array to use as a seed
        let tick_bytes = transmute(ticks as u128);
        SmallRng::from_seed(tick_bytes)
    }

    pub fn get_nums(&self) -> String {
        let values = join(&self.nums, "|");
        values
    }

    /// Apply common slot machine rules for winning:
    /// * N of a kind
    /// * High score
    ///
    ///
    pub fn evaluate_nums(&mut self) {
        let mut nums = self.nums.clone();
        nums.sort();
        let matches = self.eval_matches(&nums);
        eprintln!("matches={:?}", matches);

        let sequences = self.eval_sequences(&nums);
        eprintln!("sequences={:?}", sequences);

    }

    /// The nums parameter must be sorted ascending
    /// Inspired by: https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
    fn eval_matches(&self, nums: &Vec<u32>) -> Vec<Play> {
        let mut plays: Vec<Play> = Vec::new();
        eprintln!("nums={:?}", nums);

        let frequencies = nums.iter().fold(HashMap::new(), |mut freqs, value| {
            *freqs.entry(value).or_insert(0) += 1;
            freqs
        });
        eprintln!("freqs={:?}", frequencies);

        for (key, value) in frequencies {
            if value >= 2 {
                let values = iter::repeat(*key).take(value).collect();
                plays.push(Play::NumMatch(values));
            }
        }
        // let matches: HashMap< = frequencies.into_iter().filter(|(value, count)| *count >= 2).collect();
        // let mode = frequencies
        //     .into_iter()
        //     .max_by_key(|&(_, count)| count)
        //     .map(|(value, _)| *value);
        // eprintln!("mode={:?}", mode);

        plays
    }

    fn eval_sequences(&self, nums: &Vec<u32>) -> Vec<Play> {
        // Iterate over the possible ranges where sequences might occur
        // The minimum sequence length is 3, so limit the number of iterations so that
        // the ranges tested for a nums length of 6 is: 0..6, 1..6, 2..6
        //

        let plays = nums.into_iter().fold(Vec::<Play>::new(), |mut acc, num| {
            if let Some(play) = acc.pop() {
                match play {
                    Play::NumSequence(seq) => {
                        let mut seq = seq.clone();
                        seq.push(*num);
                        acc.push(Play::NumSequence(seq));
                    }
                    _ => ()
                }
            } else {
                acc.push(Play::NumSequence(vec![*num]));
            }
            acc
        });

        plays
    }
}


fn transmute(x: u128) -> [u8; 16] {

    let b1 : u8 = ((x >> 120) & 0xffffffff) as u8;
    let b2 : u8 = ((x >> 112) & 0xffffffff) as u8;
    let b3 : u8 = ((x >> 104) & 0xffffffff) as u8;
    let b4 : u8 = ((x >> 96) & 0xffffffff) as u8;

    let b5 : u8 = ((x >> 88) & 0xffffffff) as u8;
    let b6 : u8 = ((x >> 80) & 0xffffffff) as u8;
    let b7 : u8 = ((x >> 72) & 0xffffffff) as u8;
    let b8 : u8 = ((x >> 64) & 0xffffffff) as u8;

    let b9 : u8 = ((x >> 56) & 0xffffffff) as u8;
    let b10 : u8 = ((x >> 48) & 0xffffffff) as u8;
    let b11 : u8 = ((x >> 40) & 0xffffffff) as u8;
    let b12 : u8 = ((x >> 32) & 0xffffffff) as u8;

    let b13 : u8 = ((x >> 24) & 0xffffffff) as u8;
    let b14 : u8 = ((x >> 16) & 0xffffffff) as u8;
    let b15 : u8 = ((x >> 8) & 0xffffffff) as u8;
    let b16 : u8 = (x & 0xffffffff) as u8;

    //Most of the entropy is in the last few bytes and generators are allowed
    //to assume evenly spread entropy in the seed, so spread the bytes around
    [b16, b1, b14, b3, b12, b5, b10, b7, b8, b9, b6, b11, b4, b13, b2, b15]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand() {
        let mut slots = Slots::new(SLOT_COUNT);
        slots.deal();
        println!("{:?}", slots.get_nums());
        assert_eq!(slots.nums.len(), SLOT_COUNT);
        slots.evaluate_nums();
    }
}