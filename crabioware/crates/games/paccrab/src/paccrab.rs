use agb::display::object::{OamIterator, OamUnmanaged, ObjectUnmanaged, SpriteLoader};
use agb::display::tiled::{
    MapLoan, RegularBackgroundSize, RegularMap, TileFormat, TiledMap, VRamManager,
};
use agb::display::Priority;
use agb::input::{Button, ButtonController};
use agb::mgba::DebugLevel;
use agb::println;

use agb::rng::RandomNumberGenerator;
use crabioware_core::games::{Game, GameDifficulty, GameState, Games};
use crabioware_core::graphics::{GraphicsResource, Mode0TileMap, TileMapResource, TileMode};

use super::graphics::SpriteTag;
use super::levels::{Level, Levels};

pub struct PacCrabGame<'g> {
    time: i32,
    level: Level,
    tiles: Option<Mode0TileMap<'g>>,
}
impl<'g> PacCrabGame<'g> {
    pub fn new(difficulty: &GameDifficulty, rng: &mut RandomNumberGenerator) -> Self {
        Self {
            time: 0i32,
            level: Levels::LEVEL_1.get_level(),
            tiles: None,
        }
    }

    fn render_tiles(&self, bg1: &mut MapLoan<'g, RegularMap>, vram: &mut VRamManager) {
        self.level.set_background_paelttes(vram);

        let tileset = self.level.get_tileset();

        for y in 0..20u16 {
            for x in 0..30u16 {
                let tile_id = self.level.walls[(y * 30 + x) as usize] - 1;
                println!("x/y=({},{}) tile_id={}", x, y, tile_id);
                bg1.set_tile(
                    vram,
                    (x, y),
                    &tileset,
                    self.level.get_tilesetting(tile_id as usize),
                );
            }
        }
        bg1.commit(vram);
        bg1.set_visible(true);
    }
}
impl<'g> Game<'g> for PacCrabGame<'g> {
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.time += time;
        println!("RUNNING PACCRAB");
        if self.time < 200 {
            GameState::Running(Games::PacCrab)
        } else {
            GameState::GameOver
        }
    }

    fn renderer(&self) -> TileMode {
        TileMode::Mode0
    }

    fn clear(&mut self, vram: &mut VRamManager) {
        if let Some(tiles) = &mut self.tiles {
            tiles.clear(vram);
            tiles.commit(vram);
        }
    }

    fn init_tiles(&mut self, graphics: &'g GraphicsResource<'g>, vram: &mut VRamManager) {
        let mode0 = match graphics {
            GraphicsResource::Mode0(mode0) => mode0,
            _ => unimplemented!("WRONG MODE"),
        };

        let mut tiles = Mode0TileMap::default_32x32_4bpp(&mode0);
        tiles.bg1.set_visible(true);
        self.render_tiles(&mut tiles.bg1, vram);
        self.tiles = Some(tiles);
    }

    fn render(
        &mut self,
        vram: &mut VRamManager,
        unmanaged: &mut OamUnmanaged,
        sprite_loader: &mut SpriteLoader,
    ) -> Option<()> {
        let mut oam = unmanaged.iter();
        Some(())
    }
}
