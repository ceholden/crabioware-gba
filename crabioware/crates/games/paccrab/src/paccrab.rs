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

use crate::systems::{CollisionResult, system_gate};

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

struct GhostArchetype {
    kind: GhostKind,
    tag: SpriteTag,
    scatter_tx: i32,
    scatter_ty: i32,
}

fn build_ghost_archetype(rng: &mut RandomNumberGenerator, level: &Level) -> GhostArchetype {
    let kind = GhostKind::random(rng);
    let tag = kind.tag();
    let scatter_tx = if rng.gen() >= 0 {
        1
    } else {
        level.dimensions.x as i32 - 2
    };
    let scatter_ty = if rng.gen() >= 0 {
        1
    } else {
        level.dimensions.y as i32 - 2
    };
    GhostArchetype {
        kind,
        tag,
        scatter_tx,
        scatter_ty,
    }
}

fn spawn_ghost(
    world: &mut World,
    x: Number,
    y: Number,
    start_dir: Direction,
    exit_threshold: usize,
    archetype: GhostArchetype,
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
            kind: archetype.kind,
            scatter_tx: archetype.scatter_tx,
            scatter_ty: archetype.scatter_ty,
            scared: false,
            exited: false,
            exit_threshold: exit_threshold,
            can_exit: false,
        })
        .with(SpriteComponent {
            tag: archetype.tag,
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
                bg.set_tile(
                    vram,
                    (x, y),
                    &gfx_tileset,
                    level.get_tilesetting(tile_id as usize),
                );
            }
        }
    }
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
}

fn render_gate(
    gate_open: bool,
    level: &Level,
    bg: &mut MapLoan<'_, RegularMap>,
    vram: &mut VRamManager,
) {
    let gfx_tileset = level.get_tileset();
    let tile_setting = if gate_open {
        TileSetting::BLANK
    } else {
        level.get_tilesetting(tileset::GATE_TILE_ID)
    };
    for &(dx, dy) in level.doors {
        bg.set_tile(vram, (dx as u16, dy as u16), &gfx_tileset, tile_setting);
    }
}

pub struct GateState {
    pub exit_queue: Vec<EntityId>,
    pub gate_open: bool,
}
impl GateState {
    fn new(ghosts: &Vec<EntityId>) -> GateState {
        let exit_queue: Vec<EntityId> = ghosts.clone();
        GateState {
            exit_queue,
            gate_open: false,
        }
    }
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
    gate_state: GateState,
    respawn_queue: Vec<(usize, usize)>, // timer and ghost spawn spot
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
            level.tile_center(level.spawn.0 as i32),
            level.tile_center(level.spawn.1 as i32),
        );

        let ghosts: Vec<EntityId> = level
            .ghosts
            .iter()
            .enumerate()
            .map(|(i, &(x, y))| {
                spawn_ghost(
                    &mut world,
                    level.tile_center(x as i32),
                    level.tile_center(y as i32),
                    Direction::UP,
                    level.total_dots - (50 * i / level.ghosts.len()),
                    build_ghost_archetype(rng, &level),
                )
            })
            .collect();

        spawn_world(&mut world, &level);

        let gate_state = GateState::new(&ghosts);

        Self {
            world,
            player,
            ghosts,
            rng: game_rng,
            time: 0,
            dots_remaining: level.total_dots,
            dots_eaten: [false; 600],
            dots_dirty: Vec::new(),
            gate_state: gate_state,
            respawn_queue: Vec::new(),
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

        render_walls(&self.level, &mut tiles.bg1, vram);
        tiles.bg1.commit(vram);
        tiles.bg1.set_visible(true);

        tiles.bg2.set_visible(true);
        render_dots(&self.level, &mut tiles.bg2, vram);
        render_gate(self.gate_state.gate_open, &self.level, &mut tiles.bg2, vram);
        tiles.bg2.commit(vram);
        tiles.bg2.set_visible(true);

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
        self.dots_remaining -= self.dots_dirty.len();

        system_gate(&self.world, &mut self.gate_state, self.dots_remaining);

        system_ghost(
            &self.world,
            &self.level,
            &self.ghosts,
            &self.player,
            &self.gate_state,
            &mut self.rng,
        );

        // FIXME: return dead ghosts here so we can put them into some reincarnation queue
        match system_collision(&mut self.world, &self.level, &self.player, &mut self.ghosts) {
            CollisionResult::PlayerDied => return GameState::GameOver,
            CollisionResult::GhostDied(n) => {
                for _ in 0..n {
                    let spawn_idx = self.ghosts.len() & self.level.ghosts.len();
                    self.respawn_queue.push((300, spawn_idx));
                };
            },
            CollisionResult::None => {}
        }

        self.respawn_queue.retain_mut(|(timer, spawn_idx)| {
            if *timer == 0 {
                let (sx, sy) = self.level.ghosts[*spawn_idx];
                let ghost = spawn_ghost(
                    &mut self.world,
                    self.level.tile_center(sx as i32),
                    self.level.tile_center(sy as i32),
                    Direction::UP,
                    self.dots_remaining.saturating_sub(25),
                    build_ghost_archetype(&mut self.rng, &self.level)
                );
                self.ghosts.push(ghost);
                self.gate_state.exit_queue.push(ghost);
                false
            } else {
                *timer -= 1;
                true
            }
        });

        if self.dots_remaining == 0 {
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

        if let Some(ref mut tiles) = self.tiles {
            let tileset = self.level.get_tileset();

            // Clear dots/pellet tiles that have been eaten
            for i in self.dots_dirty.drain(..) {
                let x = (i % self.level.dimensions.x as usize) as u16;
                let y = (i / self.level.dimensions.x as usize) as u16;
                tiles
                    .bg2
                    .set_tile(vram, (x, y), &tileset, TileSetting::BLANK);
            }

            render_gate(self.gate_state.gate_open, &self.level, &mut tiles.bg2, vram);

            tiles.bg2.commit(vram);
        }

        Some(())
    }
}
