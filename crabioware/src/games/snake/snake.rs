use agb::{
    display::{
        object::{OamIterator, ObjectUnmanaged, SpriteLoader},
        Priority,
    },
    input::{ButtonController, Tri},
    rng::{self, RandomNumberGenerator},
};
use alloc::vec;
use alloc::vec::Vec;

use crate::games::{Game, GameDifficulty};
use crate::{
    ecs::{EntityId, World},
    games::{GameState, Games},
};

use super::components::{DirectionComponent, SpriteComponent, TileComponent};
use super::components::{N_TILES_TALL, N_TILES_WIDE};
use super::graphics::SpriteTag;

struct Berry {
    tile: TileComponent,
    sprite: SpriteComponent,
}
impl Berry {
    pub fn random(rng: &mut RandomNumberGenerator) -> Self {
        let tile = TileComponent::random(rng);
        Berry {
            tile,
            sprite: SpriteComponent {
                tag: SpriteTag::Berry,
                frame: 0,
            },
        }
    }

    pub fn create(&self, world: &mut World) -> EntityId {
        world.create().with(self.tile).with(self.sprite).build()
    }
}

struct Body {
    tile: TileComponent,
    sprite: SpriteComponent,
}
impl Body {
    pub fn new(tile: TileComponent) -> Self {
        Body {
            tile,
            sprite: SpriteComponent {
                tag: SpriteTag::Snake,
                frame: 0,
            },
        }
    }

    pub fn create(&self, world: &mut World) -> EntityId {
        world.create().with(self.tile).with(self.sprite).build()
    }
}

pub struct SnakeGame {
    world: World,
    rng: RandomNumberGenerator,
    head: EntityId,
    head_direction: DirectionComponent,
    body: Vec<EntityId>,
    berries: Vec<EntityId>,
    time: u32,
    speed: u8,
    score: u8,
    max_score: u8,
}
impl SnakeGame {
    pub fn new(_: &mut SpriteLoader, rng: &mut RandomNumberGenerator) -> Self {
        let mut world = World::new();
        world.register_component::<DirectionComponent>();
        world.register_component::<TileComponent>();
        world.register_component::<SpriteComponent>();

        let game_rng = RandomNumberGenerator::new_with_seed([
            rng::gen() as u32,
            rng::gen() as u32,
            rng::gen() as u32,
            rng::gen() as u32,
        ]);

        let head_tile_component = TileComponent::random(rng);
        let head_direction = DirectionComponent::random(rng);
        let head = world
            .create()
            .with(head_tile_component)
            .with(head_direction)
            .with(SpriteComponent {
                tag: SpriteTag::Snake,
                frame: 0,
            })
            .build();

        let body = vec![head];
        let berries = Vec::<EntityId>::new();

        // FIXME: implement difficulty selector
        let difficulty = GameDifficulty::MEDIUM;
        let max_score: u8 = match difficulty {
            GameDifficulty::EASY => 5,
            GameDifficulty::MEDIUM => 9,
            GameDifficulty::HARD => (N_TILES_WIDE * N_TILES_TALL / 3) as u8,
        };
        let speed: u8 = match difficulty {
            GameDifficulty::EASY => 60,
            GameDifficulty::MEDIUM => 30,
            GameDifficulty::HARD => 10,
        };

        SnakeGame {
            world,
            rng: game_rng,
            head,
            head_direction,
            body,
            berries,
            time: 0u32,
            speed,
            score: 0u8,
            max_score,
        }
    }

    fn system_controller(&mut self, buttons: &ButtonController) {
        let direction = self.world.entry::<&DirectionComponent>(self.head).clone();
        match buttons.x_tri() {
            Tri::Positive => {
                if direction != DirectionComponent::LEFT {
                    self.head_direction = DirectionComponent::RIGHT;
                }
            }
            Tri::Negative => {
                if direction != DirectionComponent::RIGHT {
                    self.head_direction = DirectionComponent::LEFT;
                }
            }
            _ => {}
        };
        match buttons.y_tri() {
            Tri::Positive => {
                if direction != DirectionComponent::UP {
                    self.head_direction = DirectionComponent::DOWN;
                }
            }
            Tri::Negative => {
                if direction != DirectionComponent::DOWN {
                    self.head_direction = DirectionComponent::UP;
                }
            }
            _ => {}
        };
    }

    fn system_head(&self, time: i32) -> TileComponent {
        // FIXME: snake sprite changes with direction
        let (mut direction, tile) = *self
            .world
            .entry::<(&mut DirectionComponent, &TileComponent)>(self.head);

        let new_tile = TileComponent {
            x: tile.x.wrapping_add(self.head_direction.dx() * time as i16),
            y: tile.y.wrapping_add(self.head_direction.dy() * time as i16),
        };
        direction.clone_from(&self.head_direction);
        new_tile
    }

