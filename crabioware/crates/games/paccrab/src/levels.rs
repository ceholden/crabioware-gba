use agb::display::tiled::{TileSet, TileSetting, VRamManager};
use agb::{fixnum::Vector2D, include_background_gfx};
use crabioware_core::types::Number;

include_background_gfx!(tile_sheet, "000000", tiles => "assets/tiles.png");

// FIXME: embed walls / path into a "Map"
pub struct Level {
    pub walls: &'static [u8],
    pub path: &'static [u8],
    pub dots: &'static [u8],
    pub total_dots: usize,
    pub dimensions: Vector2D<u32>,
    pub tile_size: u8,

    // Tile coordinates
    pub spawn: &'static (u8, u8),
    pub ghosts: &'static [(u8, u8)],
    pub berries: &'static [(u8, u8)],
    pub doors: &'static [(u8, u8)],
    pub warps: &'static [(u8, u8)],
}
impl Level {
    pub fn tile_of(&self, pos: Number) -> i32 {
        (pos / Number::new(self.tile_size as i32)).floor()
    }

    pub fn tile_center(&self, tile_idx: i32) -> Number {
        let ts = self.tile_size as i32;
        Number::new(tile_idx * ts + ts / 2)
    }

    pub fn get_tileset(&self) -> &TileSet<'_> {
        &tile_sheet::tiles.tiles
    }

    pub fn get_tilesetting(&self, idx: usize) -> TileSetting {
        tile_sheet::tiles.tile_settings[idx]
    }

    pub fn set_background_palettes(&self, vram: &mut VRamManager) {
        vram.set_background_palettes(tile_sheet::PALETTES);
    }

    /// Are we inside the level?
    fn in_level(&self, tile_x: i32, tile_y: i32) -> bool {
        tile_x >= 0
            && tile_y >= 0
            && tile_x < self.dimensions.x as i32
            && tile_y < self.dimensions.y as i32
    }

    fn tile_index(&self, tile_x: i32, tile_y: i32) -> Option<usize> {
        self.in_level(tile_x, tile_y)
            .then(|| ((tile_y * self.dimensions.x as i32 + tile_x) as usize))
    }

    pub fn is_edible_pellet_tile(&self, tile_x: i32, tile_y: i32, eaten: &[bool]) -> Option<usize> {
        self.tile_index(tile_x, tile_y)
            .map(|i| {
                if self.dots[i] == tilemaps::tileset::PELLET_TILE_ID as u8 && !eaten[i] {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap_or(None)
    }

    pub fn is_edible_dot_tile(&self, tile_x: i32, tile_y: i32, eaten: &[bool]) -> Option<usize> {
        self.tile_index(tile_x, tile_y)
            .map(|i| {
                if self.dots[i] == tilemaps::tileset::DOT_TILE_ID as u8 && !eaten[i] {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap_or(None)
    }

    /// Is this the door to the ghost house?
    pub fn is_door_tile(&self, tile_x: u8, tile_y: u8) -> bool {
        self.doors
            .iter()
            .copied()
            .any(|tile| tile == (tile_x, tile_y))
    }

    /// Are we inside the ghost house?
    fn is_ghost_house_tile(&self, tile_x: i32, tile_y: i32) -> bool {
        self.tile_index(tile_x, tile_y)
            .map(|i| self.path[i] == tilemaps::tileset::GHOST_TILE_ID as u8)
            .unwrap_or(false)
    }

    /// Can the player walk on this tile?
    pub fn is_walkable_tile(&self, tile_x: i32, tile_y: i32) -> bool {
        self.tile_index(tile_x, tile_y)
            .map(|i| self.path[i] == tilemaps::tileset::PATH_TILE_ID as u8)
            .unwrap_or(false)
    }

    /// Can ghosts walk on this tile (they're allowed in the house)
    pub fn is_ghost_walkable_tile(&self, tile_x: i32, tile_y: i32) -> bool {
        match self.tile_index(tile_x, tile_y) {
            None => false,
            Some(i) => {
                let t = self.path[i];
                t == tilemaps::tileset::PATH_TILE_ID as u8
                    || t == tilemaps::tileset::GHOST_TILE_ID as u8
            }
        }
    }

    // Warp to/from in pixel units
    pub fn warp_destination(&self, tile_x: i32, tile_y: i32) -> Option<(Number, Number)> {
        self.warps
            .iter()
            .position(|&(wx, wy)| wx as i32 == tile_x && wy as i32 == tile_y)
            .map(|i| {
                let (tx, ty) = self.warps[(i + 1) % self.warps.len()];
                (self.tile_center(tx as i32), self.tile_center(ty as i32))
            })
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

pub mod tilemaps {

    pub mod tileset {
        include!(concat!(env!("OUT_DIR"), "/tileset.rs"));
    }

    pub mod level_1 {
        include!(concat!(env!("OUT_DIR"), "/level-1.json.rs"));
    }
}
