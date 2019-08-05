/// This is a simulator for a slot machine
///

extern crate wasm_bindgen;
extern crate js_sys;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use itertools::join;
use std::collections::HashSet;

const SLOT_COUNT: usize = 6;

/// A Play is a grouping of nums that translate to a score. When numbers are generated
/// for Slots,
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Play {
    /// The highest number not used in other Ranks
    HighNum(u32),
    /// A set of matching numbers (e.g. pair, 3 of a kind)
    NumMatch(usize, u32),
    /// A sequence of numbers
    NumSequence(Vec<u32>),
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Slots {
    nums: Vec<u32>,
    plays: HashSet<Play>,
}

#[wasm_bindgen]
impl Slots {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Slots {
            nums: Vec::with_capacity(SLOT_COUNT),
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
        let ticks = Date::now();
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
        let nums = self.nums.clone();


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
        let mut slots = Slots::new();
        slots.deal();
        println!("{:?}", slots.get_nums());
        assert_eq!(slots.nums.len(), SLOT_COUNT);
    }
}