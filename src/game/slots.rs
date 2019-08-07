/// This is an informal Slots simulation based on 6 random numbers in the range of 0-9
/// In the absence of slot machine symbols, the numbers are used for poker-like scoring.
/// https://exercism.io/my/solutions/bf6b1195dc0e4b7885049db0c2605c09
///
use super::{current_time};

extern crate wasm_bindgen;
extern crate js_sys;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use itertools::join;
use std::collections::{HashMap, HashSet};
use rand::{Rng, SeedableRng};

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

impl Play {
    pub fn get_score(&self) -> f32 {
        match self {
            Play::NumSequence(nums) => nums.len() as f32 * 5.0,
            Play::NumMatch(nums) => nums.len() as f32 * 4.0,
            Play::HighNum(num) => *num as f32 * 1.0,
        }
    }
}

// #[derive(Clone, Eq, PartialEq, Hash, Debug)]
// pub enum Scoring {

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
        let mut rng = get_rng();
        for _ in 0..SLOT_COUNT {
            let x = rng.gen_range(0, 10);
            self.nums.push(x as u32);
        }
    }

    pub fn get_nums(&self) -> String {
        let values = join(&self.nums, "|");
        values
    }

    pub fn calc_score(&self, numbers: &Vec<u32>) -> f32 {
        let mut nums = numbers.clone();
        nums.sort();
        let matches = self.eval_matches(&nums);

        eprintln!("matches={:?}", matches);
        let match_score = matches.iter().fold(0.0, |mut acc, play|{
            acc += play.get_score();
            acc
        });

        let sequences = self.eval_sequences(&nums);
        eprintln!("sequences={:?}", sequences);
        let seq_score = sequences.iter().fold(0.0, |mut acc, play|{
            acc += play.get_score();
            acc
        });

        match_score + seq_score
    }

    pub fn evaluate_nums(&mut self) {
        let _ = self.calc_score(&self.nums);
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

        for (key, value) in frequencies {
            if value >= 2 {
                // let values = iter::repeat(*key).take(value).collect();
                let values = vec![*key; value];
                plays.push(Play::NumMatch(values));
            }
        }

        plays
    }

    fn eval_sequences(&self, nums: &Vec<u32>) -> Vec<Play> {
        // Iterate over the possible ranges where sequences might occur
        // The minimum sequence length is 3, so limit the number of iterations so that
        // the ranges tested for a nums length of 6 is: 0..6, 1..6, 2..6

        let plays = nums.into_iter().fold(Vec::<Play>::new(), |mut acc, num| {
            if let Some(play) = acc.pop() {
                match play {
                    Play::NumSequence(seq) => {
                        // eprintln!("seq={:?}", seq);
                        if let Some(last_num) = seq.last() {
                            // eprintln!("num={:?} last_num={:?}", num, last_num);
                            let num = num.to_owned();
                            if num == last_num.to_owned() + 1 {
                                let mut seq = seq.clone();
                                seq.push(num);
                                acc.push(Play::NumSequence(seq));
                            } else {
                                if seq.len() >= 3 {
                                    acc.push(Play::NumSequence(seq));
                                }
                                acc.push(Play::NumSequence(vec![num]));
                            }
                        } else {
                            acc.push(Play::NumSequence(vec![*num]));
                        }
                    }
                    _ => ()
                }
            } else {
                acc.push(Play::NumSequence(vec![*num]));
            }
            acc
        });
        let mut results: Vec<Play> = Vec::new();
        for play in plays {
            match play {
                Play::NumSequence(seq) => {
                    if seq.len() >= 3 {
                        results.push(Play::NumSequence(seq.clone()));
                    }
                }
                _ => ()
            }
        }
        results
    }
}

/// Copied from: https://github.com/stevenpack/bob-ross-lipsum-rust/blob/master/src/phrases.rs
/// See also: https://blog.cloudflare.com/cloudflare-workers-as-a-serverless-rust-platform/
fn get_rng() -> SmallRng {

    //from Javascript
    let ticks = current_time();

    // Hack because of unit test error:
    // cannot call wasm-bindgen imported functions on non-wasm targets
    // let ticks: f64 = 1234567890.234;

    //convert the number to byte array to use as a seed
    let tick_bytes = transmute(ticks as u128);
    SmallRng::from_seed(tick_bytes)
}

// fn get_many_rngs() {
//     let mut thread_rng = thread_rng();
//     // Create small, cheap to initialize and fast RNGs with random seeds.
//     // One can generally assume this won't fail.
//     let rngs: Vec<SmallRng> = iter::repeat(())
//         .map(|()| SmallRng::from_rng(&mut thread_rng).unwrap())
//         .take(10)
//         .collect();
// }

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

    #[test]
    fn test_these() {

        let mut slots = Slots::new(SLOT_COUNT);
        slots.nums = vec![6,3,5,4,0,8];
        let score = slots.calc_score(&slots.nums);
        eprintln!("score={:?}", score);
    }

    #[test]
    fn test_many() {
        for i in 0..50 {
            let mut slots = Slots::new(SLOT_COUNT);
            slots.deal();
            let score = slots.calc_score(&slots.nums);
            println!("{}/ {:?} >> score={}", i, slots.get_nums(), score);

        }
    }
}
