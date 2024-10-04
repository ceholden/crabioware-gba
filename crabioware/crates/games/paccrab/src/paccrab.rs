use agb::display::object::{OamUnmanaged, ObjectUnmanaged, SpriteLoader};
use agb::display::tiled::{MapLoan, RegularMap, TiledMap, VRamManager};
use agb::fixnum::{num, Vector2D};
use agb::input::{Button, ButtonController};
use agb::println;

use agb::rng::RandomNumberGenerator;
use crabioware_core::ecs::{EntityId, World};
use crabioware_core::games::{Game, GameDifficulty, GameState, Games};
use crabioware_core::graphics::{GraphicsResource, Mode0TileMap, TileMapResource, TileMode};
use crabioware_core::types::{Number, Rect};

use super::components::{
    CollisionComponent, Direction, DirectionComponent, LocationComponent, SpriteComponent,
    VelocityComponent,
};
use super::graphics::SpriteTag;
use super::levels::{Level, Levels};

struct Crab {
    location: LocationComponent,
    direction: DirectionComponent,
    velocity: VelocityComponent,
    collision: CollisionComponent,
    sprite: SpriteComponent,
}
impl Crab {
    fn new(x: Number, y: Number) -> Self {
        Crab {
            location: LocationComponent {
                location: Vector2D { x, y },
            },
            direction: DirectionComponent {
                direction: Direction::RIGHT,
            },
            velocity: VelocityComponent {
                velocity: Vector2D {
                    x: num!(0.5),
                    y: num!(0.5),
                },
            },
            collision: CollisionComponent {
                collision: Rect {
                    position: Vector2D {
                        x: 4.into(),
                        y: 4.into(),
                    },
                    size: Vector2D {
                        x: 2.into(),
                        y: 8.into(),
                    },
                },
            },
            sprite: SpriteComponent {
                tag: SpriteTag::Crab,
                offset: Vector2D {
                    x: (-4).into(),
                    y: (-4).into(),
                },
                frame: 0,
            },
        }
    }
    fn create(self, world: &mut World) -> EntityId {
        world
            .create()
            .with(self.sprite)
            .with(self.location)
            .with(self.direction)
            .with(self.velocity)
            .with(self.collision)
            .build()
    }
}

pub struct PacCrabGame<'g> {
    world: World,
    player: EntityId,
    time: i32,
    level: Level,
    tiles: Option<Mode0TileMap<'g>>,
}
impl<'g> PacCrabGame<'g> {
    pub fn new(_: &GameDifficulty, _: &mut RandomNumberGenerator) -> Self {
        let mut world = World::new();
        world.register_component::<LocationComponent>();
        world.register_component::<VelocityComponent>();
        world.register_component::<DirectionComponent>();
        world.register_component::<CollisionComponent>();
        world.register_component::<SpriteComponent>();

        let level = Levels::LEVEL_1.get_level();
        let player =
            Crab::new(Number::new(level.spawn.0), Number::new(level.spawn.1)).create(&mut world);

        Self {
            world,
            player,
            time: 0i32,
            level,
            tiles: None,
        }
    }

    fn render_tiles(&self, bg1: &mut MapLoan<'g, RegularMap>, vram: &mut VRamManager) {
        self.level.set_background_paelttes(vram);

        let tileset = self.level.get_tileset();

        for y in 0..20u16 {
            for x in 0..30u16 {
                let tile_id = self.level.walls[(y * 30 + x) as usize] - 1;
                println!("x/y=({},{}) tile_id={}", x, y, tile_id);
                bg1.set_tile(
                    vram,
                    (x, y),
                    &tileset,
                    self.level.get_tilesetting(tile_id as usize),
                );
            }
        }
        bg1.commit(vram);
        bg1.set_visible(true);
    }

    fn system_player(&self, _time: i32, buttons: &ButtonController) {
        println!("GRABBING COMPONENTS");
        let (mut location, mut direction, velocity, _collision) =
            *self.world.entry::<(
                &mut LocationComponent,
                &mut DirectionComponent,
                &VelocityComponent,
                &CollisionComponent,
            )>(&self.player);

        println!("GETTING DIRECTION");

        if buttons.is_pressed(Button::LEFT) {
            direction.direction = Direction::LEFT;
        } else if buttons.is_pressed(Button::RIGHT) {
            direction.direction = Direction::RIGHT;
        } else if buttons.is_pressed(Button::UP) {
            direction.direction = Direction::UP;
        } else if buttons.is_pressed(Button::DOWN) {
            direction.direction = Direction::DOWN;
        }

        println!("MOVING");
        match direction.direction {
            Direction::RIGHT => {
                location.location.x += velocity.velocity.x;
            }
            Direction::LEFT => {
                location.location.x -= velocity.velocity.x;
            }
            Direction::UP => {
                location.location.y -= velocity.velocity.y;
            }
            Direction::DOWN => {
                location.location.y += velocity.velocity.y;
            }
        }
    }
}
impl<'g> Game<'g> for PacCrabGame<'g> {
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
        tiles.bg1.set_visible(true);
        self.render_tiles(&mut tiles.bg1, vram);
        self.tiles = Some(tiles);
    }

    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.time += time;
        println!("RUNNING PACCRAB");

        self.system_player(time, buttons);

        // FIXME: this is not a good exit condition
        if buttons.is_just_pressed(Button::SELECT) {
            GameState::GameOver
        } else {
            GameState::Running(Games::PacCrab)
        }
    }

    fn render(
        &mut self,
        _vram: &mut VRamManager,
        unmanaged: &mut OamUnmanaged,
        sprite_loader: &mut SpriteLoader,
    ) -> Option<()> {
        let mut oam = unmanaged.iter();

        for (location, sprite) in self
            .world
            .components::<(&LocationComponent, &SpriteComponent)>()
        {
            let mut object = ObjectUnmanaged::new(
                sprite_loader.get_vram_sprite(sprite.tag.tag().sprite(sprite.frame.into())),
            );
            object
                .set_position((location.location + sprite.offset).floor())
                .show();
            oam.next()?.set(&object);
        }

        Some(())
    }
}
