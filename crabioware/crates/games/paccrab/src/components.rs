use agb::fixnum::Vector2D;
use crabioware_core::ecs::Component;
use crabioware_core::types::{Number, Rect};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectionComponent {
    pub direction: Direction,
}
impl Component for DirectionComponent {}

#[derive(Clone, Copy, Default)]
pub struct VelocityComponent {
    pub velocity: Vector2D<Number>,
}
impl Component for VelocityComponent {}

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
