use agb::display::object::OamIterator;
use agb::display::object::SpriteLoader;
use agb::display::tiled::VRamManager;
use agb::input::ButtonController;

use crate::graphics::TileMode;
use crate::graphics::GraphicsResource;

use super::games::Games;
use super::game_state::GameState;


pub trait Game<'a, 'b> {
    fn renderer(&self) -> TileMode {
        TileMode::Mode0
    }
    fn clear(&mut self, vram: &mut VRamManager) {}
    // Default impl has no background tiles (e.g., pong, snake)
    fn init_tiles(&mut self, graphics: &'a GraphicsResource<'b>, vram: &mut VRamManager) {}

    fn render(
        &mut self,
        graphics: &'b mut GraphicsResource<'a>, 
        sprite_loader: &mut SpriteLoader,
        vram: &mut VRamManager,
    ) -> Option<()>;
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState;
}
