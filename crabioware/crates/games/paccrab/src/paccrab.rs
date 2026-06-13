extern crate alloc;
use alloc::vec::Vec;

use agb::display::object::{OamUnmanaged, ObjectUnmanaged, SpriteLoader};
use agb::display::tiled::{MapLoan, RegularMap, TileSetting, TiledMap, VRamManager};
use agb::fixnum::{num, Vector2D};
use agb::input::ButtonController;
use agb::rng::RandomNumberGenerator;
use crabioware_core::ecs::{EntityId, World};
use crabioware_core::games::{Game, GameDifficulty, GameState, Games};
use crabioware_core::graphics::{GraphicsResource, Mode0TileMap, TileMapResource, TileMode};
use crabioware_core::types::Number;

use super::components::{
    Direction, DirectionComponent, GhostComponent, GhostKind, LocationComponent, PlayerComponent,
    SpeedComponent, SpriteComponent,
};
use super::graphics::SpriteTag;
use super::levels::{tilemaps::tileset, Level, Levels};
use super::systems::{system_collision, system_dots, system_ghost, system_player};

fn spawn_crab(world: &mut World, x: Number, y: Number) -> EntityId {
    world
        .create()
        .with(LocationComponent {
            location: Vector2D { x, y },
        })
        .with(DirectionComponent {
            direction: Direction::RIGHT,
            desired: Direction::RIGHT,
        })
        .with(SpeedComponent(num!(0.5)))
        .with(PlayerComponent { energized_time: 0 })
        .with(SpriteComponent {
            tag: SpriteTag::Crab,
            tag_alt: SpriteTag::SuperCrab,
            alt_mode: false,
            offset: Vector2D {
                x: (-4).into(),
                y: (-4).into(),
            },
            animation_interval: 1,
        })
        .build()
}

fn spawn_ghost(
    world: &mut World,
    x: Number,
    y: Number,
    kind: GhostKind,
    start_dir: Direction,
    scatter_tx: i32,
    scatter_ty: i32,
    tag: SpriteTag,
) -> EntityId {
    world
        .create()
        .with(LocationComponent {
            location: Vector2D { x, y },
        })
        .with(DirectionComponent {
            direction: start_dir,
            desired: start_dir,
        })
        .with(SpeedComponent(num!(0.5)))
        .with(GhostComponent {
            kind,
            scatter_tx,
            scatter_ty,
            scared: false,
        })
        .with(SpriteComponent {
            tag: tag,
            tag_alt: SpriteTag::GhostScared,
            alt_mode: false,
            offset: Vector2D {
                x: (-4).into(),
                y: (-4).into(),
            },
            animation_interval: 1,
        })
        .build()
}

/// Spawn static world sprite elements
fn spawn_world(world: &mut World, level: &Level) {
    // Spawn warp pads
    for (wx, wy) in level.warps {
        world
            .create()
            .with(LocationComponent {
                location: Vector2D {
                    x: level.tile_center(*wx as i32),
                    y: level.tile_center(*wy as i32),
                },
            })
            .with(SpriteComponent {
                tag: SpriteTag::Warp,
                tag_alt: SpriteTag::Warp,
                alt_mode: false,
                offset: Vector2D {
                    x: (-4).into(),
                    y: (-4).into(),
                },
                animation_interval: 25,
            })
            .build();
    }
}

fn render_walls(level: &Level, bg: &mut MapLoan<'_, RegularMap>, vram: &mut VRamManager) {
    let gfx_tileset = level.get_tileset();
    for y in 0..level.dimensions.y as u16 {
        for x in 0..level.dimensions.x as u16 {
            let tile_id = level.walls[(y as u32 * level.dimensions.x + x as u32) as usize];
            if tile_id != 0xFF {
                bg.set_tile(vram, (x, y), &gfx_tileset, level.get_tilesetting(tile_id as usize));
            }
        }
    }
    bg.commit(vram);
    bg.set_visible(true);
}

fn render_dots(level: &Level, bg: &mut MapLoan<'_, RegularMap>, vram: &mut VRamManager) {
    let gfx_tileset = level.get_tileset();
    for y in 0..level.dimensions.y as u16 {
        for x in 0..level.dimensions.x as u16 {
            let tile_id = level.dots[(y as u32 * level.dimensions.x + x as u32) as usize];
            let tilesetting = match tile_id {
                t if t == tileset::DOT_TILE_ID as u8 => {
                    Some(level.get_tilesetting(tileset::DOT_TILE_ID))
                }
                t if t == tileset::PELLET_TILE_ID as u8 => {
                    Some(level.get_tilesetting(tileset::PELLET_TILE_ID))
                }
                _ => None,
            };
            if let Some(tilesetting) = tilesetting {
                bg.set_tile(vram, (x, y), &gfx_tileset, tilesetting);
            }
        }
    }
    bg.commit(vram);
    bg.set_visible(true);
}

