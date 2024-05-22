use agb::{
    display::{
        object::{OamIterator, OamUnmanaged, ObjectUnmanaged, SpriteLoader},
        tiled::VRamManager,
    },
    input::{ButtonController, Tri},
    rng::RandomNumberGenerator,
};
use alloc::vec;
use alloc::vec::Vec;

use crabioware_core::{
    ecs::{EntityId, World},
    games::{Game, GameDifficulty, GameState, Games},
    graphics::{GraphicsResource, Mode0TileMap, TileMapResource, TileMode},
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

    pub fn create(self, world: &mut World) -> EntityId {
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

    pub fn create(self, world: &mut World) -> EntityId {
        world.create().with(self.tile).with(self.sprite).build()
    }
}

struct GameStateResource {
    time: u32,
    speed: u8,
    score: u8,
    max_score: u8,
}
impl GameStateResource {
    fn new(difficulty: &GameDifficulty) -> GameStateResource {
        let max_score: u8 = match difficulty {
            GameDifficulty::EASY => 5,
            GameDifficulty::MEDIUM => 9,
            GameDifficulty::HARD => (N_TILES_WIDE * N_TILES_TALL / 4) as u8,
        };
        let speed: u8 = match difficulty {
            GameDifficulty::EASY => 45,
            GameDifficulty::MEDIUM => 30,
            GameDifficulty::HARD => 15,
        };
        GameStateResource {
            time: 0,
            speed,
            score: 0,
            max_score,
        }
    }
}

pub struct SnakeGame<'g> {
    world: World,
    rng: RandomNumberGenerator,
    // Store head direction and commit upon movement
    head_direction: DirectionComponent,
    // Store entities separately, sort of like a hacky archetype
    body: Vec<EntityId>,
    berries: Vec<EntityId>,
    game_state: GameStateResource,
    // background tilemap
    tiles: Option<Mode0TileMap<'g>>,
}
impl<'g> SnakeGame<'g> {
    pub fn new(difficulty: &GameDifficulty, rng: &mut RandomNumberGenerator) -> Self {
        let mut world = World::new();
        world.register_component::<DirectionComponent>();
        world.register_component::<TileComponent>();
        world.register_component::<SpriteComponent>();

        let mut game_rng = RandomNumberGenerator::new_with_seed([
            rng.gen().abs() as u32,
            rng.gen().abs() as u32,
            rng.gen().abs() as u32,
            rng.gen().abs() as u32,
        ]);

        let head_tile_component = TileComponent::random(&mut game_rng);
        let head_direction = DirectionComponent::random(&mut game_rng);
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

        let berry = Berry::random(&mut game_rng).create(&mut world);
        let berries = vec![berry];

        SnakeGame {
            world,
            rng: game_rng,
            head_direction,
            body,
            berries,
            game_state: GameStateResource::new(difficulty),
            tiles: None,
        }
    }

    fn system_controller(&mut self, buttons: &ButtonController) {
        let direction = **self.world.entry::<&DirectionComponent>(&self.body[0]);
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
            .entry::<(&mut DirectionComponent, &TileComponent)>(&self.body[0]);

        let new_tile = TileComponent {
            x: tile.x.wrapping_add(self.head_direction.dx() * time as i16),
            y: tile.y.wrapping_add(self.head_direction.dy() * time as i16),
        };
        direction.clone_from(&self.head_direction);
        new_tile
    }

    fn system_spawn_berry(&mut self) {
        // Spawn berries?
        if self.berries.len() == 0 {
            // FIXME: berries spawn randomly where snake isn't
            let berry = Berry::random(&mut self.rng).create(&mut self.world);
            self.berries.push(berry);
        }
    }

    // Check if head has eaten a berry, returning nutritional content of berry
    fn system_eat_berry(&mut self, head_tile: &TileComponent) -> u8 {
        let mut eaten: u8 = 0;
        self.berries.retain(|&berry| {
            let berry_tile = self.world.entry::<&TileComponent>(&berry).clone();
            if berry_tile.equals(&head_tile) {
                eaten += 1;
                self.world.destroy(&berry);
                return false;
            }
            return true;
        });
        eaten
    }

