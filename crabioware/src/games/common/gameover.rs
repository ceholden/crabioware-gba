use agb::display::object::{OamIterator, ObjectUnmanaged, SpriteLoader};
use agb::display::{HEIGHT as GBA_HEIGHT, WIDTH as GBA_WIDTH};
use agb::input::{Button, ButtonController};
use agb::println;

use crate::games::{Game, GameState, Games};

use super::graphics::SpriteTag;

pub struct GameOverScreen {
    time: i32,
}
impl GameOverScreen {
    pub fn new() -> Self {
        Self { time: 0i32 }
    }
}
impl Game for GameOverScreen {
    fn advance(&mut self, time: i32, buttons: &ButtonController) -> GameState {
        self.time += time;

        if buttons.is_just_pressed(Button::A) {
            println!("Gameover acknowledged");
            return GameState::Start(Games::Start);
        }
        GameState::Running(Games::GameOver)
    }
    fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) -> Option<()> {
        let sprite_tag = SpriteTag::GameOver.tag().sprite(0);
        let mut object = ObjectUnmanaged::new(loader.get_vram_sprite(sprite_tag));

        object
            .set_x(GBA_WIDTH as u16 / 2 - 16)
            .set_y(GBA_HEIGHT as u16 / 2 - 16)
            .show();
        oam.next()?.set(&object);

        Some(())
    }
}
