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
impl Component for SpeedComponent {
}

#[derive(Debug, PartialEq, Eq)]
pub struct CollisionComponent {
    pub collision: Rect<Number>,
}
impl Component for CollisionComponent {}

#[derive(Clone, Copy, Debug)]
pub struct SpriteComponent {
    pub tag: SpriteTag,
    pub offset: Vector2D<Number>,
    pub frame: u8,
}
impl Component for SpriteComponent {}
