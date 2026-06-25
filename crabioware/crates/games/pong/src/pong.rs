// Pong
use agb::{
    display::{
        affine::AffineMatrix,
        object::{
            AffineMatrixInstance, AffineMode, OamIterator, OamUnmanaged, ObjectUnmanaged,
            SpriteLoader,
        },
        tiled::VRamManager,
        HEIGHT as GBA_HEIGHT, WIDTH as GBA_WIDTH,
    },
    fixnum::num,
    input::{ButtonController, Tri},
    rng::RandomNumberGenerator,
};
use alloc::vec;
use alloc::vec::Vec;

use crabioware_core::games::{Game, GameDifficulty};
use crabioware_core::graphics::{GraphicsResource, Mode1TileMap, TileMapResource, TileMode};
use crabioware_core::physics::Intersects;
use crabioware_core::types::{Number, Rect, RectMath, Vector2D};
use crabioware_core::{
    ecs::{EntityId, World},
    games::{GameState, Games},
};

use crate::components::{
    CollisionComponent, LocationComponent, MaxSpeed, SpriteComponent, VelocityComponent,
};
use crate::graphics::SpriteTag;

// FIXME: keep score
#[allow(unused)]
struct GameStateResource {
    player_score: u8,
    opponent_score: u8,
    max_score: u8,
    max_speed: MaxSpeed,
    spawn: Side,
}
impl GameStateResource {
    fn new(difficulty: &GameDifficulty) -> Self {
        let max_speed = match difficulty {
            GameDifficulty::EASY => MaxSpeed::symmetric(num!(1.)),
            GameDifficulty::MEDIUM => MaxSpeed::symmetric(num!(1.5)),
            GameDifficulty::HARD => MaxSpeed::symmetric(num!(2.)),
        };
        GameStateResource {
            player_score: 0,
            opponent_score: 0,
            max_score: 10,
            max_speed,
            spawn: Side::LEFT,
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
        Self::new(&GameDifficulty::EASY)
    }
}

#[derive(Default)]
struct OpponentResource {
    target: Option<EntityId>,
    tracked_duration: u32,
}
impl OpponentResource {
    fn reset(target: Option<EntityId>) -> Self {
        Self {
            target,
            tracked_duration: 0,
        }
    }
}

struct Ball {
    sprite: SpriteComponent,
    location: LocationComponent,
    velocity: VelocityComponent,
    collision: CollisionComponent,
}
impl Ball {
    fn new(side: &Side, rng: &mut RandomNumberGenerator) -> Self {
        let sprite = SpriteComponent {
            tag: SpriteTag::Ball,
            offset: Default::default(),
            frame: 0,
        };
        let x_sign: i32 = match side {
            Side::LEFT => -1,
            Side::RIGHT => 1,
        };
        let y_sign: i32 = if rng.gen() % 2 == 0 { 1 } else { -1 };
        let velocity = Vector2D::<Number>::new(
            (x_sign * rng.gen().rem_euclid(5) + x_sign * 5).into(),
            (y_sign * rng.gen().rem_euclid(5) + y_sign * 5).into(),
        ) / num!(10.);

        Self {
            sprite,
            location: LocationComponent::centered(),
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
impl Side {
    fn next(&self) -> Self {
        match self {
            Side::LEFT => Side::RIGHT,
            Side::RIGHT => Side::LEFT,
        }
    }
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

fn system_player(world: &World, player: EntityId, time: i32, buttons: &ButtonController) {
    world.with::<(
        &mut LocationComponent,
        &mut VelocityComponent,
        &CollisionComponent,
    ), _, _>(&player, |(mut location, mut velocity, collision)| {
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
        clamp_paddle(&mut location, &mut velocity, &collision);
    });
}

fn system_balls(world: &World, balls: &[EntityId], time: i32) {
    for (mut location, velocity) in
        world.entries::<(&mut LocationComponent, &VelocityComponent)>(balls)
    {
        location.position += velocity.velocity * time;
        location.angle += velocity.rotation * time;
    }
}

fn system_cpu_acquire_target(
    world: &World,
    balls: &[EntityId],
    paddle_location: &LocationComponent,
    time: i32,
) -> (Option<EntityId>, Number) {
    let mut incoming = Vec::<(Number, EntityId, Vector2D<Number>, Vector2D<Number>)>::new();
    for (entity, location, velocity, collision) in world.entries::<(
        EntityId,
        &LocationComponent,
        &VelocityComponent,
        &CollisionComponent,
    )>(balls)
    {
        let delta = paddle_location.position.x - location.position.x;
        let eta = if velocity.velocity.x != num!(0.) {
            delta / velocity.velocity.x
        } else {
            num!(9999.)
        };
        if eta > num!(0.) {
            incoming.push((
                eta,
                entity,
                location.position + collision.collision.size / num!(2.),
                velocity.velocity,
            ))
        }
    }

    // FIXME: closest.. that we can reach
    if incoming.len() > 0 {
        incoming.sort_by(|(a, ..), (b, ..)| a.cmp(b));
        let (eta, entity, position, velocity) = incoming[0];
        if eta < num!(180.) {
            return (Some(entity), position.y + velocity.y * time);
        }
    }
    (None, Number::new(GBA_HEIGHT / 2))
}

fn system_cpu_track_target(
    world: &World,
    balls: &[EntityId],
    tracked_duration: u32,
    target: EntityId,
    paddle_location: &LocationComponent,
    time: i32,
) -> (Option<EntityId>, bool, Number) {
    if world.is_alive(&target) {
        // FIXME: check frames to impact against delta_y distance.. we might not make it!
        let paddle_pos_x = paddle_location.position.x;
        let result = world
            .with::<(&LocationComponent, &VelocityComponent, &CollisionComponent), _, _>(
                &target,
                |(ball_location, ball_velocity, ball_collision)| {
                    // Don't get hyper fixated on a target without rescanning
                    if tracked_duration < 60 {
                        let delta = paddle_pos_x - ball_location.position.x;
                        if delta * ball_velocity.velocity.x > num!(0.) {
                            let target_y = ball_location.position.y
                                + ball_collision.collision.size.y
                                + ball_velocity.velocity.y * time;
                            return Some((Some(target), true, target_y));
                        }
                    }
                    None
                },
            );
        if let Some(r) = result {
            return r;
        }
    }
    let (new_target, y_target) = system_cpu_acquire_target(world, balls, paddle_location, time);
    (new_target, false, y_target)
}

fn system_cpu_paddle(
    world: &World,
    entity: EntityId,
    balls: &[EntityId],
    opponent_state: &mut OpponentResource,
    game_state: &GameStateResource,
    time: i32,
) {
    // FIXME: increment opponent logic ~ GameDifficulty
    world.with::<(
        &mut LocationComponent,
        &mut VelocityComponent,
        &CollisionComponent,
    ), _, _>(&entity, |(mut location, mut velocity, collision)| {
        let (target, target_y) = match opponent_state.target {
            Some(target) => {
                let (new_target, tracked, target_y) = system_cpu_track_target(
                    world,
                    balls,
                    opponent_state.tracked_duration,
                    target,
                    &*location,
                    time,
                );
                if tracked {
                    opponent_state.tracked_duration += 1;
                } else {
                    *opponent_state = OpponentResource::reset(new_target);
                }
                (new_target, target_y)
            }
            None => system_cpu_acquire_target(world, balls, &*location, time),
        };
        opponent_state.target = target;

        let delta_y = target_y - location.position.y - collision.collision.size.y / num!(2.);
        let zero = num!(0.);
        let new_velocity_y = if delta_y < zero {
            velocity.velocity.y - velocity.acceleration.y * time
        } else if delta_y > zero {
            velocity.velocity.y + velocity.acceleration.y * time
        } else {
            0.into()
        };
        velocity.velocity.y = new_velocity_y;
        velocity.clamp_velocity(&game_state.max_speed);

        let move_range_y = velocity.velocity.y * time;
        let move_y = match delta_y.abs() < move_range_y.abs() {
            true => delta_y,
            false => move_range_y,
        };
        location.position.y += move_y;
        clamp_paddle(&mut location, &mut velocity, &collision);
    });
}

fn system_collision(world: &World, max_speed: &MaxSpeed) {
    // We're checking intersection based on potential movement, not
    // trajectory. If entities are moving really fast we might
    // have them phase through each other, but otherwise this is
    // a quicker way of checking collisions than continuous collision detection
    for (
        (mut location_a, mut velocity_a, collision_a),
        (mut location_b, mut velocity_b, collision_b),
    ) in world.combinations::<(
        &mut LocationComponent,
        &mut VelocityComponent,
        &CollisionComponent,
    )>() {
        let collision_box_a = collision_a.collision.translate(location_a.position);
        let collision_box_b = collision_b.collision.translate(location_b.position);

        if let Some(collided) = collision_box_a.separation(&collision_box_b) {
            let inv_masses = collision_a.inv_mass + collision_b.inv_mass;
            let delta_a = collided.separation * collision_a.inv_mass / inv_masses;
            let delta_b = collided.separation * collision_b.inv_mass / inv_masses;
            location_a.position -= delta_a;
            location_b.position += delta_b;

            let elasticity = collision_a.bounce.min(collision_b.bounce);
            let relative_velocity = velocity_a.velocity - velocity_b.velocity;
            let relative_velocity_norm = relative_velocity.dot(collided.normal);

            if relative_velocity_norm > num!(0.) {
                // FIXME: missing representation of tangent impulse + friction info
                let impulse = -(num!(1.) + elasticity) * relative_velocity_norm / inv_masses;
                velocity_a.velocity += collided.normal * impulse * collision_a.inv_mass;
                velocity_b.velocity -= collided.normal * impulse * collision_b.inv_mass;
                velocity_a.clamp_velocity(max_speed);
                velocity_b.clamp_velocity(max_speed);
            }
        }
    }
}

fn system_bounds(
    world: &mut World,
    game_state: &mut GameStateResource,
    balls: &mut Vec<EntityId>,
    game_rng: &mut RandomNumberGenerator,
) {
    let zero: Number = num!(0.);
    let mut scored = Vec::<EntityId>::new();
    for (entity, location, mut velocity, collision) in world.entries::<(
        EntityId,
        &LocationComponent,
        &mut VelocityComponent,
        &CollisionComponent,
    )>(balls)
    {
        if (location.position.y < zero && velocity.velocity.y < zero)
            || (location.position.y + collision.collision.size.y > GBA_HEIGHT.into()
                && velocity.velocity.y > zero)
        {
            velocity.velocity.y *= num!(-1.0)
        }
        if location.position.x < zero && velocity.velocity.x < zero {
            game_state.opponent_score += 1;
            scored.push(entity);
        }
        if location.position.x + collision.collision.size.x > GBA_WIDTH.into()
            && velocity.velocity.x > num!(0.)
        {
            game_state.player_score += 1;
            scored.push(entity);
        }
    }
    // iterator dropped — &World reborrow released, &mut World available again
    system_ball_scored(world, game_state, balls, game_rng, scored);
}

fn system_ball_scored(
    world: &mut World,
    game_state: &mut GameStateResource,
    balls: &mut Vec<EntityId>,
    game_rng: &mut RandomNumberGenerator,
    scored: Vec<EntityId>,
) {
    balls.retain(|b| !scored.contains(b));
    for ball in scored {
        world.destroy(&ball);
        let new_ball = Ball::new(&game_state.spawn, game_rng).create(world);
        game_state.spawn = game_state.spawn.next();
        balls.push(new_ball);
    }
}

fn clamp_paddle(
    location: &mut LocationComponent,
    velocity: &mut VelocityComponent,
    collision: &CollisionComponent,
) {
    let zero = num!(0.);
    if location.position.y < zero {
        location.position.y = zero;
        velocity.velocity.y = zero;
    } else if location.position.y + collision.collision.size.y > GBA_HEIGHT.into() {
        location.position.y = Number::new(GBA_HEIGHT) - collision.collision.size.y;
        velocity.velocity.y = zero;
    }
}

// TODO: add a "render cache" that helps us disconnect object setup and render
//       e.g., so we can sort on z-axis or priority
pub struct PongGame<'g> {
    world: World,
    game_rng: RandomNumberGenerator,
    player: EntityId,
    opponent: EntityId,
    balls: Vec<EntityId>,
    opponent_state: OpponentResource,
    game_state: GameStateResource,
    tiles: Option<Mode1TileMap<'g>>,
}
impl<'g> PongGame<'g> {
    pub fn new(difficulty: &GameDifficulty, rng: &mut RandomNumberGenerator) -> Self {
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

        let mut game_state = GameStateResource::new(difficulty);

        let balls: Vec<EntityId> = (0..2)
            .map(|_| {
                let ball = Ball::new(&game_state.spawn, &mut game_rng).create(&mut world);
                game_state.spawn = game_state.spawn.next();
                ball
            })
            .collect();

        Self {
            world,
            game_rng,
            player,
            opponent,
            balls,
            opponent_state: OpponentResource::default(),
            game_state,
            tiles: None,
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

impl<'g> Game<'g> for PongGame<'g> {
    fn renderer(&self) -> TileMode {
        TileMode::Mode1
    }

    fn clear(&mut self, vram: &mut VRamManager) {
        if let Some(tiles) = &mut self.tiles {
            tiles.clear(vram);
            tiles.commit(vram);
        }
    }

    fn init_tiles(&mut self, graphics: &'g GraphicsResource<'g>, vram: &mut VRamManager) {
        let mode1 = match graphics {
            GraphicsResource::Mode1(mode1) => mode1,
            _ => unimplemented!("WRONG MODE"),
        };

        let mut tiles = Mode1TileMap::default_32x32_4bpp(&mode1);
        tiles.set_visible(false);
        self.tiles = Some(tiles);
    }

    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        system_player(&self.world, self.player, time, buttons);
        system_balls(&self.world, &self.balls, time);
        system_cpu_paddle(
            &self.world,
            self.opponent,
            &self.balls,
            &mut self.opponent_state,
            &self.game_state,
            time,
        );
        system_collision(&self.world, &self.game_state.max_speed);
        system_bounds(
            &mut self.world,
            &mut self.game_state,
            &mut self.balls,
            &mut self.game_rng,
        );
        self.game_state.game_state()
    }

    // TODO: split into 2 steps - create sprite objects & then render according to z-axis
    fn render(
        &mut self,
        vram: &mut VRamManager,
        unmanaged: &mut OamUnmanaged,
        sprite_loader: &mut SpriteLoader,
    ) -> Option<()> {
        let mut oam = unmanaged.iter();

        self.renderer_digits(
            sprite_loader,
            &mut oam,
            self.game_state.player_score,
            Side::LEFT,
        );
        self.renderer_digits(
            sprite_loader,
            &mut oam,
            self.game_state.opponent_score,
            Side::RIGHT,
        );

        for (location, sprite) in self
            .world
            .components::<(&LocationComponent, &mut SpriteComponent)>()
        {
            let affine = AffineMatrixInstance::new(
                AffineMatrix::from_rotation(location.angle).to_object_wrapping(),
            );
            let position = (location.position + sprite.offset).floor();
            let mut object = ObjectUnmanaged::new(
                sprite_loader.get_vram_sprite(sprite.tag.tag().sprite(sprite.frame.into())),
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
