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

use crate::{metagame::{MetaGame, MetaGameState}, resources::Mode0Resource};
use crate::resources::{GraphicsMode, GraphicsResource, Mode0TileMap, TileMap, TileMode};

#[derive(Debug)]
pub enum GameState {
    RUN,
    LOSE,
    WIN,
}

pub trait Game<'g> {
    fn advance(&mut self, time: i32) -> GameState;

    fn renderer(&self) -> GraphicsMode;

    fn init_tiles(&mut self, _: &'g GraphicsResource<'g>) {}

    fn render(&self, graphics: &GraphicsResource<'g>) {
        match graphics {
            GraphicsResource::NotTiled(gfx) => {
                println!("Rendering WITHOUT tile background");
            }
            GraphicsResource::Mode0(gfx) => {
                println!("Rendering WITH tile background");
            }
        }
    }
}

pub struct NotTiledGame {
    time: i32,
}
impl NotTiledGame {
    fn new<'g>() -> Self {
        Self { time: 0i32 }
    }
}
impl<'g> Game<'g> for NotTiledGame {
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
    fn renderer(&self) -> GraphicsMode {
        GraphicsMode::NotTiled
    }

    fn render(&self, graphics: &GraphicsResource<'g>) {
        match graphics {
            GraphicsResource::NotTiled(gfx) => {
                println!("Rendering WITHOUT tile background");
            }
            _ => unimplemented!("Unsupported mode"),
        };
    }
}


pub struct Tiled0Game<'m> {
    time: i32,
    ball: i32,
    tiles: Option<Mode0TileMap<'m>>,
}
impl<'m, 'g> Tiled0Game<'m> {

    pub fn new() -> Self {
        Self {
            tiles: None,
            time: 0i32,
            ball: 5i32,
        }
    }

//        gfx.vram.set_background_palettes(&[agb::display::palette16::Palette16::new([
//            0xff00, 0x0ff0, 0x00ff, 0xf00f, 0xf0f0, 0x0f0f, 0xaaaa, 0x5555, 0x0000, 0x0000, 0x0000,
//            0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
//        ])]);
//
//        for y in 0..20u32 {
//            for x in 0..30u32 {
//                let dynamic_tile = gfx.vram.new_dynamic_tile();
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
//                self.tiles.unwrap().bg1.set_tile(
//                    &mut gfx.vram,
//                    (x as u16, y as u16),
//                    &dynamic_tile.tile_set(),
//                    dynamic_tile.tile_setting(),
//                );
//
//                gfx.vram.remove_dynamic_tile(dynamic_tile);
//            }
//        }
//
//        self.tiles.unwrap().bg1.commit(&mut gfx.vram);
//        self.tiles.unwrap().bg1.set_visible(true);

}
impl<'m, 'g> Game<'g> for Tiled0Game<'m>
where 'g: 'm
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

    fn renderer(&self) -> GraphicsMode {
        GraphicsMode::Mode0
    }

    fn render(&self, graphics: &GraphicsResource<'g>) {
        match graphics {
            GraphicsResource::Mode0(gfx) => {
                println!("Rendering WITH tile background");
            }
            _ => unimplemented!("Unsupported mode"),
        };
    }

    fn init_tiles(&mut self, graphics: &'g GraphicsResource<'g>) {
        let gfx = match graphics {
            GraphicsResource::Mode0(gfx) => gfx,
            _ => unimplemented!("WRONG MODE")
        };
        let bg1 = gfx.tile0.background(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        self.tiles = Some(Mode0TileMap::new(bg1));

    }

}


#[derive(Clone, Copy)]
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

    pub fn load<'g>(&self) -> Box<dyn Game<'g> {
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
}

pub fn test(gba: &mut Gba) -> ! {
    let vblank = agb::interrupt::VBlank::get();

    let game_runner = GamePicker::new();

    let mut selected_game = game_runner.games[0].clone();

    let mut game = selected_game.load();
    let mut graphics = game.renderer().create(gba);
    game.init_tiles(&graphics);

    loop {
    //    match game.advance(1) {
    //        GameState::RUN => {
    //            game.render(&graphics);
    //        }
    //        GameState::WIN => {
    //            println!("YOU WIN");
    //            selected_game = selected_game.next();
    //            game = selected_game.load();
    //            graphics = game.renderer().create(gba);
    //            game.init_tiles(&graphics);
    //        }
    //        GameState::LOSE => {
    //            println!("YOU LOSE");
    //            game = selected_game.next().load();
    //            graphics = game.renderer().create(gba);
    //            game.init_tiles(&graphics);
    //        }
    //    }
    //    vblank.wait_for_vblank();
    }
}
