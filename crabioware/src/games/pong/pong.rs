// Pong
use agb::{
    display::affine::AffineMatrix,
    display::object::{
        AffineMatrixInstance, AffineMode, OamIterator, ObjectUnmanaged, SpriteLoader,
    },
    display::{HEIGHT as GBA_HEIGHT, WIDTH as GBA_WIDTH},
    fixnum::num,
    input::{ButtonController, Tri},
    println,
    rng::RandomNumberGenerator,
};
use alloc::vec;
use alloc::vec::Vec;

use crate::games::{Game, GameDifficulty};
use crate::physics::intersect::Intersects;
use crate::types::VecMath;
use crate::types::{Number, Rect, RectMath, Vector2D};
use crate::{
    ecs::{EntityId, World},
    games::{GameState, Games},
};

use super::components::{
    CollisionComponent, LocationComponent, MaxSpeed, SpriteComponent, VelocityComponent,
};
use super::graphics::SpriteTag;

// FIXME: keep score
#[allow(unused)]
struct GameStateResource {
    player_score: u8,
    opponent_score: u8,
    max_score: u8,
    max_speed: MaxSpeed,
}
impl GameStateResource {
    fn new(difficulty: &GameDifficulty) -> Self {
        let max_speed = match difficulty {
            GameDifficulty::EASY => MaxSpeed::symmetric(num!(1.)),
            GameDifficulty::MEDIUM => MaxSpeed::symmetric(num!(1.5)),
            GameDifficulty::HARD => MaxSpeed::symmetric(num!(2.)),
        };
        println!("MAX SPEED: {:?}", max_speed);
        GameStateResource {
            player_score: 0,
            opponent_score: 0,
            max_score: 10,
            max_speed,
        }
    }

    fn game_state(&self) -> GameState {
        if self.player_score >= self.max_score {
            GameState::Win(Games::Pong)
        } else if self.opponent_score >= self.max_score {
            GameState::GameOver
        } else {
            GameState::Running(Games::Pong)
        }
    }
}
impl Default for GameStateResource {
    fn default() -> Self {
        Self {
            player_score: 0,
            opponent_score: 0,
            max_score: 10,
            max_speed: MaxSpeed::default(),
        }
    }
}

#[derive(Default)]
struct OpponentResource {
    target: Option<EntityId>,
}

struct Ball {
    sprite: SpriteComponent,
    location: LocationComponent,
    velocity: VelocityComponent,
    collision: CollisionComponent,
}
impl Ball {
    fn new(rng: &mut RandomNumberGenerator) -> Self {
        let sprite = SpriteComponent {
            tag: SpriteTag::Ball,
            offset: Default::default(),
            frame: 0,
        };
        let pos_x = (GBA_WIDTH / 2 + (rng.gen() % (GBA_WIDTH / 10))).abs();
        let pos_y = (rng.gen() % GBA_HEIGHT).abs();
        let velocity = Vector2D::<Number>::new(
            (-(rng.gen().rem_euclid(10) + 5)).into(),
            (rng.gen().rem_euclid(10) + 5).into(),
        ) / num!(10.);
        Self {
            sprite,
            location: LocationComponent {
                position: Vector2D {
                    x: pos_x.into(),
                    y: pos_y.into(),
                },
                angle: num!(0.),
            },
            velocity: VelocityComponent {
                velocity,
                acceleration: Vector2D::default(),
                rotation: num!(0.01),
            },
            collision: CollisionComponent {
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
            },
        }
    }

    pub fn create(self, world: &mut World) -> EntityId {
        world
            .create()
            .with(self.sprite)
            .with(self.location)
            .with(self.velocity)
            .with(self.collision)
            .build()
    }
}

enum Side {
    LEFT,
    RIGHT,
}

struct Paddle {
    sprite: SpriteComponent,
    location: LocationComponent,
    velocity: VelocityComponent,
    collision: CollisionComponent,
}
impl Paddle {
    fn new(side: Side, y_velocity: Number) -> Self {
        let x_start: Number = match side {
            Side::LEFT => num!(0.1) * GBA_WIDTH,
            Side::RIGHT => num!(0.8) * GBA_WIDTH,
        };
        let y_start: Number = num!(0.25) * GBA_HEIGHT;
        Self {
            // paddle mid
            sprite: SpriteComponent {
                tag: SpriteTag::Paddle,
                offset: Vector2D::default(),
                frame: 0,
            },
            location: LocationComponent {
                position: Vector2D {
                    x: x_start,
                    y: y_start,
                },
                angle: num!(0.),
            },
            velocity: VelocityComponent {
                velocity: Vector2D {
                    x: num!(0.),
                    y: y_velocity,
                },
                acceleration: Vector2D {
                    x: num!(0.),
                    y: num!(0.1),
                },
                rotation: num!(0.0),
            },
            collision: CollisionComponent {
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
            },
        }
    }

