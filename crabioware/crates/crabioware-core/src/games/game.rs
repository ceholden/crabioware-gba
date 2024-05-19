use agb::display::object::OamIterator;
use agb::display::object::SpriteLoader;
use agb::display::tiled::VRamManager;
use agb::input::ButtonController;

use crate::graphics::TileMode;
use crate::graphics::TileModeResource;

use super::games::Games;
use super::game_state::GameState;


pub trait Game<'a, 'b> {
    fn renderer(&self) -> TileMode {
        TileMode::Mode0
    }
    fn clear(&mut self, vram: &mut VRamManager) {}
    // Default impl has no background tiles (e.g., pong, snake)
    fn init_tiles(&mut self, tiled: &'a TileModeResource<'b>, vram: &mut VRamManager) {}

    fn render(
        &mut self,
        loader: &mut SpriteLoader,
        oam: &mut OamIterator,
        vram: &mut VRamManager,
    ) -> Option<()>;
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState;
}
