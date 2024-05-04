use core::{cell::RefMut, marker::PhantomData};

use agb::{
    display::{
        object::{OamUnmanaged, SpriteLoader},
        tiled::{MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled0, VRamManager},
        Priority,
    },
    println, Gba,
};
use alloc::boxed::Box;

pub trait Graphics<'g> {
    fn new(gba: &'g mut Gba) -> Self;
    fn render(&mut self);
}

pub struct NotTiledResource<'g> {
    pub unmanaged: Box<OamUnmanaged<'g>>,
    pub sprite_loader: Box<SpriteLoader>,
}
impl<'g> Graphics<'g> for NotTiledResource<'g> {
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
    pub tiled: Tiled0<'g>,
    pub vram: VRamManager,
    pub unmanaged: OamUnmanaged<'g>,
    pub sprite_loader: SpriteLoader,
}
impl<'g> Graphics<'g> for Tiled0Resource<'g> {
    fn new(gba: &'g mut Gba) -> Self {
        let (tiled, vram) = gba.display.video.tiled0();
        let (unmanaged, sprite_loader) = gba.display.object.get_unmanaged();
        // let mut bg1 = tiled.background(
        //     Priority::P0,
        //     RegularBackgroundSize::Background32x32,
        //     TileFormat::FourBpp,
        // );
        Self {
            tiled: tiled,
            vram: vram,
            unmanaged: unmanaged,
            sprite_loader: sprite_loader,
        }
    }

    fn render(&mut self) {
        println!("Rendering <Tiled0Resource>");
    }
}

pub enum GraphicsResource<'g> {
    NotTiled(NotTiledResource<'g>),
    Mode0(Tiled0Resource<'g>),
}

#[derive(Debug)]
pub enum GraphicsMode {
    NotTiled,
    Mode0,
}
impl GraphicsMode {
    pub fn create<'g>(&self, gba: &'g mut Gba) -> GraphicsResource<'g> {
        println!("CREATING GRAPHICS FOR {self:?}");
        match self {
            GraphicsMode::NotTiled => GraphicsResource::NotTiled(NotTiledResource::new(gba)),
            GraphicsMode::Mode0 => GraphicsResource::Mode0(Tiled0Resource::new(gba)),
        }
    }
}


pub struct Mode0TileMap<'m> {
    pub bg1: MapLoan<'m, RegularMap>,
    pub bg2: MapLoan<'m, RegularMap>,
    pub dirty: bool,
}
impl<'m> Mode0TileMap<'m> {
    pub fn new(bg1: MapLoan<'m, RegularMap>, bg2: MapLoan<'m, RegularMap>) -> Self {
        Self {
            bg1,
            bg2,
            dirty: false,
        }
    }
}


pub enum TileMap<'m> {
    NotTiled,
    Mode0(Mode0TileMap<'m>),
}
