use agb::{interrupt::VBlank, Gba};
use alloc::boxed::Box;
use crabioware_core::games::{Game, Games};




pub enum MetaGameState {
    START(MetaGameType),
    RUNNING(MetaGameType),
    WIN,
    LOSE,
}

pub trait MetaGame {
    fn pick(&self) -> Games;
    fn next(&self, current: &Games) -> Games;
    fn run(&self, gba: &mut Gba, vblank: &VBlank) -> MetaGameState;
    fn load(&self, game: &Games) -> Box<impl Game + '_>;
}

pub enum MetaGameType {
    PICKER,
}
