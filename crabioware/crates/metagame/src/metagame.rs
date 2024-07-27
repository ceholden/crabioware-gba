use agb::{input::ButtonController, interrupt::VBlank, Gba};
use crabioware_core::games::GameLoader;

pub enum MetaGameState {
    START(MetaGameType),
    RUNNING(MetaGameType),
    WIN,
    LOSE,
}

pub trait MetaGame {
    fn run(
        &self,
        gba: &mut Gba,
        vblank: &VBlank,
        buttons: &mut ButtonController,
        loader: &impl GameLoader,
    ) -> MetaGameState;
}

pub enum MetaGameType {
    PICKER,
}
