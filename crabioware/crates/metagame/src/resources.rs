use agb::{println, Gba};
use agb::display::Priority;
use agb::display::object::{OamUnmanaged, SpriteLoader};
use agb::display::tiled::{MapLoan, RegularMap, TileFormat, RegularBackgroundSize, Tiled0, VRamManager};

use alloc::boxed::Box;


pub trait GraphicsResource<'g> {
    fn new(gba: &'g mut Gba) -> Self;
}

pub struct NotTiledResource<'g> {
    pub unmanaged: Box<OamUnmanaged<'g>>,
    pub sprite_loader: Box<SpriteLoader>,
}
impl<'g> GraphicsResource<'g> for NotTiledResource<'g> {

    fn new(gba: &'g mut Gba) -> Self {
        let (unmanaged, sprite_loader) = gba.display.object.get_unmanaged();
        Self {
            unmanaged: Box::new(unmanaged),
            sprite_loader: Box::new(sprite_loader),
        }
    }
}


// Specification for how to create a background layer
pub struct BackgroundSpec {
    priority: Priority,
    size: RegularBackgroundSize,
    colours: TileFormat,
}

pub struct Tiled0Resource<'g> {
    pub unmanaged: Box<OamUnmanaged<'g>>,
    pub sprite_loader: Box<SpriteLoader>,
    pub vram: Box<VRamManager>,
    pub bg1: Box<MapLoan<'g, RegularMap>>,
    pub bg2: Box<MapLoan<'g, RegularMap>>,
}
impl<'g> GraphicsResource<'g> for Tiled0Resource<'g> {

    fn new(gba: &'g mut Gba) -> Self {
        println!("HELLO GRAPHICS");
        let (tiled, vram) = gba.display.video.tiled0();
        let (unmanaged, sprite_loader) = gba.display.object.get_unmanaged();
        let bg1 = tiled.background(Priority::P0, RegularBackgroundSize::Background32x32, TileFormat::FourBpp);
        let bg2 = tiled.background(Priority::P1, RegularBackgroundSize::Background32x32, TileFormat::FourBpp);
        Self {
            vram: Box::new(vram),
            unmanaged: Box::new(unmanaged),
            sprite_loader: Box::new(sprite_loader),
            bg1: Box::new(bg1),
            bg2: Box::new(bg2),
        }
    }
}


pub enum TileModeResource<'g> {
    NotTiled(NotTiledResource<'g>),
    TiledMode0(Tiled0Resource<'g>),
}