    fn system_body(&mut self, next_head: &TileComponent, berries_eaten: u8) {
        // Store original body length
        let body_length = self.body.len();

        // Add new tail segment(s)
        if berries_eaten > 0 {
            let tail = self
                .world
                .entry::<&TileComponent>(&self.body[body_length - 1])
                .clone();
            for _ in 0..berries_eaten {
                let new_tail = Body::new(tail.clone()).create(&mut self.world);
                self.body.push(new_tail);
            }
        }

        // Move the snake body up 1 segment
        for (i, body_from_tail) in self.body.iter().enumerate().rev() {
            let mut tile_body_from_tail = self.world.entry::<&mut TileComponent>(body_from_tail);
            if i == 0 {
                // Move head to next tile
                tile_body_from_tail.x = next_head.x;
                tile_body_from_tail.y = next_head.y;
            } else {
                // Or move tail one segment closer to head
                let tile_body_from_head = self.world.entry::<&TileComponent>(&self.body[i - 1]);
                tile_body_from_tail.x = tile_body_from_head.x;
                tile_body_from_tail.y = tile_body_from_head.y;
            }
        }
    }

    fn system_collide(&mut self, head: &TileComponent) -> GameState {
        // Head hit a wall
        if head.hit_wall() {
            return GameState::GameOver;
        }
        // Snake bit itself
        for body in self.body[1..].iter() {
            let body_tile = self.world.entry::<&TileComponent>(body);
            if head.equals(&body_tile) {
                return GameState::GameOver;
            }
        }
        GameState::Running(Games::Snake)
    }

    fn renderer_digits(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) {
        // FIXME: refactor into some commonly useful score screen
        // FIXMEx2: isn't there a background layer for stuff like this?
        let digits: Vec<u8> = match self.game_state.score {
            0 => vec![0u8],
            _ => {
                let mut digits: Vec<u8> = Vec::new();
                let mut score_ = self.game_state.score.clone();
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
impl<'g> Game<'g> for SnakeGame<'g> {
    fn renderer(&self) -> TileMode {
        TileMode::Mode0
    }

    fn clear(&mut self, vram: &mut VRamManager) {
        if let Some(tiles) = &mut self.tiles {
            tiles.clear(vram);
            tiles.commit(vram);
        }
    }

    fn init_tiles(&mut self, graphics: &'g GraphicsResource<'g>, vram: &mut VRamManager) {
        let mode0 = match graphics {
            GraphicsResource::Mode0(mode0) => mode0,
            _ => unimplemented!("WRONG MODE"),
        };

        let mut tiles = Mode0TileMap::default_32x32_4bpp(&mode0);
        tiles.set_visible(false);
        self.tiles = Some(tiles);
    }

    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.game_state.time = self.game_state.time.wrapping_add_signed(time);

        self.system_controller(buttons);

        // Only advance every FPS / speed ~+ 1/sec on easy
        if self.game_state.time % self.game_state.speed as u32 != 0 {
            return GameState::Running(Games::Snake);
        }

        // Move head tile in direction
        let head_tile = self.system_head(time);

        let eaten = self.system_eat_berry(&head_tile);
        self.system_spawn_berry();

        self.system_body(&head_tile, eaten);

        let state = self.system_collide(&head_tile);
        match state {
            GameState::Running(game) => {
                self.game_state.score += eaten as u8;
                if self.game_state.score > self.game_state.max_score {
                    GameState::Win(game)
                } else {
                    state
                }
            }
            _ => state,
        }
    }

    fn render(
        &mut self,
        vram: &mut VRamManager,
        unmanaged: &mut OamUnmanaged,
        sprite_loader: &mut SpriteLoader,
    ) -> Option<()> {
        let mut oam = unmanaged.iter();

        self.renderer_digits(sprite_loader, &mut oam);

        let iter = self
            .world
            .components::<(&TileComponent, &SpriteComponent)>();

        for (tile, sprite) in iter {
            let sprite_tag = sprite.tag.tag().sprite(sprite.frame.into());
            let mut object = ObjectUnmanaged::new(sprite_loader.get_vram_sprite(sprite_tag));

            object
                .set_x(tile.position_x())
                .set_y(tile.position_y())
                .show();

            oam.next()?.set(&object);
        }

        Some(())
    }
}
