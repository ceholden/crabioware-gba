use agb::display::object::{OamIterator, OamUnmanaged, ObjectUnmanaged, SpriteLoader};
use agb::display::{HEIGHT as GBA_HEIGHT, WIDTH as GBA_WIDTH};
use agb::input::{Button, ButtonController};
use agb::interrupt::VBlank;

use crate::games::{Game, GameState, Games};

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

    pub fn new_unpaused(game: Games) -> Self {
        Self::new(game, false)
    }

    pub fn new_paused(game: Games) -> Self {
        Self::new(game, true)
    }

    pub fn check(
        &mut self,
        unmanaged: &mut OamUnmanaged,
        sprite_loader: &mut SpriteLoader,
        buttons: &mut ButtonController,
        vblank: &VBlank,
    ) {
        loop {
            match self.is_paused(buttons) {
                true => {
                    self.render(unmanaged, sprite_loader);
                    vblank.wait_for_vblank();
                    buttons.update();
                }
                false => return,
            }
        }
    }

    fn is_paused(&mut self, buttons: &ButtonController) -> bool {
        self.paused = match buttons.is_just_pressed(Button::START) {
            true => !self.paused,
            false => self.paused,
        };
        self.paused
    }

    fn render(
        &mut self,
        unmanaged: &mut OamUnmanaged,
        sprite_loader: &mut SpriteLoader,
    ) -> Option<()> {
        let oam = &mut unmanaged.iter();

        if !self.paused {
            return None;
        }
        let sprite_tag = self.sprite.tag().sprite(0);
        let mut object = ObjectUnmanaged::new(sprite_loader.get_vram_sprite(sprite_tag));

        object
            .set_x(GBA_WIDTH as u16 / 2 - 16)
            .set_y(GBA_HEIGHT as u16 / 2 - 16)
            .show();
        oam.next()?.set(&object);
        Some(())
    }
}
