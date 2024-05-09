use agb::display::object::{OamIterator, ObjectUnmanaged, SpriteLoader};
use agb::display::tiled::{RegularBackgroundSize, TileFormat, TiledMap};
use agb::display::Priority;
use agb::input::{Button, ButtonController};
use agb::mgba::DebugLevel;
use agb::println;

use agb::rng::RandomNumberGenerator;
use crabioware_core::games::{GameDifficulty, GameState, Games, RunnableGame};
use crabioware_core::graphics::{
    GraphicsMode, GraphicsResource, Mode0TileMap, TileMap, Tiled0Resource,
};

use super::graphics::SpriteTag;
use super::levels::{Level, Levels};

pub struct PacCrabGame {
    time: i32,
    level: Level,
}
impl Default for PacCrabGame {
    fn default() -> Self {
        Self {
            time: 0i32,
            level: Levels::LEVEL_1.get_level(),
        }
    }
}
impl PacCrabGame {
    pub fn new(
        difficulty: &GameDifficulty,
        _: &mut SpriteLoader,
        rng: &mut RandomNumberGenerator,
    ) -> Self {
        Self {
            time: 0i32,
            level: Levels::LEVEL_1.get_level(),
        }
    }

    //    pub fn test(self, mut gba: agb::Gba) {
    //        let (tiled, mut vram) = gba.display.video.tiled0();
    // pub fn render_level(&mut self, gfx: &mut Tiled0Resource) {
    fn render_level<'g>(&self, gfx: &mut Tiled0Resource, mode0: &mut Mode0TileMap) -> Option<()> {
        self.level.set_background_paelttes(&mut gfx.vram);

        let tileset = self.level.get_tileset();

        for y in 0..20u16 {
            for x in 0..30u16 {
                let tile_id = self.level.walls[(y * 30 + x) as usize] - 1;
                println!("x/y=({},{}) tile_id={}", x, y, tile_id);
                mode0.bg1.set_tile(
                    &mut gfx.vram,
                    (x, y),
                    &tileset,
                    self.level.get_tilesetting(tile_id as usize),
                );
            }
        }
        mode0.bg1.commit(&mut gfx.vram);
        mode0.bg1.set_visible(true);
        mode0.dirty = false;
        Some(())
    }
}
impl RunnableGame for PacCrabGame {
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.time += time;
        println!("RUNNING PACCRAB");
        GameState::Running(Games::PacCrab)
    }

    fn renderer(&self) -> GraphicsMode {
        GraphicsMode::Mode0
    }

    fn tilemaps<'g>(&'g self, graphics: &'g mut GraphicsResource<'g>) -> TileMap<'g> {
        let gfx = match graphics {
            GraphicsResource::Mode0(gfx) => gfx,
            _ => unimplemented!("WRONG MODE"),
        };
        let bg1 = gfx.tiled.background(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        let bg2 = gfx.tiled.background(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        TileMap::Mode0(Mode0TileMap::new(bg1, bg2))
    }

    fn render_map<'g>(
        &self,
        graphics: &mut GraphicsResource<'g>,
        tilemap: &mut TileMap<'g>,
    ) -> Option<()> {
        let gfx = match graphics {
            GraphicsResource::Mode0(gfx) => gfx,
            _ => unimplemented!("WRONG MODE"),
        };
        let mut mode0 = match tilemap {
            TileMap::Mode0(tilemap_) => tilemap_,
            _ => unimplemented!("WRONG MODE"),
        };
        if mode0.dirty {
            self.render_level(gfx, &mut mode0);
        };
        Some(())
    }

    fn render<'g>(&self, graphics: &mut GraphicsResource<'g>) -> Option<()> {
        let gfx = match graphics {
            GraphicsResource::Mode0(gfx) => gfx,
            _ => unimplemented!("WRONG MODE"),
        };
        // let oam = gfx.unmanaged.iter();
        // let loader = &mut gfx.sprite_loader;

        Some(())
    }
}
