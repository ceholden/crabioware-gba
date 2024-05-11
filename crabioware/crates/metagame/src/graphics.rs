use agb::{
    display::{
        object::{OamUnmanaged, SpriteLoader},
        palette16::Palette16,
        tiled::{
            AffineBackgroundSize, MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled0, TiledMap, VRamManager
        },
        Priority,
    }, include_background_gfx, interrupt::VBlank, println, Gba
};
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use crate::resources::{Mode0TileMap, Mode1TileMap, TileMapResource};
use crate::{
    metagame::{MetaGame, MetaGameState},
    resources::{TiledMode, TiledModeResource},
};

#[derive(Debug)]
pub enum GameState {
    RUN,
    LOSE,
    WIN,
}

pub trait Game<'a, 'b> {
    fn advance(&mut self, time: i32) -> GameState;

    fn renderer(&self) -> TiledMode {
        TiledMode::Mode0
    }
    fn init_tiles(&mut self, tiled: &'a TiledModeResource<'b>, vram: &mut VRamManager, vblank: &VBlank);
    fn render(&self, vram: &mut VRamManager);
    fn clear(&mut self, vram: &mut VRamManager) {}
}


include_background_gfx!(water_tiles, "3f3f74", water_tiles => 256 "water_tiles.png");

pub struct Mode1Game<'a> {
    time: i32,
    max_score: i32,
    tiles: Option<Mode1TileMap<'a>>,
}
impl<'a> Mode1Game<'a> {
    fn new() -> Self {
        Self {
            time: 0i32,
            max_score: 100,
            tiles: None,
        }
    }
}
impl<'a, 'b> Game<'a, 'b> for Mode1Game<'a> {
    fn advance(&mut self, time: i32) -> GameState {
        self.time += time;
        if self.time < 100 {
            GameState::RUN
        } else if agb::rng::gen().rem_euclid(2) == 0 {
            GameState::WIN
        } else {
            GameState::LOSE
        }
    }

    fn init_tiles(&mut self, tiled: &'a TiledModeResource<'b>, vram: &mut VRamManager, vblank: &VBlank) {
        let mode1 = match tiled {
            TiledModeResource::Mode1(gfx) => gfx,
            _ => unimplemented!("WRONG MODE"),
        };

        let tileset = water_tiles::water_tiles.tiles;
        println!("WATER TILESET FORMAT={:?}", tileset.format());

        println!("INIT TILES FOR MODE1");
        let mut bg1 = mode1.regular(
            Priority::P1,
            RegularBackgroundSize::Background32x32,
            tileset.format(),
        );
        let mut bg2 = mode1.regular(
            Priority::P2,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        let mut affine = mode1.affine(
            Priority::P0,
            AffineBackgroundSize::Background32x32,
        );

        vram.set_background_palettes(water_tiles::PALETTES);
        for y in 0..20u16 {
            for x in 0..30u16 {
                affine.set_tile(vram, (x, y), &tileset, 1);
            }
        }
        bg1.set_visible(false);
        bg2.set_visible(false);
        affine.set_visible(true);

        let mut tiles = Mode1TileMap::new(bg1, bg2, affine);

        tiles.commit(vram);
        println!("DONE INIT TILES FOR MODE1");
    }

    fn clear(&mut self, vram: &mut VRamManager) {
        if let Some(tiles) = &mut self.tiles {
            tiles.clear(vram);
            tiles.commit(vram);
        }
    }

    fn render(&self, vram: &mut VRamManager) {}
}

pub struct Mode0Game<'a> {
    time: i32,
    max_score: i32,
    tiles: Option<Mode0TileMap<'a>>,
}
impl<'a> Mode0Game<'a> {
    pub fn new() -> Self {
        Self {
            time: 0i32,
            max_score: 150,
            tiles: None,
        }
    }

