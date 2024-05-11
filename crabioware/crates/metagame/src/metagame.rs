use agb::{interrupt::VBlank, Gba};
use alloc::boxed::Box;

use crate::graphics::{Game, Games};

pub enum MetaGameState {
    START(MetaGameType),
    RUNNING(MetaGameType),
    WIN,
    LOSE,
}

pub trait MetaGame {
    fn pick(&self) -> Games {
        Games::GAME1
    }
    fn next(&self, current: &Games) -> Games {
        current.next()
    }
    fn run(&self, gba: &mut Gba, vblank: &VBlank) -> MetaGameState;
    fn load(&self, game: &Games) -> Box<dyn Game + '_>;
}


pub enum MetaGameType {
    PICKER,
}
