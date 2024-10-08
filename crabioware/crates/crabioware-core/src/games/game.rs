use agb::display::object::OamIterator;
use agb::display::object::OamUnmanaged;
use agb::display::object::SpriteLoader;
use agb::display::tiled::VRamManager;
use agb::input::ButtonController;

use crate::graphics::GraphicsResource;
use crate::graphics::TileMode;

use super::game_state::GameState;
use super::games::Games;

pub trait Game<'g> {
    fn renderer(&self) -> TileMode {
        TileMode::Mode0
    }
    fn clear(&mut self, vram: &mut VRamManager) {}
    // Default impl has no background tiles (e.g., pong, snake)
    fn init_tiles(&mut self, graphics: &'g GraphicsResource<'g>, vram: &mut VRamManager) {}

    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState;
    fn render(
        &mut self,
        vram: &mut VRamManager,
        unmanaged: &mut OamUnmanaged,
        sprite_loader: &mut SpriteLoader,
    ) -> Option<()>;
}
