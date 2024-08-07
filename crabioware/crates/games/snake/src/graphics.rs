// Pong
use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
};

// Graphics assets
const SPRITES: &Graphics = include_aseprite!("assets/sprites.aseprite");
const SNAKE: &Tag = SPRITES.tags().get("green");
const BERRY: &Tag = SPRITES.tags().get("red");
// FIXME: more nutritious purple berries
const NUMBERS: &Tag = include_aseprite!("assets/numbers.aseprite")
    .tags()
    .get("white");

#[derive(Clone, Copy)]
pub enum SpriteTag {
    Snake,
    Berry,
    Numbers,
}
impl SpriteTag {
    pub fn tag(&self) -> &Tag {
        match self {
            SpriteTag::Snake => SNAKE,
            SpriteTag::Berry => BERRY,
            SpriteTag::Numbers => NUMBERS,
        }
    }
}
