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
    pub tiled: Tiled0<'g>,
}

pub enum TiledMode {
    NotTiled,
    Mode0,
}
impl TiledMode {
    pub fn create<'g>(&self, gba: &'g mut Gba) -> (TiledResource<'g>, VRamManager) {
        match self {
            TiledMode::NotTiled => {
                let (_, vram) = gba.display.video.tiled0();
                (TiledResource::NotTiled, vram)
            }
            TiledMode::Mode0 => {
                let (tiled0, vram) = gba.display.video.tiled0();
                (TiledResource::Mode0(Mode0Resource { tiled: tiled0 }), vram)
            }
        }
    }
}

pub enum TiledResource<'g> {
    NotTiled,
    Mode0(Mode0Resource<'g>),
}

pub struct Mode0TileMap<'m> {
    pub bg1: MapLoan<'m, RegularMap>,
    pub dirty: bool,
}
impl<'m> Mode0TileMap<'m> {
    pub fn new(bg1: MapLoan<'m, RegularMap>) -> Self {
        Self { bg1, dirty: false }
    }
}

pub enum TileModeMap<'m> {
    NotTiled,
    Mode0(Mode0TileMap<'m>),
}
