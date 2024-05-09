use agb::Gba;
use alloc::boxed::Box;

use crate::graphics::{Game, Games};

pub enum MetaGameState {
    START(Games),
    RUNNING,
}

pub trait MetaGame {
    fn pick(&self) -> Games {
        Games::GAME1
    }
    fn next(&self, current: &Games) -> Games {
        current.next()
    }
    fn run(&self, gba: &mut Gba, state: &MetaGameState) -> MetaGameState;
    fn load(&self, game: &Games) -> Box<dyn Game + '_>;
}