    fn render_tiles(&self, bg1: &mut MapLoan<'a, RegularMap>, vram: &mut VRamManager) {
        for y in 0..20u32 {
            for x in 0..30u32 {
                let dynamic_tile = vram.new_dynamic_tile();

                for (i, bit) in dynamic_tile.tile_data.iter_mut().enumerate() {
                    let i = i as u32;
                    let mut value = 0;

                    for j in 0..4 {
                        value |= (value << 8) | ((x + i) % 8) | ((y + j) % 8) << 4;
                    }

                    *bit = value;
                }

                bg1.set_tile(
                    vram,
                    (x as u16, y as u16),
                    &dynamic_tile.tile_set(),
                    dynamic_tile.tile_setting(),
                );
                vram.remove_dynamic_tile(dynamic_tile);
            }
        }
    }
}
impl<'a, 'b> Game<'a, 'b> for Mode0Game<'a> {
    fn advance(&mut self, time: i32) -> GameState {
        self.time += time;
        if self.time < self.max_score {
            GameState::RUN
        } else if agb::rng::gen().rem_euclid(2) == 0 {
            GameState::WIN
        } else {
            GameState::LOSE
        }
    }

    fn renderer(&self) -> TiledMode {
        TiledMode::Mode0
    }

    fn render(&self, vram: &mut VRamManager) {}

    fn clear(&mut self, vram: &mut VRamManager) {
        if let Some(tiles) = &mut self.tiles {
            tiles.clear(vram);
            tiles.commit(vram);
        }
    }

    fn init_tiles(&mut self, tiled: &'a TiledModeResource<'b>, vram: &mut VRamManager, vblank: &VBlank) {
        println!("INIT TILES FOR MODE0");
        let mode0 = match tiled {
            TiledModeResource::Mode0(gfx) => gfx,
            _ => unimplemented!("WRONG MODE"),
        };
        let mut bg1 = mode0.background(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        let mut bg2 = mode0.background(
            Priority::P1,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        let mut bg3 = mode0.background(
            Priority::P2,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        let mut bg4 = mode0.background(
            Priority::P3,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        vram.set_background_palettes(&[Palette16::new([
            0xff00, 0x0ff0, 0x00ff, 0xf00f, 0xf0f0, 0x0f0f, 0xaaaa, 0x5555, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        ])]);

        self.render_tiles(&mut bg1, vram);
        bg1.commit(vram);

        let mut tiles = Mode0TileMap::new(bg1, bg2, bg3, bg4);
        tiles.set_visible(true);

        self.tiles = Some(tiles);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Games {
    GAME1,
    GAME2,
}
impl Games {
    pub fn next(&self) -> Games {
        match self {
            Games::GAME1 => Games::GAME2,
            Games::GAME2 => Games::GAME1,
        }
    }

    pub fn load(&self) -> Box<dyn Game + '_> {
        match self {
            Games::GAME1 => Box::new(Mode1Game::new()),
            Games::GAME2 => Box::new(Mode0Game::new()),
        }
    }

    pub fn renderer(&self) -> TiledMode {
        match self {
            Games::GAME1 => TiledMode::Mode1,
            Games::GAME2 => TiledMode::Mode0,
        }
    }
}

pub struct GamePicker {
    games: Vec<Games>,
}
impl GamePicker {
    pub fn new() -> Self {
        Self {
            games: vec![Games::GAME1, Games::GAME2],
        }
    }
}
impl MetaGame for GamePicker {
    fn run(&self, gba: &mut Gba, state: &MetaGameState) -> MetaGameState {
        MetaGameState::RUNNING
    }
    fn load(&self, game: &Games) -> Box<dyn Game + '_> {
        match game {
            Games::GAME1 => Box::new(Mode1Game::new()),
            Games::GAME2 => Box::new(Mode0Game::new()),
        }
    }
}

pub fn test(gba: &mut Gba, metagame: &impl MetaGame, state: &MetaGameState) -> MetaGameState {
    let vblank = agb::interrupt::VBlank::get();

    let mut selected_game = *match state {
        MetaGameState::RUNNING => unimplemented!("SHOULD NOT BE RUNNING"),
        MetaGameState::START(game) => game,
    };

    let (graphics, mut vram) = selected_game.renderer().create(gba);
    let mut game = metagame.load(&selected_game);
    // game.clear(&mut vram);
    game.init_tiles(&graphics, &mut vram, &vblank);

    loop {
        match game.advance(1) {
            GameState::RUN => {
                // println!("RENDERING {selected_game:?}");
                game.render(&mut vram);
            }
            GameState::WIN => {
                println!("YOU WIN");
                selected_game = metagame.next(&selected_game);
                game.clear(&mut vram);
                vblank.wait_for_vblank();
                return MetaGameState::START(selected_game);
                // game = game_runner.load(&selected_game);
                // let (graphics, vram) = game.renderer().create(gba);
                // game.init_tiles(&graphics, &mut vram);
            }
            GameState::LOSE => {
                println!("YOU LOSE");
                selected_game = metagame.next(&selected_game);
                game.clear(&mut vram);
                vblank.wait_for_vblank();
                return MetaGameState::START(selected_game);
                // game = game_runner.load(&selected_game);
                // let (graphics, vram) = game.renderer().create(gba);
                // game.init_tiles(&graphics, &mut vram);
            }
        }
        vblank.wait_for_vblank();
    }
}
