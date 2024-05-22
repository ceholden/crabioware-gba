use agb::{display::object::SpriteLoader, rng::RandomNumberGenerator};
use alloc::boxed::Box;

// use crabioware_core::screens::{GameOverScreen, StartScreen, VictoryScreen};
use crabioware_paccrab::PacCrabGame;
use crabioware_pong::PongGame;
use crabioware_snake::SnakeGame;

use crabioware_core::games::{Game, GameDifficulty, Games};

pub trait GameRunner {
    fn load_game<'a>(
        self,
        difficulty: &'a GameDifficulty,
        rng: &'a mut RandomNumberGenerator,
    ) -> Box<dyn Game<'a> + 'a>;
}
impl GameRunner for Games {
    fn load_game<'a>(
        self,
        difficulty: &'a GameDifficulty,
        rng: &'a mut RandomNumberGenerator,
    ) -> Box<dyn Game<'a> + 'a> {
        match self {
            Self::Pong => Box::new(PongGame::new(&difficulty, rng)),
            Self::Snake => Box::new(SnakeGame::new(&difficulty, rng)),
            Self::PacCrab => Box::new(PacCrabGame::new(&difficulty, rng)),
            //            Self::Start => Box::new(StartScreen::new()),
            //            Self::GameOver => Box::new(GameOverScreen::new()),
            //            Self::Victory => Box::new(VictoryScreen::new()),
        }
    }
}
