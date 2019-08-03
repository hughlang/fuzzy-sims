/// A prototyping space
///
///
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Game {
    pub id: u32,
}

#[wasm_bindgen]
impl Game {
    pub fn new(id: u32) -> Self {
        Game {
            id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test_create() {
        let mut game = Game::new(0);
        assert_eq!(game.id, 0);
        game.id = 10;
        assert_eq!(game.id, 10);
    }

}
