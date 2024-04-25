use agb::display::object::{OamIterator, ObjectUnmanaged, SpriteLoader};
use agb::input::{Button, ButtonController};
use agb::println;

use agb::rng::RandomNumberGenerator;
use crabioware_core::games::{GameDifficulty, GameState, Games, RunnableGame};

use super::graphics::SpriteTag;
use super::levels;

pub struct PacCrabGame {
    time: i32,
}
impl PacCrabGame {
    pub fn new(
        difficulty: &GameDifficulty,
        _: &mut SpriteLoader,
        rng: &mut RandomNumberGenerator,
    ) -> Self {
        Self { time: 0i32 }
    }
}
impl RunnableGame for PacCrabGame {
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.time += time;
        println!("RUNNING PACCRAB");
        GameState::Running(Games::PacCrab)
    }
    fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) -> Option<()> {
        Some(())
    }
}
