use agb::rng::RandomNumberGenerator;
use alloc::boxed::Box;

use crabioware_paccrab::PacCrabGame;
use crabioware_pong::PongGame;
use crabioware_snake::SnakeGame;

use crabioware_core::games::{Game, GameDifficulty, GameLoader, Games};

#[derive(Copy, Clone)]
pub struct Registry {}
impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Registry {
        Registry {}
    }
}
impl GameLoader for Registry {
    fn load_game<'a>(
        self,
        game: &Games,
        difficulty: &'a GameDifficulty,
        rng: &'a mut RandomNumberGenerator,
    ) -> Box<dyn Game<'a> + 'a> {
        match game {
            Games::Pong => Box::new(PongGame::new(difficulty, rng)),
            Games::Snake => Box::new(SnakeGame::new(difficulty, rng)),
            Games::PacCrab => Box::new(PacCrabGame::new(difficulty, rng)),
        }
    }
}
