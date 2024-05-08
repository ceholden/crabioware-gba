use agb::{
    display::{
        object::{OamUnmanaged, SpriteLoader},
        tiled::{MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled0, VRamManager},
        Priority,
    },
    println, Gba,
};
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use crate::resources::{GraphicsMode, GraphicsResource, Mode0TileMap, TileMap, TileMode};

#[derive(Debug)]
pub enum GameState {
    RUN,
    LOSE,
    WIN,
}

pub trait RunnableGame {
    fn print(&self) {
        println!("Running game");
    }

    fn advance(&mut self, time: i32) -> GameState;
}
pub trait Renderable {
    fn renderer(&self) -> GraphicsMode;
    fn render<'g>(&self, graphics: &GraphicsResource<'g>) {
        match graphics {
            GraphicsResource::NotTiled(gfx) => {
                println!("Rendering WITHOUT tile background");
            }
            GraphicsResource::Mode0(gfx) => {
                println!("Rendering WITH tile background");
            }
        }
    }

    fn tiles<'g>(&self) -> Box<dyn Fn(&TileMode<'g>) -> TileMap<'g>> {
        Box::new(|_: &TileMode<'g>| TileMap::NotTiled)
    }
}

pub trait Game: RunnableGame + Renderable {}

pub struct NotTiledGame {
    time: i32,
}
impl Game for NotTiledGame {}
impl RunnableGame for NotTiledGame {
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
impl NotTiledGame {
    fn new<'g>() -> Self
    where
        Self: Sized,
    {
        Self { time: 0i32 }
    }
}
impl Renderable for NotTiledGame {
    fn renderer(&self) -> GraphicsMode {
        GraphicsMode::NotTiled
    }

    fn render<'g>(&self, graphics: &GraphicsResource<'g>) {
        match graphics {
            GraphicsResource::NotTiled(gfx) => {
                println!("Rendering WITHOUT tile background");
            }
            _ => unimplemented!("Unsupported mode"),
        };
    }
}

pub struct Tiled0Game {
    time: i32,
    ball: i32,
}
impl Game for Tiled0Game {}
impl RunnableGame for Tiled0Game {
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
impl Renderable for Tiled0Game {
    fn renderer(&self) -> GraphicsMode {
        GraphicsMode::Mode0
    }

    fn render<'g>(&self, graphics: &GraphicsResource<'g>) {
        match graphics {
            GraphicsResource::Mode0(gfx) => {
                println!("Rendering WITH tile background");
            }
            _ => unimplemented!("Unsupported mode"),
        };
    }

    fn tiles<'g>(&self) -> Box<dyn Fn(&TileMode<'g>) -> TileMap<'g>> {
        Box::new(|mode: &TileMode<'g>| {
            let mode0 = match mode {
                TileMode::Mode0(mode0) => mode0,
                _ => unimplemented!("WRONG MODE"),
            };
            let bg1 = mode0.background(
                Priority::P0,
                RegularBackgroundSize::Background32x32,
                TileFormat::FourBpp,
            );
            let bg2 = mode0.background(
                Priority::P1,
                RegularBackgroundSize::Background32x32,
                TileFormat::FourBpp,
            );
            TileMap::Mode0(Mode0TileMap::new(Box::new(bg1), Box::new(bg2)))
        })
    }
}
impl Tiled0Game {
    pub fn new<'g>() -> Self
    where
        Self: Sized,
    {
        Self {
            time: 0i32,
            ball: 5i32,
        }
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
}

pub struct GameRunner {
    games: Vec<Games>,
}
impl GameRunner {
    pub fn new() -> Self {
        Self {
            games: vec![Games::GAME1, Games::GAME2],
        }
    }

    pub fn new_game(&self, game: Games) -> Box<dyn Game> {
        match game {
            Games::GAME1 => Box::new(NotTiledGame::new()),
            Games::GAME2 => Box::new(Tiled0Game::new()),
        }
    }

    pub fn run(&self) -> ! {
        loop {}
    }
}

pub fn test(gba: &mut Gba) -> ! {
    let vblank = agb::interrupt::VBlank::get();

    let game_runner = GameRunner::new();

    let mut selected_game = game_runner.games[0].clone();

    let mut game = game_runner.new_game(selected_game);
    let mut tiles = game.tiles();
    let mut graphics = game.renderer().create(gba, tiles);

    loop {
        match game.advance(1) {
            GameState::RUN => {
                game.render(&graphics);
            }
            GameState::WIN => {
                println!("YOU WIN");
                selected_game = selected_game.next();
                game = game_runner.new_game(selected_game);
                graphics = game.renderer().create(gba);
            }
            GameState::LOSE => {
                println!("YOU LOSE");
                selected_game = selected_game.next();
                game = game_runner.new_game(selected_game);
                graphics = game.renderer().create(gba);
            }
        }
        vblank.wait_for_vblank();
    }
}
