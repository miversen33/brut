use std::char;

use rand::rngs::ThreadRng;
use rand::Rng;

const ENGLISH_ASCII_RANGE: std::ops::Range<u32> = 31 .. 128;

pub struct Monkey {

    /// How many guesses have we made?
    pub guess_count: u64,
    /// random thread holder
    random: ThreadRng,
    /// How many characters are we limiting our guess to?
    character_limit: u64
}

impl Monkey {
    pub fn new(character_limit: u64) -> Monkey {
        let random = rand::thread_rng();
        Monkey { random, character_limit, guess_count: 0}
    }

    pub fn smack_typewriter(&mut self) -> String {
        self.guess_count += 1;
        let guess_limit = rand::thread_rng().gen_range(0 .. self.character_limit + 1);
        let mut guess = String::new();
        for _ in 0 .. guess_limit {
            guess.push(char::from_u32(self.random.gen_range(ENGLISH_ASCII_RANGE)).unwrap())
        }
        guess
    }
}
