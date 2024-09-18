use agb::display::object::{OamUnmanaged, ObjectUnmanaged, SpriteLoader};
use agb::display::tiled::{
    MapLoan, RegularBackgroundSize, RegularMap, TileFormat, TiledMap, VRamManager,
};
use agb::fixnum::Vector2D;
use agb::input::{Button, ButtonController};
use agb::println;

use agb::rng::RandomNumberGenerator;
use crabioware_core::ecs::{EntityId, World};
use crabioware_core::games::{Game, GameDifficulty, GameState, Games};
use crabioware_core::graphics::{GraphicsResource, Mode0TileMap, TileMapResource, TileMode};
use crabioware_core::types::Number;

use super::components::{SpriteComponent, LocationComponent, VelocityComponent};
use super::graphics::SpriteTag;
use super::levels::{Level, Levels};


struct Crab {
    sprite: SpriteComponent,
    location: LocationComponent,
    velocity: VelocityComponent,
}
impl Crab {
    fn new(x: Number, y: Number) -> Self {
        Crab {
            sprite: SpriteComponent { tag: SpriteTag::Crab, frame: 0 },
            location: LocationComponent { location: Vector2D { x, y } },
            velocity: VelocityComponent { velocity: Vector2D { x: 0.into(), y: 0.into() } },
        }
    }
    fn create(self, world: &mut World) -> EntityId {
        world
            .create()
            .with(self.sprite)
            .with(self.location)
            .with(self.velocity)
            .build()
    }
}


pub struct PacCrabGame<'g> {
    world: World,
    time: i32,
    level: Level,
    tiles: Option<Mode0TileMap<'g>>,
}
impl<'g> PacCrabGame<'g> {
    pub fn new(difficulty: &GameDifficulty, rng: &mut RandomNumberGenerator) -> Self {

        let mut world = World::new();
        world.register_component::<SpriteComponent>();
        world.register_component::<LocationComponent>();
        world.register_component::<VelocityComponent>();

        let level = Levels::LEVEL_1.get_level();
        let crab = Crab::new(Number::new(level.spawn.0), Number::new(level.spawn.1)).create(&mut world);

        Self {
            world,
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
        if self.time < 200 {
            GameState::Running(Games::PacCrab)
        } else {
            GameState::GameOver
        }
    }

    fn render(
        &mut self,
        vram: &mut VRamManager,
        unmanaged: &mut OamUnmanaged,
        sprite_loader: &mut SpriteLoader,
    ) -> Option<()> {
        let mut oam = unmanaged.iter();

        for (location, sprite) in self.world.components::<(&LocationComponent, &SpriteComponent)>() {
            let mut object = ObjectUnmanaged::new(
                sprite_loader.get_vram_sprite(sprite.tag.tag().sprite(sprite.frame.into()))
            );
            object
                .set_position(location.location.floor())
                .show();
            oam.next()?.set(&object);
        }

        Some(())
    }
}
