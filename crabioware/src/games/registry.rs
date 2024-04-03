use agb::{display::object::SpriteLoader, rng::RandomNumberGenerator};
use alloc::boxed::Box;

use super::common::{GameOverScreen, StartScreen};
use super::pong::PongGame;
use super::snake::SnakeGame;

use super::Game;

#[derive(Clone, Copy, Debug)]
pub enum Games {
    Pong,
    Snake,
    GameOver,
    Start,
}
impl Games {
    pub fn new(&self, loader: &mut SpriteLoader, rng: &mut RandomNumberGenerator) -> Box<dyn Game> {
        match self {
            Self::Pong => Box::new(PongGame::new(loader, rng)),
            Self::Snake => Box::new(SnakeGame::new(loader, rng)),
            Self::Start => Box::new(StartScreen::new()),
            Self::GameOver => Box::new(GameOverScreen::new()),
        }
    }
}