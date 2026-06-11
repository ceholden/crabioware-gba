extern crate alloc;
use alloc::vec::Vec;

use agb::display::object::{OamUnmanaged, ObjectUnmanaged, SpriteLoader};
use agb::display::tiled::{MapLoan, RegularMap, TiledMap, VRamManager};
use agb::fixnum::{num, Vector2D};
use agb::input::{Button, ButtonController};
use agb::rng::RandomNumberGenerator;
use crabioware_core::ecs::{EntityId, World};
use crabioware_core::games::{Game, GameDifficulty, GameState, Games};
use crabioware_core::graphics::{GraphicsResource, Mode0TileMap, TileMapResource, TileMode};
use crabioware_core::types::Number;

use super::ai::ghost_desired;
use super::components::{
    Direction, DirectionComponent, GhostComponent, GhostKind, LocationComponent, PlayerComponent,
    SpeedComponent, SpriteComponent,
};
use super::graphics::SpriteTag;
use super::levels::{Level, Levels};
use super::movement::{apply_movement, tile_of};

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
        .with(PlayerComponent)
        .with(SpriteComponent {
            tag: SpriteTag::Crab,
            offset: Vector2D {
                x: (-4).into(),
                y: (-4).into(),
            },
            frame: 0,
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
        })
        .with(SpriteComponent {
            tag,
            offset: Vector2D {
                x: (-4).into(),
                y: (-4).into(),
            },
            frame: 0,
        })
        .build()
}

fn render_tiles(level: &Level, bg1: &mut MapLoan<'_, RegularMap>, vram: &mut VRamManager) {
    level.set_background_paelttes(vram);
    let tileset = level.get_tileset();
    for y in 0..level.dimensions.y as u16 {
        for x in 0..level.dimensions.x as u16 {
            let tile_id = level.walls[(y as u32 * level.dimensions.x + x as u32) as usize] - 1;
            bg1.set_tile(
                vram,
                (x, y),
                &tileset,
                level.get_tilesetting(tile_id as usize),
            );
        }
    }
    bg1.commit(vram);
    bg1.set_visible(true);
}

fn system_player(world: &World, player: &EntityId, level: &Level, buttons: &ButtonController) {
    world.with::<(
        &mut LocationComponent,
        &mut DirectionComponent,
        &SpeedComponent,
    ), _, _>(player, |(mut location, mut direction, speed)| {
        if buttons.is_pressed(Button::LEFT) {
            direction.desired = Direction::LEFT;
        } else if buttons.is_pressed(Button::RIGHT) {
            direction.desired = Direction::RIGHT;
        } else if buttons.is_pressed(Button::UP) {
            direction.desired = Direction::UP;
        } else if buttons.is_pressed(Button::DOWN) {
            direction.desired = Direction::DOWN;
        }

        apply_movement(
            &mut location,
            &mut direction,
            speed.0,
            |tile_x, tile_y| level.is_walkable_tile(tile_x, tile_y),
        );
    });
}

fn system_ghost(
    world: &World,
    ghosts: &[EntityId],
    level: &Level,
    player_tx: i32,
    player_ty: i32,
    player_dir: Direction,
    rng: &mut RandomNumberGenerator,
) {
    for ghost in ghosts {
        world.with::<(
            &mut LocationComponent,
            &mut DirectionComponent,
            &SpeedComponent,
            &mut GhostComponent,
        ), _, _>(
            ghost,
            |(mut location, mut direction, speed, mut ghost_comp)| {
                let tx = tile_of(location.location.x);
                let ty = tile_of(location.location.y);
                direction.desired = ghost_desired(
                    &mut ghost_comp,
                    tx,
                    ty,
                    direction.direction,
                    player_tx,
                    player_ty,
                    player_dir,
                    level,
                    rng,
                );
                apply_movement(
                    &mut location,
                    &mut direction,
                    speed.0,
                    |tile_x, tile_y| level.is_ghost_walkable_tile(tile_x, tile_y),
                );
            },
        );
    }
}

pub struct PacCrabGame<'g> {
    world: World,
    player: EntityId,
    ghosts: Vec<EntityId>,
    rng: RandomNumberGenerator,
    time: i32,
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
            Number::new(level.spawn.0),
            Number::new(level.spawn.1),
        );

        // TODO: exit gate and logic
        let ghosts: Vec<EntityId> = level
            .ghosts
            .iter()
            .zip([
                (
                    GhostKind::Patrol {
                        chase_ticks: 120,
                        shy_ticks: 60,
                        timer: 0,
                        chasing: false,
                    },
                    Direction::UP,
                    1,
                    1,
                    SpriteTag::GhostPink,
                ),
                (
                    GhostKind::Ambush,
                    Direction::UP,
                    28,
                    1,
                    SpriteTag::GhostYellow,
                ),
                (
                    GhostKind::Random,
                    Direction::UP,
                    1,
                    18,
                    SpriteTag::GhostBlue,
                ),
            ])
            .map(|(&(x, y), (kind, dir, stx, sty, tag))| {
                spawn_ghost(
                    &mut world,
                    Number::new(x),
                    Number::new(y),
                    kind,
                    dir,
                    stx,
                    sty,
                    tag,
                )
            })
            .collect();

        Self {
            world,
            player,
            ghosts,
            rng: game_rng,
            time: 0,
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
        tiles.bg1.set_visible(true);
        render_tiles(&self.level, &mut tiles.bg1, vram);
        self.tiles = Some(tiles);
    }

    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.time += time;

        // Snapshot player tile position for ghost AI before running player system
        let (player_tx, player_ty, player_dir) =
            self.world
                .with::<(&LocationComponent, &DirectionComponent), _, _>(
                    &self.player,
                    |(loc, dir)| {
                        (
                            tile_of(loc.location.x),
                            tile_of(loc.location.y),
                            dir.direction,
                        )
                    },
                );

        system_player(&self.world, &self.player, &self.level, buttons);
        system_ghost(
            &self.world,
            &self.ghosts,
            &self.level,
            player_tx,
            player_ty,
            player_dir,
            &mut self.rng,
        );

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
