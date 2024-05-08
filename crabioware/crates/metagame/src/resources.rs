use agb::display::object::{OamUnmanaged, SpriteLoader};
use agb::display::tiled::{
    MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled0, VRamManager,
};
use agb::display::Priority;
use agb::{println, Gba};
use alloc::boxed::Box;

pub trait Graphics<'g> {
    fn new(gba: &'g mut Gba) -> Self;
}

pub struct NotTiledResource<'g> {
    pub unmanaged: OamUnmanaged<'g>,
    pub sprite_loader: SpriteLoader,
}
impl<'g> Graphics<'g> for NotTiledResource<'g> {
    fn new(gba: &'g mut Gba) -> Self {
        let (unmanaged, sprite_loader) = gba.display.object.get_unmanaged();
        Self {
            unmanaged,
            sprite_loader,
        }
    }
}

pub enum TileMode<'g> {
    NotTiled,
    Mode0(Tiled0<'g>),
}

// Specification for how to create a background layer
// pub struct BackgroundSpec {
//     priority: Priority,
//     size: RegularBackgroundSize,
//     colours: TileFormat,
// }

pub struct Mode0Resource<'g> {
    pub tile0: Tiled0<'g>,
}
impl<'g> Graphics<'g> for Mode0Resource<'g> {
    fn new(gba: &'g mut Gba) -> Self {
        let (tile0, _) = gba.display.video.tiled0();
        Self {
            tile0,
        }
    }
}

pub enum GraphicsMode {
    NotTiled,
    Mode0,
}
impl GraphicsMode {
    pub fn create<'g>(&self, gba: &'g mut Gba) -> GraphicsResource<'g> {
        match self {
            GraphicsMode::NotTiled => GraphicsResource::NotTiled(NotTiledResource::new(gba)),
            GraphicsMode::Mode0 => GraphicsResource::Mode0(Mode0Resource::new(gba)),
        }
    }
}

pub enum GraphicsResource<'g> {
    NotTiled(NotTiledResource<'g>),
    Mode0(Mode0Resource<'g>),
}

pub struct Mode0TileMap<'m> {
    pub bg1: MapLoan<'m, RegularMap>,
    pub dirty: bool,
}
impl<'m> Mode0TileMap<'m> {
    pub fn new(
        bg1: MapLoan<'m, RegularMap>,
    ) -> Self {
        Self {
            bg1,
            dirty: false,
        }
    }
}

pub enum TileMap<'m> {
    NotTiled,
    Mode0(Mode0TileMap<'m>),
}
