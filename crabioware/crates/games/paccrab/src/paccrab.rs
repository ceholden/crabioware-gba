use core::borrow::BorrowMut;

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
    tiles_dirty: bool,
}
impl Default for PacCrabGame {
    fn default() -> Self {
        Self {
            time: 0i32,
            level: Levels::LEVEL_1.get_level(),
            tiles_dirty: true,
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
            tiles_dirty: true,
        }
    }

    //    pub fn test(self, mut gba: agb::Gba) {
    //        let (tiled, mut vram) = gba.display.video.tiled0();
    // pub fn render_level(&mut self, gfx: &mut Tiled0Resource) {
    fn render_level<'g>(
        &mut self,
        gfx: &mut Tiled0Resource,
        mode0: &mut Mode0TileMap,
    ) -> Option<()> {
        self.level.set_background_paelttes(&mut gfx.vram);

        let tileset = self.level.get_tileset();

        loop {
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
        }
        self.tiles_dirty = false;
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

    fn tilemaps<'g, 'm>(&'g self, graphics: &mut GraphicsResource<'g>) -> TileMap<'m> {
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
        TileMap::Mode0(Mode0TileMap { bg1, bg2 })
    }

    fn render_map<'g>(
        &mut self,
        graphics: &mut GraphicsResource<'g>,
        tilemap: &mut TileMap<'g>,
    ) -> Option<()> {
        let gfx = match graphics {
            GraphicsResource::Mode0(gfx) => gfx,
            _ => unimplemented!("WRONG MODE"),
        };
        let mode0 = match tilemap {
            TileMap::Mode0(tilemap_) => tilemap_,
            _ => unimplemented!("WRONG MODE"),
        };
        if self.tiles_dirty {
            self.render_level(gfx, mode0);
        };
        Some(())
    }

    fn render<'g>(&mut self, graphics: &mut GraphicsResource<'g>) -> Option<()> {
        let gfx = match graphics {
            GraphicsResource::Mode0(gfx) => gfx,
            _ => unimplemented!("WRONG MODE"),
        };
        // let oam = gfx.unmanaged.iter();
        // let loader = &mut gfx.sprite_loader;

        Some(())
    }
}