    pub fn create(self, world: &mut World) -> EntityId {
        world
            .create()
            .with(self.sprite)
            .with(self.location)
            .with(self.velocity)
            .with(self.collision)
            .build()
    }
}

// TODO: add a "render cache" that helps us disconnect object setup and render
//       e.g., so we can sort on z-axis or priority
// TODO: alias our own Vector2D so we can add impl like,
//       * Mul for Rect
//       * MulAssign for Rect
//       * Display for all
pub struct PongGame {
    world: World,
    game_rng: RandomNumberGenerator,
    player: EntityId,
    opponent: EntityId,
    balls: Vec<EntityId>,
    opponent_state: OpponentResource,
    game_state: GameStateResource,
}
impl PongGame {
    pub fn new(
        difficulty: &GameDifficulty,
        _: &mut SpriteLoader,
        rng: &mut RandomNumberGenerator,
    ) -> Self {
        let mut game_rng = RandomNumberGenerator::new_with_seed([
            rng.gen().abs() as u32,
            rng.gen().abs() as u32,
            rng.gen().abs() as u32,
            rng.gen().abs() as u32,
        ]);

        let mut world = World::new();
        world.register_component::<SpriteComponent>();
        world.register_component::<LocationComponent>();
        world.register_component::<VelocityComponent>();
        world.register_component::<CollisionComponent>();

        let player = Paddle::new(Side::LEFT, num!(0.)).create(&mut world);
        let opponent = Paddle::new(Side::RIGHT, num!(1.)).create(&mut world);
        let balls = vec![
            Ball::new(&mut game_rng).create(&mut world),
            Ball::new(&mut game_rng).create(&mut world),
        ];

        Self {
            world,
            game_rng,
            player,
            opponent,
            balls,
            opponent_state: OpponentResource::default(),
            game_state: GameStateResource::new(difficulty),
        }
    }

    fn system_player(&self, time: i32, buttons: &ButtonController) {
        let (mut location, mut velocity) = *self
            .world
            .entry::<(&mut LocationComponent, &mut VelocityComponent)>(self.player);

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
        location.clamp_world();
    }