    fn system_berry(&mut self, head_tile: &TileComponent) -> u8 {
        let mut eaten: u8 = 0;
        self.berries.retain(|&berry| {
            let berry_tile = self.world.entry::<&TileComponent>(berry).clone();
            if berry_tile.equals(&head_tile) {
                eaten += 1;
                self.world.destroy(berry);
                return false;
            }
            return true;
        });
        eaten
    }

    fn system_body(&mut self, head: &TileComponent, berries_eaten: u8) {
        // Store original body length
        let body_length = self.body.len();

        // Add new tail segment(s)
        if berries_eaten > 0 {
            let tail = self
                .world
                .entry::<&TileComponent>(self.body[body_length - 1])
                .clone();
            for _ in 0..berries_eaten {
                let new_tail = Body::new(tail.clone()).create(&mut self.world);
                self.body.push(new_tail);
            }
        }

        // Move the snake body up 1 segment
        for (i, body_from_tail) in self.body.iter().enumerate().rev() {
            let mut tile_body_from_tail = self.world.entry::<&mut TileComponent>(*body_from_tail);
            // Avoid move head on its own
            if i == 0 {
                tile_body_from_tail.x = head.x;
                tile_body_from_tail.y = head.y;
            } else {
                let tile_body_from_head = self.world.entry::<&TileComponent>(self.body[i - 1]);
                tile_body_from_tail.x = tile_body_from_head.x;
                tile_body_from_tail.y = tile_body_from_head.y;
            }
        }

        let mut head_tile = *self.world.entry::<&mut TileComponent>(self.head);
        head_tile.x = head.x;
        head_tile.y = head.y;
    }

    fn system_collide(&mut self, head: &TileComponent) -> GameState {
        // Head hit a wall
        if head.hit_wall() {
            return GameState::GameOver;
        }
        // Snake bit itself
        for body in self.body[1..].iter() {
            let body_tile = self.world.entry::<&TileComponent>(*body);
            if head.equals(&body_tile) {
                return GameState::GameOver;
            }
        }
        GameState::Running(Games::Snake)
    }

    fn renderer_digits(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) {
        // FIXME: refactor into some commonly useful score screen
        // FIXMEx2: isn't there a background layer for stuff like this?
        let digits: Vec<u8> = match self.score {
            0 => vec![0u8],
            _ => {
                let mut digits: Vec<u8> = Vec::new();
                let mut score_ = self.score.clone();
                while score_ != 0 {
                    digits.push(score_ % 10);
                    score_ /= 10;
                }
                digits
            }
        };
        for (i, digit) in digits.iter().rev().enumerate() {
            let sprite_tag = SpriteTag::Numbers.tag().sprite(*digit as usize);
            let mut object = ObjectUnmanaged::new(loader.get_vram_sprite(sprite_tag));
            object.set_x(8 + 4 * i as u16).set_y(8).show();
            if let Some(slot) = oam.next() {
                slot.set(&object);
            }
        }
    }
}
impl Game for SnakeGame {
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.time = self.time.wrapping_add_signed(time);

        // Spawn berries?
        if self.berries.len() == 0 {
            // FIXME: berries spawn randomly where snake isn't
            let berry = Berry::random(&mut self.rng).create(&mut self.world);
            self.berries.push(berry);
        }

        self.system_controller(buttons);

        // Only advance every FPS / speed ~+ 1/sec on easy
        if self.time % self.speed as u32 != 0 {
            return GameState::Running(Games::Snake);
        }

        let head_tile = self.system_head(time);

        let eaten = self.system_berry(&head_tile);
        self.system_body(&head_tile, eaten);
        let state = self.system_collide(&head_tile);
        match state {
            GameState::Running(game) => {
                self.score += eaten as u8;
                if self.score > self.max_score {
                    GameState::Win(game)
                } else {
                    state
                }
            }
            _ => state,
        }
    }

    fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) -> Option<()> {
        self.renderer_digits(loader, oam);

        let iter = self
            .world
            .components::<(&TileComponent, &SpriteComponent)>();

        for (tile, sprite) in iter {
            let sprite_tag = sprite.tag.tag().sprite(sprite.frame.into());
            let mut object = ObjectUnmanaged::new(loader.get_vram_sprite(sprite_tag));

            object
                .set_x(tile.position_x())
                .set_y(tile.position_y())
                .show();

            oam.next()?.set(&object);
        }

        Some(())
    }
}