use agb::display::object::{OamIterator, ObjectUnmanaged, SpriteLoader};
use agb::display::{HEIGHT as GBA_HEIGHT, WIDTH as GBA_WIDTH};
use agb::input::{Button, ButtonController};

use crate::games::{Games, RunnableGame, GameState};

use super::graphics::SpriteTag;

pub struct PauseScreen {
    game: Games,
    paused: bool,
    sprite: SpriteTag,
}
impl PauseScreen {
    pub fn new(game: Games, paused: bool) -> Self {
        Self {
            game,
            paused,
            sprite: SpriteTag::Pause,
        }
    }

    pub fn unpaused(game: Games) -> Self {
        Self::new(game, false)
    }

    pub fn paused(game: Games) -> Self {
        Self::new(game, true)
    }
}
impl RunnableGame for PauseScreen {
    fn advance(&mut self, _: i32, buttons: &ButtonController) -> GameState {
        self.paused = match buttons.is_just_pressed(Button::START) {
            true => !self.paused,
            false => self.paused,
        };
        match self.paused {
            true => GameState::Pause(self.game),
            false => GameState::Running(self.game),
        }
    }
    fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) -> Option<()> {
        if !self.paused {
            return None;
        }
        let sprite_tag = self.sprite.tag().sprite(0);
        let mut object = ObjectUnmanaged::new(loader.get_vram_sprite(sprite_tag));

        object
            .set_x(GBA_WIDTH as u16 / 2 - 16)
            .set_y(GBA_HEIGHT as u16 / 2 - 16)
            .show();
        oam.next()?.set(&object);
        Some(())
    }
}
