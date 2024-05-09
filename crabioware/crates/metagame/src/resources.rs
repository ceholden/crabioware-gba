use agb::display::object::{OamUnmanaged, SpriteLoader};
use agb::display::tiled::{
    AffineMap, MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled0, Tiled1, TiledMap,
    VRamManager,
};
use agb::{println, Gba};

pub trait Graphics<'g> {
    fn new(gba: &'g mut Gba) -> Self;
}

// Specification for how to create a background layer
// pub struct BackgroundSpec {
//     priority: Priority,
//     size: RegularBackgroundSize,
//     colours: TileFormat,
// }

pub enum TiledModeResource<'g> {
    Mode0(Tiled0<'g>),
    Mode1(Tiled1<'g>),
}

#[derive(Debug)]
pub enum TiledMode {
    Mode0,
    Mode1,
}
impl TiledMode {
    pub fn create<'g>(&self, gba: &'g mut Gba) -> (TiledModeResource<'g>, VRamManager) {
        println!("CREATING NEW TILE RESOURCE FOR {self:?}");
        match self {
            TiledMode::Mode0 => {
                let (tiled0, vram) = gba.display.video.tiled0();
                (TiledModeResource::Mode0(tiled0), vram)
            }
            TiledMode::Mode1 => {
                let (tiled1, vram) = gba.display.video.tiled1();
                (TiledModeResource::Mode1(tiled1), vram)
            }
        }
    }
}

pub trait TileMapResource {
    fn clear(&mut self, vram: &mut VRamManager);
}

pub struct Mode0TileMap<'m> {
    pub bg1: MapLoan<'m, RegularMap>,
    pub bg2: MapLoan<'m, RegularMap>,
    pub bg3: MapLoan<'m, RegularMap>,
    pub bg4: MapLoan<'m, RegularMap>,
    pub dirty: bool,
}
impl<'m> Mode0TileMap<'m> {
    pub fn new(
        bg1: MapLoan<'m, RegularMap>,
        bg2: MapLoan<'m, RegularMap>,
        bg3: MapLoan<'m, RegularMap>,
        bg4: MapLoan<'m, RegularMap>,
    ) -> Self {
        Self {
            bg1,
            bg2,
            bg3,
            bg4,
            dirty: false,
        }
    }
}
impl<'m> TileMapResource for Mode0TileMap<'m> {
    fn clear(&mut self, vram: &mut VRamManager) {
        self.bg1.clear(vram);
        self.bg2.clear(vram);
        self.bg3.clear(vram);
        self.bg4.clear(vram);
        self.bg1.commit(vram);
        self.bg2.commit(vram);
        self.bg3.commit(vram);
        self.bg4.commit(vram);
    }
}

pub struct Mode1TileMap<'m> {
    pub bg1: MapLoan<'m, RegularMap>,
    pub bg2: MapLoan<'m, RegularMap>,
    pub affine: MapLoan<'m, AffineMap>,
    pub dirty: bool,
}
impl<'m> Mode1TileMap<'m> {
    pub fn new(
        bg1: MapLoan<'m, RegularMap>,
        bg2: MapLoan<'m, RegularMap>,
        affine: MapLoan<'m, AffineMap>,
    ) -> Self {
        Self {
            bg1,
            bg2,
            affine,
            dirty: false,
        }
    }
}
impl<'m> TileMapResource for Mode1TileMap<'m> {
    fn clear(&mut self, vram: &mut VRamManager) {
        self.bg1.clear(vram);
        self.bg2.clear(vram);
        self.affine.clear(vram);
        self.bg1.commit(vram);
        self.bg2.commit(vram);
        self.affine.commit(vram);
    }
}
