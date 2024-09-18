// Pong
use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
};

// Graphics assets
static SPRITES: &Graphics = include_aseprite!("assets/sprites.aseprite");
static PADDLE: &Tag = SPRITES.tags().get("paddle");
static BALL: &Tag = SPRITES.tags().get("crab");
static NUMBERS: &Tag = include_aseprite!("assets/numbers.aseprite")
    .tags()
    .get("white");

#[derive(Clone, Copy, Debug)]
pub enum SpriteTag {
    Paddle,
    Ball,
    Numbers,
}
impl SpriteTag {
    pub fn tag(&self) -> &Tag {
        match self {
            SpriteTag::Ball => BALL,
            SpriteTag::Paddle => PADDLE,
            SpriteTag::Numbers => NUMBERS,
        }
    }
}
