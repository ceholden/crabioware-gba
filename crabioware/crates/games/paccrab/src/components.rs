use agb::fixnum::Vector2D;
use crabioware_core::ecs::Component;
use crabioware_core::types::Number;

use super::graphics::SpriteTag;

#[derive(Clone, Copy, Default)]
pub struct LocationComponent {
    pub location: Vector2D<Number>,
}
impl Component for LocationComponent {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectionComponent {
    pub direction: Direction,
    pub desired: Direction,
}
impl Component for DirectionComponent {}

#[derive(Clone, Copy)]
pub struct SpeedComponent(pub Number);
impl Component for SpeedComponent {}

/// Marker for the player entity; used by collision and rendering systems to distinguish the crab from ghosts.
#[derive(Clone, Copy)]
pub struct PlayerComponent {
    pub energized_time: u32,
}
impl PlayerComponent {
    pub fn is_energized(&self) -> bool {
        self.energized_time > 0
    }
}
impl Component for PlayerComponent {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GhostKind {
    Chase,  // always targets player's current tile (Blinky)
    Ambush, // targets N tiles ahead of player's direction (Pinky)
    Shy,    // chases when far, scatters to corner when close (Clyde)
    Random, // picks a random valid direction at each intersection
    /// Alternates between chasing and scattering on a fixed timer.
    Patrol {
        chase_ticks: u32,
        shy_ticks: u32,
        timer: u32,
        chasing: bool,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct GhostComponent {
    pub kind: GhostKind,
    // Patrol and Shy ghosts will scatter
    pub scatter_tx: i32,
    pub scatter_ty: i32,
    // FIXME: scared mode to avoid energized player
    // pub scared: bool
}
impl Component for GhostComponent {}

#[derive(Clone, Copy, Debug)]
pub struct SpriteComponent {
    pub tag: SpriteTag,
    pub tag_alt: SpriteTag,
    pub alt_mode: bool,
    pub offset: Vector2D<Number>,
    pub frame: u8,
}
impl SpriteComponent {
    pub fn get_tag(&self) -> SpriteTag {
        if self.alt_mode {
            self.tag_alt
        } else {
            self.tag
        }
    }
}
impl Component for SpriteComponent {}
