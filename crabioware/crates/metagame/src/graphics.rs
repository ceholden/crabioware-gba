use agb::{
    display::{object::{OamUnmanaged, SpriteLoader}, tiled::{Tiled0, VRamManager}}, println, Gba
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
    pub tiled: Box<Tiled0<'g>>,
    pub vram: Box<VRamManager>,
    pub unmanaged: Box<OamUnmanaged<'g>>,
    pub sprite_loader: Box<SpriteLoader>,
}
impl<'g> Graphics<'g> for Tiled0Resource<'g> {

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

pub enum TileModeResource<'g> {
    NotTiled(NotTiledResource<'g>),
    TiledMode0(Tiled0Resource<'g>),
}


pub trait Game<'g, G: Graphics<'g>> {
    fn render(&mut self, graphics: G);
}


pub struct NotTiledGame {}
impl<'g, G> Game<'g, G> for NotTiledGame
where
    G: Graphics<'g>
{
    fn render(&mut self, graphics: G) {
        println!("Rendering with no graphics");
        graphics.render();
    }
}


pub struct GameRunner<'g, G: Graphics<'g>> {
    game: dyn Game<'g, G>,
}
impl<'g, G> GameRunner<'g, G>
where
    G: Graphics<'g>
{
    fn run(&mut self, graphics: G) {
        self.game.render(graphics);
    }
}
