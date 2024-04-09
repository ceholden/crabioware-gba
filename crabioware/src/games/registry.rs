use agb::{display::object::SpriteLoader, rng::RandomNumberGenerator};
use alloc::boxed::Box;

use super::common::{GameOverScreen, StartScreen, VictoryScreen};
use super::pong::PongGame;
use super::snake::SnakeGame;

use super::{Game, GameDifficulty};

#[derive(Clone, Copy, Debug)]
pub enum Games {
    Pong,
    Snake,
    GameOver,
    Start,
    Victory,
}
impl Games {
    pub fn new(
        &self,
        difficulty: &GameDifficulty,
        loader: &mut SpriteLoader,
        rng: &mut RandomNumberGenerator,
    ) -> Box<dyn Game> {
        match self {
            Self::Pong => Box::new(PongGame::new(&difficulty, loader, rng)),
            Self::Snake => Box::new(SnakeGame::new(&difficulty, loader, rng)),
            Self::Start => Box::new(StartScreen::new()),
            Self::GameOver => Box::new(GameOverScreen::new()),
            Self::Victory => Box::new(VictoryScreen::new()),
        }
    }
}
