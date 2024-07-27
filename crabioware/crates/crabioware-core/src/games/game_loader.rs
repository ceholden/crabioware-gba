use agb::rng::RandomNumberGenerator;
use alloc::boxed::Box;

use super::{Game, GameDifficulty, Games};

pub trait GameLoader: Copy {
    fn load_game<'a>(
        self,
        game: &Games,
        difficulty: &'a GameDifficulty,
        rng: &'a mut RandomNumberGenerator,
    ) -> Box<dyn Game<'a> + 'a>;
}
