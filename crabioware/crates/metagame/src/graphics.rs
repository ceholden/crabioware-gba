use agb::{
    display::{object::{OamUnmanaged, SpriteLoader}, tiled::{Tiled0, VRamManager}}, println, Gba
};
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;


#[derive(Debug)]
pub enum GameState {
    RUN,
    LOSE,
    WIN,
}


pub trait TileGraphics<'g> {
    fn new(gba: &'g mut Gba) -> Self;
    fn render(&mut self);
}

pub struct NotTiledResource<'g> {
    pub unmanaged: Box<OamUnmanaged<'g>>,
    pub sprite_loader: Box<SpriteLoader>,
}
impl<'g> TileGraphics<'g> for NotTiledResource<'g> {

    fn new(gba: &'g mut Gba) -> Self {
        let (unmanaged, sprite_loader) = gba.display.object.get_unmanaged();
        Self {
            unmanaged: Box::new(unmanaged),
            sprite_loader: Box::new(sprite_loader),
        }
    }

    fn render(&mut self) {
        println!("Rendering <NotTiledResource>");
    }
}

pub struct Tiled0Resource<'g> {
    pub tiled: Box<Tiled0<'g>>,
    pub vram: Box<VRamManager>,
    pub unmanaged: Box<OamUnmanaged<'g>>,
    pub sprite_loader: Box<SpriteLoader>,
}
impl<'g> TileGraphics<'g> for Tiled0Resource<'g> {

    fn new(gba: &'g mut Gba) -> Self {
        let (tiled, vram) = gba.display.video.tiled0();
        let (unmanaged, sprite_loader) = gba.display.object.get_unmanaged();
        Self {
            tiled: Box::new(tiled),
            vram: Box::new(vram),
            unmanaged: Box::new(unmanaged),
            sprite_loader: Box::new(sprite_loader),
        }
    }

    fn render(&mut self) {
        println!("Rendering <Tiled0Resource>");
    }
}

pub enum TileGraphicsMode {
    NotTiled,
    TiledMode0,
}
impl TileGraphicsMode {
    pub fn create<'g>(&self, gba: &'g mut Gba) -> TileGraphicsModeResource<'g> {
        match self {
            TileGraphicsMode::NotTiled => TileGraphicsModeResource::NotTiled(NotTiledResource::new(gba)),
            TileGraphicsMode::TiledMode0 => TileGraphicsModeResource::TiledMode0(Tiled0Resource::new(gba)),
        }
    }
}

pub enum TileGraphicsModeResource<'g> {
    NotTiled(NotTiledResource<'g>),
    TiledMode0(Tiled0Resource<'g>),
}

pub trait RunnableGame {
    fn print(&self) {
        println!("Running game");
    }

    fn advance(&mut self, time: i32) -> GameState;

}
pub trait Renderable {

    fn renderer(&self) -> TileGraphicsMode;

    fn render<'g>(&self, graphics: &TileGraphicsModeResource<'g>) {
        match graphics {
            TileGraphicsModeResource::NotTiled(gfx) => {
                println!("Rendering WITHOUT tile background");
            },
            TileGraphicsModeResource::TiledMode0(gfx) => {
                println!("Rendering WITH tile background");
            }
        }
    }
}

pub trait Game : RunnableGame + Renderable {}


pub struct NotTiledGame {
    time: i32
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
    fn new<'g>() -> Self where Self: Sized {
        Self {
            time: 0i32,
        }
    }
}
impl Renderable for NotTiledGame {

    fn renderer(&self) -> TileGraphicsMode {
        TileGraphicsMode::NotTiled
    }

    fn render<'g>(&self, graphics: &TileGraphicsModeResource<'g>) {
        match graphics {
            TileGraphicsModeResource::NotTiled(gfx) => {
                println!("Rendering WITHOUT tile background");
            },
            _ => unimplemented!("Unsupported mode")
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

    fn renderer(&self) -> TileGraphicsMode {
        TileGraphicsMode::TiledMode0
    }

    fn render<'g>(&self, graphics: &TileGraphicsModeResource<'g>) {
        match graphics {
            TileGraphicsModeResource::TiledMode0(gfx) => {
                println!("Rendering WITH tile background");
            },
            _ => unimplemented!("Unsupported mode")
        };
    }
}
impl Tiled0Game {
    pub fn new<'g>() -> Self where Self: Sized {
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
    games: Vec::<Games>
}
impl GameRunner {

    pub fn new() -> Self {
        Self {
            games: vec![
                Games::GAME1,
                Games::GAME2,
            ]
        }
    }

    pub fn new_game(&self, game: Games) -> Box<dyn Game> {
        match game {
            Games::GAME1 => Box::new(NotTiledGame::new()),
            Games::GAME2 => Box::new(Tiled0Game::new())
        }
    }

    pub fn run(&self) -> ! {
        loop {

        }
    }
}


pub fn test(gba: &mut Gba) {
    let game_runner = GameRunner::new();

    let mut selected_game = game_runner.games[0].clone();

    let mut game = game_runner.new_game(selected_game);
    let mut graphics = game.renderer().create(gba);

    loop {

        match game.advance(1) {
            GameState::RUN => {
                game.render(&graphics);
            },
            GameState::WIN => {
                println!("YOU WIN");
                selected_game = selected_game.next();
                game = game_runner.new_game(selected_game);
                graphics = game.renderer().create(gba);
            },
            GameState::LOSE => {
                println!("YOU LOSE");
                selected_game = selected_game.next();
                game = game_runner.new_game(selected_game);
                graphics = game.renderer().create(gba);
            },
        }

    }
}
