// Pong
use agb::{
    display::affine::AffineMatrix,
    display::object::{
        AffineMatrixInstance, AffineMode, OamIterator, ObjectUnmanaged, SpriteLoader,
    },
    display::{HEIGHT as GBA_HEIGHT, WIDTH as GBA_WIDTH},
    fixnum::{num, Vector2D},
    input::{ButtonController, Tri},
    rng::RandomNumberGenerator,
};
use alloc::vec;
use alloc::vec::Vec;

use crate::games::Game;
use crate::physics::intersect::Intersects;
use crate::types::{Number, Rect, RectMath};
use crate::{
    ecs::{EntityId, IsNotEntity, World},
    games::{GameState, Games},
};
use crate::types::VecMath;

use super::components::{CollisionComponent, LocationComponent, SpriteComponent, VelocityComponent};
use super::graphics::SpriteTag;

#[derive(Default)]
struct GameStateResource {
    score: u8,
    max_score: u8,
}

struct Ball {}
impl Ball {
    fn new(
        rng: &mut RandomNumberGenerator,
    ) -> (
        Option<SpriteComponent>,
        Option<LocationComponent>,
        Option<VelocityComponent>,
        Option<CollisionComponent>,
    ) {
        let sprite = SpriteComponent {
            tag: SpriteTag::Ball,
            offset: Default::default(),
            frame: 0,
        };
        let pos_x = (rng.gen() % GBA_WIDTH).abs();
        let pos_y = (rng.gen() % GBA_HEIGHT).abs();
        let velocity = Vector2D {
            x: (num!(-1.) * (rng.gen().rem_euclid(10) + 1)).into(),
            y: num!(0.), // y: (rng.gen().rem_euclid(10) + 1).into(),
        } / num!(2.0);
        (
            Some(sprite),
            Some(LocationComponent {
                position: Vector2D {
                    x: pos_x.into(),
                    y: pos_y.into(),
                },
                angle: num!(0.),
            }),
            Some(VelocityComponent {
                velocity,
                acceleration: Vector2D::default(),
                rotation: num!(0.01),
            }),
            Some(CollisionComponent {
                collision: Rect::new(
                    Vector2D {
                        x: num!(4.0),
                        y: num!(4.0),
                    },
                    Vector2D {
                        x: num!(8.),
                        y: num!(8.),
                    },
                ),
                bounce: num!(0.9),
                inv_mass: num!(1.),
            }),
        )
    }
}

enum Side {
    LEFT,
    RIGHT,
}

struct Paddle {}
impl Paddle {
    fn new(
        side: Side,
        y_velocity: Number,
    ) -> (
        Option<SpriteComponent>,
        Option<LocationComponent>,
        Option<VelocityComponent>,
        Option<CollisionComponent>,
    ) {
        let x_start: Number = match side {
            Side::LEFT => num!(0.1) * GBA_WIDTH,
            Side::RIGHT => num!(0.8) * GBA_WIDTH,
        };
        let y_start: Number = num!(0.25) * GBA_HEIGHT;
        (
            // paddle mid
            Some(SpriteComponent {
                tag: SpriteTag::Paddle,
                offset: Vector2D::default(),
                frame: 0,
            }),
            Some(LocationComponent {
                position: Vector2D {
                    x: x_start,
                    y: y_start,
                },
                angle: num!(0.),
            }),
            Some(VelocityComponent {
                velocity: Vector2D {
                    x: num!(0.),
                    y: y_velocity,
                },
                acceleration: Vector2D {
                    x: num!(0.),
                    y: num!(0.1),
                },
                rotation: num!(0.0),
            }),
            Some(CollisionComponent {
                collision: Rect::new(
                    Vector2D {
                        x: num!(3.),
                        y: num!(0.),
                    },
                    Vector2D {
                        x: num!(10.),
                        y: num!(16.),
                    },
                ),
                bounce: num!(1.0),
                inv_mass: num!(1e-3),
            }),
        )
    }
}

// TODO: add a "render cache" that helps us disconnect object setup and render
//       e.g., so we can sort on z-axis or priority
// TODO: SpriteComponent does NOT need the SpriteVram!
//       The SpriteLoader takes care of
// TODO: alias our own Vector2D so we can add impl like,
//       * Mul for Rect
//       * MulAssign for Rect
//       * Display for all
pub struct PongGame {
    world: World,
    player: EntityId,
    opponent: EntityId,
    game_state: GameStateResource,
}
impl PongGame {
    pub fn new(loader: &mut SpriteLoader, _: &mut RandomNumberGenerator) -> Self {
        let mut world = World::new();
        world.register_component::<SpriteComponent>();
        world.register_component::<LocationComponent>();
        world.register_component::<VelocityComponent>();
        world.register_component::<CollisionComponent>();

        let mut rng = RandomNumberGenerator::new();
        let mut entities = vec![
            // player
            Paddle::new(Side::LEFT, num!(0.)),
            // opponent
            Paddle::new(Side::RIGHT, num!(1.)),
            // balls
            Ball::new(&mut rng),
            Ball::new(&mut rng),
        ];

        let entities: Vec<EntityId> = entities
            .drain(..)
            .map(|(spr, loc, vel, col)| {
                // Preload sprite
                match spr {
                    Some(ref spr_) => {
                        loader.get_vram_sprite(spr_.tag.tag().sprite(spr_.frame.into()));
                    }
                    None => {}
                }
                // Create entity
                world
                    .create()
                    .maybe_with(spr)
                    .maybe_with(loc)
                    .maybe_with(vel)
                    .maybe_with(col)
                    .build()
            })
            .collect();

        Self {
            world,
            player: entities[0],
            opponent: entities[1],
            game_state: GameStateResource::default(),
        }
    }

