use agb::display::tiled::{TileSet, TileSetting, VRamManager};
use agb::{fixnum::Vector2D, include_background_gfx};

include_background_gfx!(tile_sheet, "000000", tiles => "assets/tiles.png");

pub const TILE_SIZE: i32 = 8;

// FIXME: embed walls / path into a "Map"
pub struct Level {
    pub walls: &'static [u8],
    pub path: &'static [u8],
    pub dimensions: Vector2D<u32>,

    pub spawn: &'static (i32, i32),
    pub ghosts: &'static [(i32, i32)],
    pub berries: &'static [(i32, i32)],
    pub doors: &'static [(i32, i32)],
    pub warps: &'static [(i32, i32)],
}
impl Level {
    pub fn get_tileset(&self) -> &TileSet<'_> {
        &tile_sheet::tiles.tiles
    }

    pub fn get_tilesetting(&self, idx: usize) -> TileSetting {
        tile_sheet::tiles.tile_settings[idx]
    }

    pub fn set_background_paelttes(&self, vram: &mut VRamManager) {
        vram.set_background_palettes(tile_sheet::PALETTES);
    }

    pub fn is_walkable_tile(&self, tile_x: i32, tile_y: i32) -> bool {
        if tile_x < 0
            || tile_y < 0
            || tile_x >= self.dimensions.x as i32
            || tile_y >= self.dimensions.y as i32
        {
            return false;
        }
        self.path[(tile_y * self.dimensions.x as i32 + tile_x) as usize] != 1
    }
}

pub enum Levels {
    LEVEL_1,
}
impl Levels {
    pub fn get_level(&self) -> Level {
        match self {
            Levels::LEVEL_1 => tilemaps::level_1::get_level(),
        }
    }
}

mod tilemaps {

    use super::{tile_sheet, Level};

    pub mod tilemap {
        include!(concat!(env!("OUT_DIR"), "/tilemap.rs"));
    }

    pub mod level_1 {
        include!(concat!(env!("OUT_DIR"), "/level-1.json.rs"));
    }
}
