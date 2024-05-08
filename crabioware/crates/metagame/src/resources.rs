use agb::display::object::{OamUnmanaged, SpriteLoader};
use agb::display::tiled::{
    MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled0, VRamManager,
};
use agb::display::Priority;
use agb::{println, Gba};
use alloc::boxed::Box;

pub trait Graphics<'g> {
    fn new(gba: &'g mut Gba, tiles: Box<dyn Fn(TileMode<'g>) -> TileMap<'g>>) -> Self;
}

pub struct NotTiledResource<'g> {
    pub unmanaged: OamUnmanaged<'g>,
    pub sprite_loader: SpriteLoader,
}
impl<'g> Graphics<'g> for NotTiledResource<'g> {
    fn new(gba: &'g mut Gba, tiles: Box<dyn Fn(TileMode<'g>) -> TileMap<'g>>) -> Self {
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
    pub vram: VRamManager,
    pub unmanaged: OamUnmanaged<'g>,
    pub sprite_loader: SpriteLoader,
    pub tilemap: Mode0TileMap<'g>,
}
impl<'g> Graphics<'g> for Mode0Resource<'g> {
    fn new(gba: &'g mut Gba, tiles: Box<dyn Fn(TileMode<'g>) -> TileMap<'g>>) -> Self {
        let (tiled, vram) = gba.display.video.tiled0();
        let (unmanaged, sprite_loader) = gba.display.object.get_unmanaged();
        let tilemap = match tiles(TileMode::Mode0(tiled)) {
            TileMap::Mode0(tilemap) => tilemap,
            _ => unimplemented!("WRONG MODE")
        };
        Self {
            vram,
            unmanaged,
            sprite_loader,
            tilemap,
        }
    }
}

pub enum GraphicsMode {
    NotTiled,
    Mode0,
}
impl GraphicsMode {
    pub fn create<'g>(
        &self,
        gba: &'g mut Gba,
        tiles: Box<dyn Fn(TileMode<'g>) -> TileMap<'g>>,
    ) -> GraphicsResource<'g> {
        match self {
            GraphicsMode::NotTiled => GraphicsResource::NotTiled(NotTiledResource::new(gba, tiles)),
            GraphicsMode::Mode0 => GraphicsResource::Mode0(Mode0Resource::new(gba, tiles)),
        }
    }
}

pub enum GraphicsResource<'g> {
    NotTiled(NotTiledResource<'g>),
    Mode0(Mode0Resource<'g>),
}

pub struct Mode0TileMap<'m> {
    pub bg1: Box<MapLoan<'m, RegularMap>>,
    pub bg2: Box<MapLoan<'m, RegularMap>>,
    pub dirty: bool,
}
impl<'m> Mode0TileMap<'m> {
    pub fn new(bg1: Box<MapLoan<'m, RegularMap>>, bg2: Box<MapLoan<'m, RegularMap>>) -> Self {
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
