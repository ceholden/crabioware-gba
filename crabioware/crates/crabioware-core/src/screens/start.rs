use agb::display::object::{OamIterator, ObjectUnmanaged, SpriteLoader};
use agb::input::{Button, ButtonController};
use agb::println;
use alloc::vec;
use alloc::vec::Vec;

use crate::games::{Games, RunnableGame, GameState};
use crate::graphics::GraphicsResource;

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
}
impl RunnableGame for StartScreen {
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.time += time;
        if self.time < 25 {
            return GameState::Running(Games::Start);
        }

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
            return GameState::Start(game);
        }

        GameState::Running(Games::Start)
    }
    // fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) -> Option<()> {
    fn render<'g>(&self, graphics: &mut GraphicsResource<'g>) -> Option<()> {
        let gfx = match graphics {
            GraphicsResource::NotTiled(gfx) => gfx,
            _ => unimplemented!("WRONG MODE")
        };
        let oam = &mut gfx.unmanaged.iter();
        let loader = &mut gfx.sprite_loader;

        let x0 = 16u16;
        let dx = 16u16;
        let y0 = 48u16;
        let dy = 16u16;

        for (i, game) in self.games.iter().enumerate() {
            let sprite_tag = game.sprite.tag().sprite(0);
            let mut object = ObjectUnmanaged::new(loader.get_vram_sprite(sprite_tag));

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
