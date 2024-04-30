use agb::{
    display::{object::{OamIterator, OamUnmanaged, SpriteLoader}, tiled::{Tiled0, VRamManager}},
    input::ButtonController, Gba,
};
use alloc::boxed::Box;


pub trait GraphicsResource<'g> {}

pub struct NotTiledResource<'g> {
    pub unmanaged: Box<OamUnmanaged<'g>>,
    pub sprite_loader: Box<SpriteLoader>,
}
impl<'g> NotTiledResource<'g> {

    pub fn new(gba: &'g mut Gba) -> Self {
        let (unmanaged, sprite_loader) = gba.display.object.get_unmanaged();
        Self {
            unmanaged: Box::new(unmanaged),
            sprite_loader: Box::new(sprite_loader),
        }
    }
}
impl<'g> GraphicsResource<'g> for NotTiledResource<'g> {}


pub struct Tiled0Resource<'g> {
    pub tiled: Box<Tiled0<'g>>,
    pub vram: Box<VRamManager>,
    pub unmanaged: Box<OamUnmanaged<'g>>,
    pub sprite_loader: Box<SpriteLoader>,
}
impl<'g> Tiled0Resource<'g> {

    pub fn new(gba: &'g mut Gba) -> Self {
        let (tiled, vram) = gba.display.video.tiled0();
        let (unmanaged, sprite_loader) = gba.display.object.get_unmanaged();
        Self {
            tiled: Box::new(tiled),
            vram: Box::new(vram),
            unmanaged: Box::new(unmanaged),
            sprite_loader: Box::new(sprite_loader),
        }
    }
}
impl<'g> GraphicsResource<'g> for Tiled0Resource<'g> {}


pub trait RenderableGame {

    fn setup<'g>(&mut self, gba: &'g mut Gba) -> TileModeResource<'g>;
    fn render<'g, G:GraphicsResource<'g>>(&self, graphics: &'g G) -> Option<()>;
}

pub trait RunnableGame : RenderableGame {

    fn advance(&mut self, time: i32, buttons: &ButtonController);
}

pub trait Tiled0Game {


}


pub enum TileModeResource<'g> {
    NotTiled(NotTiledResource<'g>),
    TiledMode0(Tiled0Resource<'g>),
}
