use agb::{
    display::object::{OamIterator, SpriteLoader},
    input::ButtonController,
};

use super::states::GameState;

pub trait Game {
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState;
    fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) -> Option<()>;
}
