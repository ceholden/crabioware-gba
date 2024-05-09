use agb::{display::object::SpriteLoader, rng::RandomNumberGenerator};
use alloc::boxed::Box;

use crabioware_core::screens::{GameOverScreen, StartScreen, VictoryScreen};
use crabioware_paccrab::PacCrabGame;
use crabioware_pong::PongGame;
use crabioware_snake::SnakeGame;

use crabioware_core::games::{GameDifficulty, Games, RunnableGame};

pub trait GameRunner {
    fn new(
        &self,
        difficulty: &GameDifficulty,
        loader: &mut SpriteLoader,
        rng: &mut RandomNumberGenerator,
    ) -> Box<dyn RunnableGame>;
}
impl GameRunner for Games {
    fn new(
        &self,
        difficulty: &GameDifficulty,
        loader: &mut SpriteLoader,
        rng: &mut RandomNumberGenerator,
    ) -> Box<dyn RunnableGame> {
        match self {
            Self::Pong => Box::new(PongGame::new(&difficulty, loader, rng)),
            Self::Snake => Box::new(SnakeGame::new(&difficulty, loader, rng)),
            Self::PacCrab => Box::new(PacCrabGame::new(&difficulty, loader, rng)),
            Self::Start => Box::new(StartScreen::new()),
            Self::GameOver => Box::new(GameOverScreen::new()),
            Self::Victory => Box::new(VictoryScreen::new()),
        }
    }
}
