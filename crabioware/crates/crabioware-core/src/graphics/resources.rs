use agb::display::object::{OamUnmanaged, SpriteLoader};
use agb::display::tiled::{
    AffineMap, MapLoan, RegularBackgroundSize, RegularMap, TileFormat, Tiled0, Tiled1, TiledMap, VRamManager
};
use agb::display::Priority;
use agb::Gba;


pub enum GraphicsResource<'g> {
    Mode0(Tiled0<'g>, OamUnmanaged<'g>),
    Mode1(Tiled1<'g>, OamUnmanaged<'g>),
}

#[derive(Debug)]
pub enum TileMode {
    Mode0,
    Mode1,
}
impl TileMode {
    pub fn create<'g>(&self, gba: &'g mut Gba) -> (GraphicsResource<'g>, VRamManager, SpriteLoader) {
        let (unmanaged, sprite_loader) = gba.display.object.get_unmanaged();
        match self {
            TileMode::Mode0 => {
                let (tiled0, vram) = gba.display.video.tiled0();
                (GraphicsResource::Mode0(tiled0, unmanaged), vram, sprite_loader)
            }
            TileMode::Mode1 => {
                let (tiled1, vram) = gba.display.video.tiled1();
                (GraphicsResource::Mode1(tiled1, unmanaged), vram, sprite_loader)
            }
        }
    }
}

pub trait TileMapResource {
    fn clear(&mut self, vram: &mut VRamManager);
    fn set_visible(&mut self, is_visible: bool);
    fn commit(&mut self, vram: &mut VRamManager);
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

    pub fn default_32x32_4bpp<'t>(mode0: &'m Tiled0<'t>) -> Self {
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
        let bg3 = mode0.background(
            Priority::P2,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        let bg4 = mode0.background(
            Priority::P3,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        Self::new(bg1, bg2, bg3, bg4)
    }
}
impl<'m> TileMapResource for Mode0TileMap<'m> {
    fn clear(&mut self, vram: &mut VRamManager) {
        self.bg1.clear(vram);
        self.bg2.clear(vram);
        self.bg3.clear(vram);
        self.bg4.clear(vram);
    }

    fn set_visible(&mut self, visible: bool) {
        self.bg1.set_visible(visible);
        self.bg2.set_visible(visible);
        self.bg3.set_visible(visible);
        self.bg4.set_visible(visible);
    }
    fn commit(&mut self, vram: &mut VRamManager) {
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
    }
    fn set_visible(&mut self, visible: bool) {
        self.bg1.set_visible(visible);
        self.bg2.set_visible(visible);
        self.affine.set_visible(visible);
    }
    fn commit(&mut self, vram: &mut VRamManager) {
        self.bg1.commit(vram);
        self.bg2.commit(vram);
        self.affine.commit(vram);
    }
}