    fn system_player(&self, time: i32, buttons: &ButtonController) {
        let (mut location, mut velocity) = *self
            .world
            .entry::<(&mut LocationComponent, &mut VelocityComponent)>(self.player);

        // FIXME: this jank
        match buttons.y_tri() {
            Tri::Positive => {
                let new_velocity = velocity.velocity.y + velocity.acceleration.y * time;
                velocity.velocity.y = new_velocity;
            }
            Tri::Negative => {
                let new_velocity = velocity.velocity.y - velocity.acceleration.y * time;
                velocity.velocity.y = new_velocity;
            }
            _ => {
                let new_velocity = if velocity.velocity.y == num!(0.) {
                    velocity.velocity.y
                } else if velocity.velocity.y > num!(0.) {
                    velocity.velocity.y - velocity.acceleration.y * time
                } else {
                    velocity.velocity.y + velocity.acceleration.y * time
                };
                velocity.velocity.y = new_velocity;
            }
        };
        location.position.y += velocity.velocity.y * time;
    }

    fn system_movement(&self, time: i32) {
        let filter = IsNotEntity::new(self.player);
        let iter = self
            .world
            .query::<(&mut LocationComponent, &VelocityComponent), IsNotEntity>(&filter);
        for (mut location, velocity) in iter {
            location.position += velocity.velocity * time;
            location.angle += velocity.rotation * time;
        }
    }

    fn system_collision(&self, _: i32) {
        // We're checking intersection based on potential movement, not
        // trajectory. If entities are moving really fast we might
        // have them phase through each other, but otherwise this is
        // a quicker way of checking collisions than continuous collision detection

        let iter = self.world.combinations::<(
            &mut LocationComponent,
            &mut VelocityComponent,
            &CollisionComponent,
        )>();

        for (
            (mut location_a, mut velocity_a, collision_a),
            (mut location_b, mut velocity_b, collision_b),
        ) in iter
        {
            // FIXME: it'd be easier to use as a center x/y + half width/height
            let collision_box_a = collision_a.collision.translate(location_a.position);
            let collision_box_b = collision_b.collision.translate(location_b.position);

            if let Some(collided) = collision_box_a.separation(&collision_box_b) {
                // FIXMEE -- this isn't quite right...
                // Unstick
                let inv_masses = collision_a.inv_mass + collision_b.inv_mass;
                let delta_a =
                    collided.normal * collided.distance / inv_masses * collision_a.inv_mass;
                let delta_b =
                    collided.normal * collided.distance / inv_masses * collision_b.inv_mass;
                location_a.position -= delta_a;
                location_b.position += delta_b;

                // Resolve collision
                let elasticity = collision_a.bounce.min(collision_b.bounce);
                let relative_velocity = velocity_a.velocity - velocity_b.velocity;
                let relative_velocity_norm = relative_velocity.dot(collided.normal);

                let impulse = -(num!(1.) + elasticity) * relative_velocity_norm / inv_masses;

                velocity_a.velocity += collided.normal * impulse * collision_a.inv_mass;
                velocity_b.velocity -= collided.normal * impulse * collision_b.inv_mass;
            }
        }
    }

    fn system_bounds(&self, _: i32) {
        let iter = self.world.components::<(
            &LocationComponent,
            &mut VelocityComponent,
            &CollisionComponent,
        )>();

        for (location, mut velocity, collision) in iter {
            if (location.position.y < num!(0.0) && velocity.velocity.y < num!(0.))
                || (location.position.y + collision.collision.0.size.y > GBA_HEIGHT.into()
                    && velocity.velocity.y > num!(0.))
            {
                velocity.velocity.y *= num!(-1.0)
            }
            if (location.position.x < num!(0.0) && velocity.velocity.x < num!(0.))
                || (location.position.x + collision.collision.0.size.x > GBA_WIDTH.into()
                    && velocity.velocity.x > num!(0.))
            {
                velocity.velocity.x *= num!(-1.0)
            }
        }
    }
}

impl Game for PongGame {
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.system_player(time, &buttons);
        self.system_movement(time);
        self.system_collision(time);
        self.system_bounds(time);
        GameState::Running(Games::Pong)
    }

    // TODO: split into 2 steps - create objects & then render
    fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) -> Option<()> {
        for (location, sprite) in self
            .world
            .components::<(&LocationComponent, &mut SpriteComponent)>()
        {
            let affine = AffineMatrixInstance::new(
                AffineMatrix::from_rotation(location.angle).to_object_wrapping(),
            );
            let position = (location.position + sprite.offset).floor();
            let mut object = ObjectUnmanaged::new(
                loader.get_vram_sprite(sprite.tag.tag().sprite(sprite.frame.into())),
            );
            object
                .set_position(position)
                .set_affine_matrix(affine)
                .show();
            object.show_affine(AffineMode::Affine);
            oam.next()?.set(&object);
        }
        Some(())
    }
}
