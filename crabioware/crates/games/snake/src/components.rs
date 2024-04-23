use agb::{
    display::{HEIGHT as GBA_HEIGHT, WIDTH as GBA_WIDTH},
    rng::RandomNumberGenerator,
};

use crabioware_core::ecs::Component;

use super::graphics::SpriteTag;

static TILE_WIDTH: i32 = 16;
static TILE_HEIGHT: i32 = 16;
pub static N_TILES_WIDE: i16 = (GBA_WIDTH / TILE_WIDTH) as i16;
pub static N_TILES_TALL: i16 = (GBA_HEIGHT / TILE_HEIGHT) as i16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionComponent {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Component for DirectionComponent {}
impl DirectionComponent {
    pub fn dx(&self) -> i16 {
        match self {
            DirectionComponent::LEFT => -1,
            DirectionComponent::RIGHT => 1,
            _ => 0,
        }
    }

    pub fn dy(&self) -> i16 {
        match self {
            DirectionComponent::UP => -1,
            DirectionComponent::DOWN => 1,
            _ => 0,
        }
    }

    pub fn random(rng: &mut RandomNumberGenerator) -> Self {
        match rng.gen().rem_euclid(4) {
            0 => Self::UP,
            1 => Self::DOWN,
            2 => Self::LEFT,
            3 => Self::RIGHT,
            _ => panic!("Not possible!"),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct TileComponent {
    pub x: i16,
    pub y: i16,
}
impl Component for TileComponent {}
impl TileComponent {
    pub fn equals(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    pub fn random(rng: &mut RandomNumberGenerator) -> Self {
        Self {
            x: rng.gen().rem_euclid(N_TILES_WIDE.into()) as i16,
            y: rng.gen().rem_euclid(N_TILES_TALL.into()) as i16,
        }
    }

    pub fn position_x(&self) -> u16 {
        (self.x as i32 * TILE_WIDTH) as u16
    }

    pub fn position_y(&self) -> u16 {
        (self.y as i32 * TILE_HEIGHT) as u16
    }

    pub fn hit_wall(&self) -> bool {
        self.x < 0 || self.x >= N_TILES_WIDE || self.y < 0 || self.y >= N_TILES_TALL
    }
}

#[derive(Clone, Copy)]
pub struct SpriteComponent {
    pub tag: SpriteTag,
    pub frame: u8,
    // FIXME: add sprite vs sprite priority once we have render queue
    // pub sprite_priority: SpritePriority,
    // FIXME: add priority back when we have a background layer
    // pub priority: Priority,
}
impl Component for SpriteComponent {}
