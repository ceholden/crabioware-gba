use agb::input::ButtonController;

use super::states::GameState;
use crate::graphics::{GraphicsMode, GraphicsResource, TileMap};

pub trait RunnableGame {
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState;
    fn renderer(&self) -> GraphicsMode {
        GraphicsMode::NotTiled
    }
    fn render<'g>(&mut self, graphics: &mut GraphicsResource<'g>) -> Option<()>;
    fn render_map<'g, 'm>(&mut self, _: &mut GraphicsResource<'g>, _: &mut TileMap<'g>) -> Option<()> {
        Some(())
    }
    fn tilemaps<'g, 'm>(&'g self, _: &mut GraphicsResource<'g>) -> TileMap<'m> {
        TileMap::NotTiled
    }
}
