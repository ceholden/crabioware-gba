use agb::display::object::{OamIterator, ObjectUnmanaged, SpriteLoader};
use agb::display::tiled::{RegularBackgroundSize, TileFormat, TiledMap};
use agb::display::Priority;
use agb::input::{Button, ButtonController};
use agb::mgba::DebugLevel;
use agb::println;

use agb::rng::RandomNumberGenerator;
use crabioware_core::games::{GameDifficulty, GameState, Games, RunnableGame};

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

    pub fn test(self, mut gba: agb::Gba) {
        let (tiled, mut vram) = gba.display.video.tiled0();
        let vblank = agb::interrupt::VBlank::get();

        self.level.set_background_paelttes(&mut vram);
        let mut bg = tiled.background(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        let tileset = self.level.get_tileset();

        loop {
            for y in 0..20u16 {
                for x in 0..30u16 {
                    let tile_id = self.level.walls[(y * 30 + x) as usize] - 1;
                    println!("x/y=({},{}) tile_id={}", x, y, tile_id);
                    bg.set_tile(
                        &mut vram,
                        (x, y).into(),
                        &tileset,
                        self.level.get_tilesetting(
                            tile_id as usize
                        )
                    );
                }
            }
            bg.commit(&mut vram);
            bg.show();
        }
    }
}
impl RunnableGame for PacCrabGame {
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.time += time;
        println!("RUNNING PACCRAB");
        GameState::Running(Games::PacCrab)
    }
    fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) -> Option<()> {
        Some(())
    }
}
