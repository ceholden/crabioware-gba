use agb::display::object::{OamIterator, OamUnmanaged, ObjectUnmanaged, SpriteLoader};
use agb::display::tiled::VRamManager;
use agb::input::{Button, ButtonController};
use agb::interrupt::VBlank;
use agb::println;
use alloc::vec;
use alloc::vec::Vec;

use crate::games::{GameState, Games, Game};
use crate::graphics::{GraphicsResource, Mode0TileMap, TileMapResource, TileMode};

use super::graphics::SpriteTag;


struct GameEntry {
    game: Games,
    sprite: SpriteTag,
}

pub struct StartScreen {
    time: i32,
    games: Vec<GameEntry>,
    selection: u8,
}
impl StartScreen {
    pub fn new() -> Self {
        let games = vec![
            GameEntry {
                game: Games::Pong,
                sprite: SpriteTag::Pong,
            },
            GameEntry {
                game: Games::Snake,
                sprite: SpriteTag::Snake,
            },
            GameEntry {
                game: Games::PacCrab,
                sprite: SpriteTag::PacCrab,
            },
        ];

        Self {
            time: 0i32,
            games,
            selection: 0u8,
        }
    }

    pub fn pick_game(
        gba: &mut agb::Gba,
        buttons: &mut ButtonController,
        vblank: &VBlank,
    ) -> Games {

        let mut start_screen = Self::new();
        let (graphics, mut vram, mut unmanaged, mut sprite_loader) = TileMode::Mode0.create(gba);

        let mode0 = match graphics {
            GraphicsResource::Mode0(mode0) => mode0,
            _ => unimplemented!("WRONG MODE"),
        };

        let mut tiles = Mode0TileMap::default_32x32_4bpp(&mode0);
        tiles.set_visible(false);

        loop {
            buttons.update();
            if let Some(selected_game) = start_screen.update(buttons) {
                return selected_game
            };
            start_screen.render(&mut unmanaged, &mut sprite_loader);
            vblank.wait_for_vblank();
        }
    }

    fn update(
        &mut self,
        buttons: &ButtonController,
    ) -> Option<Games> {
        if buttons.is_just_pressed(Button::DOWN) && self.selection < (self.games.len() - 1) as u8 {
            println!("PRESSED DOWN");
            self.selection += 1;
        } else if buttons.is_just_pressed(Button::UP) && self.selection > 0 {
            println!("PRESSED UP");
            self.selection -= 1;
        }

        if buttons.is_just_pressed(Button::A) {
            let game = self.games[self.selection as usize].game;
            println!("SELECTING GAME index={}, game={:?}", self.selection, game);
            return Some(game)
        }
        None
    }

    fn render(
        &mut self,
        unmanaged: &mut OamUnmanaged,
        sprite_loader: &mut SpriteLoader,
    ) -> Option<()> {
        let mut oam = unmanaged.iter();

        let x0 = 16u16;
        let dx = 16u16;
        let y0 = 48u16;
        let dy = 16u16;

        for (i, game) in self.games.iter().enumerate() {
            let sprite_tag = game.sprite.tag().sprite(0);
            let mut object = ObjectUnmanaged::new(sprite_loader.get_vram_sprite(sprite_tag));

            let x = match self.selection == i as u8 {
                true => x0 - dx,
                false => x0,
            };

            object.set_x(x).set_y(y0 + i as u16 * dy).show();
            oam.next()?.set(&object);
        }

        Some(())
    }
}
