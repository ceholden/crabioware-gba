use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
};

// Graphics assets
const SPRITES: &Graphics = include_aseprite!("assets/common.aseprite");
const GAMEOVER: &Tag = SPRITES.tags().get("gameover");
const VICTORY: &Tag = SPRITES.tags().get("victory");
const PAUSE: &Tag = SPRITES.tags().get("pause");
const PONG: &Tag = SPRITES.tags().get("pong");
const SNAKE: &Tag = SPRITES.tags().get("snake");

pub enum SpriteTag {
    GameOver,
    Victory,
    Pause,
    Snake,
    Pong,
}
impl SpriteTag {
    pub fn tag(&self) -> &Tag {
        match self {
            SpriteTag::GameOver => GAMEOVER,
            SpriteTag::Victory => VICTORY,
            SpriteTag::Pause => PAUSE,
            SpriteTag::Pong => PONG,
            SpriteTag::Snake => SNAKE,
        }
    }
}
