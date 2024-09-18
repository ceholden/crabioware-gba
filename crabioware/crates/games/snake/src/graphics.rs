// Pong
use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
};

// Graphics assets
static SPRITES: &Graphics = include_aseprite!("assets/sprites.aseprite");
static SNAKE: &Tag = SPRITES.tags().get("green");
static BERRY: &Tag = SPRITES.tags().get("red");
// FIXME: more nutritious purple berries
static NUMBERS: &Tag = include_aseprite!("assets/numbers.aseprite")
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
