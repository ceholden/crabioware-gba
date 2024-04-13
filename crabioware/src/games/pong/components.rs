use agb::{
    display::{HEIGHT as GBA_HEIGHT, WIDTH as GBA_WIDTH},
    fixnum::num,
};

use crate::ecs::Component;
use crate::types::{Number, Rect, Vector2D};

use super::graphics::SpriteTag;

#[derive(Debug)]
pub struct SpriteComponent {
    pub tag: SpriteTag,
    pub offset: Vector2D<Number>,
    // FIXME: add frame number, or use a separate "AnimationComponent"?
    pub frame: u8,
}
impl Component for SpriteComponent {}

// For now, location is relative to the GBA screen (!!)
#[derive(Debug, Default)]
pub struct LocationComponent {
    pub position: Vector2D<Number>,
    pub angle: Number,
}
impl LocationComponent {
    pub fn centered() -> LocationComponent {
        Self {
            position: Vector2D {
                x: Number::new(GBA_WIDTH / 2),
                y: Number::new(GBA_HEIGHT / 2),
            },
            angle: num!(0.),
        }
    }

}
impl Component for LocationComponent {}

#[derive(Debug)]
pub struct MaxSpeed {
    pos: Number,
    neg: Number,
}
impl MaxSpeed {
    pub fn symmetric(pos: Number) -> Self {
        Self { pos, neg: -pos }
    }
}
impl Default for MaxSpeed {
    fn default() -> Self {
        Self {
            pos: Number::new(2),
            neg: Number::new(-2),
        }
    }
}

#[derive(Debug, Default)]
pub struct VelocityComponent {
    pub velocity: Vector2D<Number>,
    // TODO: add acceleration and some notion of possible jerk
    // (e.g., apply some force per button push or AI update that changes accel)
    pub acceleration: Vector2D<Number>,
    pub rotation: Number,
}
impl Component for VelocityComponent {}
impl VelocityComponent {
    pub fn clamp_velocity(&mut self, max_speed: &MaxSpeed) {
        self.velocity.x = self.velocity.x.clamp(max_speed.neg, max_speed.pos);
        self.velocity.y = self.velocity.y.clamp(max_speed.neg, max_speed.pos);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CollisionComponent {
    pub collision: Rect<Number>,
    pub bounce: Number,
    pub inv_mass: Number,
}
impl Component for CollisionComponent {}

#[derive(Debug, PartialEq, Eq)]
pub struct PhysicsComponent {
    pub position: Vector2D<Number>,
    pub angle: Number,
    pub velocity: Vector2D<Number>,
    pub rotation: Number,
    pub inv_mass: Number, // 1/mass is used for calculations, so do it once
    pub bounce: Number,
}
impl Component for PhysicsComponent {}
