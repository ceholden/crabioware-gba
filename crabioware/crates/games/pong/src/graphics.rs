// Pong
use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
};

// Graphics assets
const SPRITES: &Graphics = include_aseprite!("assets/sprites.aseprite");
const PADDLE: &Tag = SPRITES.tags().get("paddle");
const BALL: &Tag = SPRITES.tags().get("crab");
const NUMBERS: &Tag = include_aseprite!("assets/numbers.aseprite")
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