pub struct PacCrabGame<'g> {
    world: World,
    // FIXME: ideally we wouldn't ever store entity IDs, and just query for ghosts/player by component
    player: EntityId,
    ghosts: Vec<EntityId>,
    rng: RandomNumberGenerator,
    time: usize,
    dots_remaining: usize,
    dots_eaten: [bool; 600],
    dots_dirty: Vec<usize>,
    level: Level,
    tiles: Option<Mode0TileMap<'g>>,
}
impl<'g> PacCrabGame<'g> {
    pub fn new(_: &GameDifficulty, rng: &mut RandomNumberGenerator) -> Self {
        let mut world = World::new();
        world.register_component::<LocationComponent>();
        world.register_component::<DirectionComponent>();
        world.register_component::<SpeedComponent>();
        world.register_component::<PlayerComponent>();
        world.register_component::<GhostComponent>();
        world.register_component::<SpriteComponent>();

        let game_rng = RandomNumberGenerator::new_with_seed([
            rng.gen().abs() as u32,
            rng.gen().abs() as u32,
            rng.gen().abs() as u32,
            rng.gen().abs() as u32,
        ]);

        let level = Levels::LEVEL_1.get_level();
        let player = spawn_crab(
            &mut world,
            Number::new(level.spawn.0 as i32 * level.tile_size as i32),
            Number::new(level.spawn.1 as i32 * level.tile_size as i32),
        );

        // TODO: exit gate and logic
        let ghosts: Vec<EntityId> = level
            .ghosts
            .iter()
            .zip([
                (GhostKind::Chase, Direction::UP, 1, 1, SpriteTag::GhostPink),
                (
                    GhostKind::Ambush,
                    Direction::UP,
                    28,
                    1,
                    SpriteTag::GhostYellow,
                ),
                (
                    GhostKind::Patrol {
                        chase_ticks: 120,
                        shy_ticks: 60,
                        timer: 0,
                        chasing: false,
                    },
                    Direction::UP,
                    1,
                    18,
                    SpriteTag::GhostOrange,
                ),
            ])
            .map(|(&(x, y), (kind, dir, stx, sty, tag))| {
                spawn_ghost(
                    &mut world,
                    Number::new(x as i32 * level.tile_size as i32),
                    Number::new(y as i32 * level.tile_size as i32),
                    kind,
                    dir,
                    stx,
                    sty,
                    tag,
                )
            })
            .collect();

        spawn_world(&mut world, &level);

        Self {
            world,
            player,
            ghosts,
            rng: game_rng,
            time: 0,
            dots_remaining: level.total_dots,
            dots_eaten: [false; 600],
            dots_dirty: Vec::new(),
            level,
            tiles: None,
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

        self.level.set_background_palettes(vram);

        tiles.bg1.set_visible(true);
        render_walls(&self.level, &mut tiles.bg1, vram);

        tiles.bg2.set_visible(true);
        render_dots(&self.level, &mut tiles.bg2, vram);

        self.tiles = Some(tiles);
    }

    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.time = self.time.saturating_add(time as usize);

        system_player(&self.world, &self.level, &self.player, buttons);
        system_dots(
            &self.world,
            &self.level,
            &self.player,
            &mut self.dots_eaten,
            &mut self.dots_dirty,
        );
        system_ghost(
            &self.world,
            &self.level,
            &self.ghosts,
            &self.player,
            &mut self.rng,
        );
        // FIXME: return dead ghosts here so we can put them into some reincarnation queue
        let player_dead =
            system_collision(&mut self.world, &self.level, &self.player, &mut self.ghosts);

        self.dots_remaining -= self.dots_dirty.len();
        agb::println!("Dots remaining {}", self.dots_remaining);

        if player_dead {
            GameState::GameOver
        } else if self.dots_remaining == 0 {
            GameState::Win(Games::PacCrab)
        } else {
            GameState::Running(Games::PacCrab)
        }
    }

    fn render(
        &mut self,
        vram: &mut VRamManager,
        unmanaged: &mut OamUnmanaged,
        sprite_loader: &mut SpriteLoader,
    ) -> Option<()> {
        let mut oam = unmanaged.iter();

        for (location, sprite) in self
            .world
            .components::<(&LocationComponent, &SpriteComponent)>()
        {
            let mut object = ObjectUnmanaged::new(
                sprite_loader.get_vram_sprite(
                    sprite
                        .get_tag()
                        .tag()
                        .animation_sprite(self.time / sprite.animation_interval),
                ),
            );
            object
                .set_position((location.location + sprite.offset).floor())
                .show();
            oam.next()?.set(&object);
        }

        // Clear dots/pellet tiles that have been eaten
        if let Some(ref mut tiles) = self.tiles {
            let tileset = self.level.get_tileset();
            for &i in &self.dots_dirty {
                let x = (i % self.level.dimensions.x as usize) as u16;
                let y = (i / self.level.dimensions.x as usize) as u16;
                tiles
                    .bg2
                    .set_tile(vram, (x, y), &tileset, TileSetting::BLANK);
            }
            self.dots_dirty.clear();
            tiles.bg2.commit(vram);
        }

        Some(())
    }
}
