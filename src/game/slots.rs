/// This is a simulator for a slot machine
///

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use rand::prelude::*;

const SLOT_COUNT: usize = 6;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Slots {
    nums: Vec<u32>,
}

#[wasm_bindgen]
impl Slots {
    pub fn new() -> Self {
        Slots {
            nums: Vec::with_capacity(SLOT_COUNT)
        }
    }

    pub fn deal(&mut self) {
        self.nums.clear();
        let mut rng = self.get_rng();
        for _ in 0..SLOT_COUNT {
            let x = rng.gen_range(0, 10);
            println!("{}", x);
            self.nums.push(x as u32);
        }
    }

    fn get_rng(&self) -> SmallRng {
        SmallRng::from_entropy()
    }

    pub fn get_nums(&self) -> Vec<u32> {
        self.nums.clone()
    }

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