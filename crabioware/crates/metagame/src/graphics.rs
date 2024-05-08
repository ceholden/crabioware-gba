use agb::{
    display::{
        object::{OamUnmanaged, SpriteLoader},
        tiled::{TiledMap, MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled0, VRamManager},
        Priority,
    },
    println, Gba,
};
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use crate::{metagame::{MetaGame, MetaGameState}, resources::{NotTiledResource, TiledMode, TiledResource}};
use crate::resources::{Mode0TileMap, TileModeMap};

#[derive(Debug)]
pub enum GameState {
    RUN,
    LOSE,
    WIN,
}

pub trait Game<'a, 'b> {
    fn advance(&mut self, time: i32) -> GameState;

    fn renderer(&self) -> TiledMode {
        TiledMode::NotTiled
    }
    fn init_tiles(
        &mut self,
        tiled: &'a TiledResource<'a>,
        vram: &mut VRamManager,
    ) {}
    fn render(
        &self,
        vram: &mut VRamManager,
    ) {}
}

pub struct NotTiledGame {
    time: i32,
}
impl NotTiledGame {
    fn new<'g>() -> Self {
        Self { time: 0i32 }
    }
}
impl<'a, 'b> Game<'a, 'b> for NotTiledGame {
    fn advance(&mut self, time: i32) -> GameState {
        self.time += time;
        if self.time < 10 {
            GameState::RUN
        } else if agb::rng::gen().rem_euclid(2) == 0 {
            GameState::WIN
        } else {
            GameState::LOSE
        }
    }
}


pub struct Tiled0Game<'a> {
    time: i32,
    ball: i32,
    tiles: Option<TileModeMap<'a>>,
    // tiles: Option<MapLoan<'a, RegularMap>>,
}
impl<'a> Tiled0Game<'a> {

    pub fn new() -> Self {
        Self {
            tiles: None,
            time: 0i32,
            ball: 5i32,
        }
    }

}
impl<'a, 'b> Game<'a, 'b> for Tiled0Game<'a>
{
    fn advance(&mut self, time: i32) -> GameState {
        self.time += time;
        if self.time < 10 {
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

    fn render(
        &self,
        vram: &mut VRamManager,
    ) {}

    fn init_tiles(
        &mut self,
        tiled: &'a TiledResource<'a>,
        vram: &mut VRamManager,
    ) {
        let mut mode0 = match tiled {
            TiledResource::Mode0(gfx) => gfx,
            _ => unimplemented!("WRONG MODE")
        };
        let mut bg1 = mode0.tiled.background(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        self.tiles = Some(TileModeMap::Mode0(Mode0TileMap::new(bg1)));

        vram.set_background_palettes(&[agb::display::palette16::Palette16::new([
            0xff00, 0x0ff0, 0x00ff, 0xf00f, 0xf0f0, 0x0f0f, 0xaaaa, 0x5555, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        ])]);

//        for y in 0..20u32 {
//            for x in 0..30u32 {
//                let dynamic_tile = mode0.vram.new_dynamic_tile();
//
//                for (i, bit) in dynamic_tile.tile_data.iter_mut().enumerate() {
//                    let i = i as u32;
//                    let mut value = 0;
//
//                    for j in 0..4 {
//                        value |= (value << 8) | ((x + i) % 8) | ((y + j) % 8) << 4;
//                    }
//
//                    *bit = value;
//                }
//
//                bg1.set_tile(
//                    &mut mode0.vram,
//                    (x as u16, y as u16),
//                    &dynamic_tile.tile_set(),
//                    dynamic_tile.tile_setting(),
//                );
//
//                mode0.vram.remove_dynamic_tile(dynamic_tile);
//            }
//        }
//
//        bg1.commit(&mut mode0.vram);
//        bg1.set_visible(true);

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
            Games::GAME1 => Box::new(NotTiledGame::new()),
            Games::GAME2 => Box::new(Tiled0Game::new()),
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
            Games::GAME1 => Box::new(NotTiledGame::new()),
            Games::GAME2 => Box::new(Tiled0Game::new()),
        }
    }
}

pub fn test(gba: &mut Gba, metagame: impl MetaGame) -> MetaGameState {
    let vblank = agb::interrupt::VBlank::get();

    let mut selected_game = metagame.pick();

    let mut game = metagame.load(&selected_game);
    let (graphics, mut vram) = game.renderer().create(gba);
    game.init_tiles(&graphics, &mut vram);

    game.render(&mut vram);
    game.advance(1);

    loop {
        match game.advance(1) {
            GameState::RUN => {
                game.render(&mut vram);
            }
            GameState::WIN => {
                println!("YOU WIN");
                selected_game = selected_game.next();
                break
                // game = metagame.load(&selected_game);
                // let (graphics, vram) = game.renderer().create(gba);
                // game.init_tiles(&graphics, &mut vram);
            }
            GameState::LOSE => {
                println!("YOU LOSE");
                selected_game = selected_game.next();
                break
                // game = metagame.load(&selected_game);
                // let (graphics, vram) = game.renderer().create(gba);
                // game.init_tiles(&graphics, &mut vram);
            }
        }
        vblank.wait_for_vblank();
    }
    MetaGameState::START(metagame.next(&selected_game))
}
