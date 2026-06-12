extern crate alloc;
use alloc::vec::Vec;

use agb::display::object::{OamUnmanaged, ObjectUnmanaged, SpriteLoader};
use agb::display::tiled::{MapLoan, RegularMap, TiledMap, VRamManager};
use agb::fixnum::{num, Vector2D};
use agb::input::ButtonController;
use agb::rng::RandomNumberGenerator;
use crabioware_core::ecs::{EntityId, World};
use crabioware_core::games::{Game, GameDifficulty, GameState};
use crabioware_core::graphics::{GraphicsResource, Mode0TileMap, TileMapResource, TileMode};
use crabioware_core::types::Number;

use crate::systems::{system_collision, system_dots};

use super::components::{
    Direction, DirectionComponent, GhostComponent, GhostKind, LocationComponent, PlayerComponent,
    SpeedComponent, SpriteComponent,
};
use super::graphics::SpriteTag;
use super::levels::{Level, Levels};
use super::systems::{system_ghost, system_player};

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
            tag: tag,
            tag_alt: SpriteTag::GhostScared,
            alt_mode: false,
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

pub struct PacCrabGame<'g> {
    world: World,
    // FIXME: ideally we wouldn't ever store entity IDs, and just query for ghosts/player by component
    player: EntityId,
    ghosts: Vec<EntityId>,
    rng: RandomNumberGenerator,
    time: i32,
    // FIXME: dots remaining for win condition - set at level load time based on tile data
    // dots_remaining: u32,
    dots_eaten: [bool; 600],
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

        Self {
            world,
            player,
            ghosts,
            rng: game_rng,
            time: 0,
            dots_eaten: [false; 600],
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

        system_player(&self.world, &self.level, &self.player, buttons);
        system_dots(&self.world, &self.level, &self.player, &mut self.dots_eaten);
        system_ghost(
            &self.world,
            &self.level,
            &self.ghosts,
            &self.player,
            &mut self.rng,
        );
        // FIXME: return dead ghosts here so we can put them into some reincarnation queue
        system_collision(&mut self.world, &self.level, &self.player, &mut self.ghosts)
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
                sprite_loader.get_vram_sprite(sprite.get_tag().tag().sprite(sprite.frame.into())),
            );
            object
                .set_position((location.location + sprite.offset).floor())
                .show();
            oam.next()?.set(&object);
        }

        Some(())
    }
}