    #[allow(unused_variables)]
    #[allow(unused_mut)]
    fn system_cpu_paddle(&self, entity: EntityId, time: i32) {
        // FIXME: opponent logic ~ GameDifficulty
        let (mut paddle_location, mut paddle_velocity, paddle_collision) =
            *self.world.entry::<(
                &mut LocationComponent,
                &mut VelocityComponent,
                &CollisionComponent,
            )>(entity);

        let balls = self
            .world
            .entries::<(&LocationComponent, &VelocityComponent, &CollisionComponent)>(&self.balls);

        // 1. Detect incoming ball(s) moving towards paddle
        // FIXME: track a "target" and only reset if scored or rebounded
        //        to avoid changing target and reduce compute
        let mut incoming = Vec::<(Number, Vector2D<Number>, Vector2D<Number>)>::new();
        for (location, velocity, collision) in balls {
            let delta = paddle_location.position.x - location.position.x;
            if delta * velocity.velocity.x > num!(0.) {
                incoming.push((
                    delta,
                    location.position + collision.collision.size / num!(2.),
                    velocity.velocity,
                ))
            }
        }

        // Define target Y position -- we either want to reset to middle or towards ball
        // 2. If incoming balls,
        let target_y = if incoming.len() > 0 {
            //  a. True => prioritize closest and move towards
            incoming.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));
            let (_, position, _) = incoming[0];
            position.y
        } else {
            // b. False => move towards middle point
            Number::new(GBA_HEIGHT / 2) - paddle_collision.collision.size.y / num!(2.)
        };

        let projected_y = paddle_location.position.y + paddle_velocity.velocity.y * time;
        let new_velocity_y = if projected_y > target_y {
            paddle_velocity.velocity.y - paddle_velocity.acceleration.y * time
        } else if projected_y < target_y {
            paddle_velocity.velocity.y + paddle_velocity.acceleration.y * time
        } else {
            paddle_velocity.velocity.y
        };
        paddle_velocity.velocity.y = new_velocity_y;

        paddle_velocity.clamp_velocity(&self.game_state.max_speed);
        paddle_location.position.y += paddle_velocity.velocity.y * time;

        paddle_location.clamp_world();
    }

    fn system_balls(&self, time: i32) {
        let iter = self
            .world
            .entries::<(&mut LocationComponent, &VelocityComponent)>(&self.balls);
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
            let collision_box_a = collision_a.collision.translate(location_a.position);
            let collision_box_b = collision_b.collision.translate(location_b.position);

            if let Some(collided) = collision_box_a.separation(&collision_box_b) {
                // Unstick
                let inv_masses = collision_a.inv_mass + collision_b.inv_mass;
                let delta_a = collided.separation * collision_a.inv_mass / inv_masses;
                let delta_b = collided.separation * collision_b.inv_mass / inv_masses;
                location_a.position -= delta_a;
                location_b.position += delta_b;

                // Resolve collision
                let elasticity = collision_a.bounce.min(collision_b.bounce);
                let relative_velocity = velocity_a.velocity - velocity_b.velocity;
                let relative_velocity_norm = relative_velocity.dot(collided.normal);

                // Don't update if already moving away
                if relative_velocity_norm > num!(0.) {
                    // FIXME: missing representation of tangent impulse + friction info
                    let impulse = -(num!(1.) + elasticity) * relative_velocity_norm / inv_masses;

                    velocity_a.velocity += collided.normal * impulse * collision_a.inv_mass;
                    velocity_b.velocity -= collided.normal * impulse * collision_b.inv_mass;

                    velocity_a.clamp_velocity(&self.game_state.max_speed);
                    velocity_b.clamp_velocity(&self.game_state.max_speed);
                }
            }
        }
    }

    fn system_bounds(&mut self, _: i32) {
        let iter = self.world.entries::<(
            EntityId,
            &LocationComponent,
            &mut VelocityComponent,
            &CollisionComponent,
        )>(&self.balls);

        let zero: Number = num!(0.);
        let mut scored = Vec::<EntityId>::new();
        for (entity, location, mut velocity, collision) in iter {
            // Bounce off top/bottom
            if (location.position.y < zero && velocity.velocity.y < zero)
                || (location.position.y + collision.collision.size.y > GBA_HEIGHT.into()
                    && velocity.velocity.y > zero)
            {
                velocity.velocity.y *= num!(-1.0)
            }

            if location.position.x < zero && velocity.velocity.x < zero {
                self.game_state.opponent_score += 1;
                scored.push(entity);
            }
            if location.position.x + collision.collision.size.x > GBA_WIDTH.into()
                && velocity.velocity.x > num!(0.)
            {
                self.game_state.player_score += 1;
                scored.push(entity);
            }
        }

        self.system_ball_scored(scored);
    }

    fn system_ball_scored(&mut self, balls: Vec<EntityId>) {
        self.balls.retain(|b| !balls.contains(b));
        for ball in balls {
            self.world.destroy(ball);
            let new_ball = Ball::new(&mut self.game_rng).create(&mut self.world);
            self.balls.push(new_ball);
        }
    }

    fn renderer_digits(
        &self,
        loader: &mut SpriteLoader,
        oam: &mut OamIterator,
        score: u8,
        side: Side,
    ) {
        // FIXME: refactor into some commonly useful score screen
        // FIXMEx2: isn't there a background layer for stuff like this?
        let digits: Vec<u8> = match score {
            0 => vec![0u8],
            _ => {
                let mut digits: Vec<u8> = Vec::new();
                let mut score_ = score.clone();
                while score_ != 0 {
                    digits.push(score_ % 10);
                    score_ /= 10;
                }
                digits
            }
        };

        let x0: u16 = match side {
            Side::LEFT => GBA_WIDTH / 2 - 16,
            Side::RIGHT => GBA_WIDTH / 2 + 16,
        } as u16;
        for (i, digit) in digits.iter().rev().enumerate() {
            let sprite_tag = SpriteTag::Numbers.tag().sprite(*digit as usize);
            let mut object = ObjectUnmanaged::new(loader.get_vram_sprite(sprite_tag));
            object.set_x(x0 + 4 * i as u16).set_y(8).show();
            if let Some(slot) = oam.next() {
                slot.set(&object);
            }
        }
    }
}

impl Game for PongGame {
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.system_player(time, &buttons);
        self.system_balls(time);
        self.system_cpu_paddle(self.opponent, time);
        self.system_collision(time);
        self.system_bounds(time);
        self.game_state.game_state()
    }

    // TODO: split into 2 steps - create sprite objects & then render according to z-axis
    fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) -> Option<()> {
        self.renderer_digits(loader, oam, self.game_state.player_score, Side::LEFT);
        self.renderer_digits(loader, oam, self.game_state.opponent_score, Side::RIGHT);

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
