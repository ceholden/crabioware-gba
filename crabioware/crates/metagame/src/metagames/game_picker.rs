use alloc::box::Box;
use alloc::vec::Vec;

use crate::metagame::{MetaGame, MetaGameState};


// TODO -- this is where we have up/down menu to select minigames
pub struct GamePicker {
    games: Vec<Box<dyn Game>>,
}
