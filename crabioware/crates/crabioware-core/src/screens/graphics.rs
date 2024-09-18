use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
};

// Graphics assets
static SPRITES: &Graphics = include_aseprite!("assets/common.aseprite");
static GAMEOVER: &Tag = SPRITES.tags().get("gameover");
static VICTORY: &Tag = SPRITES.tags().get("victory");
static PAUSE: &Tag = SPRITES.tags().get("pause");
static PONG: &Tag = SPRITES.tags().get("pong");
static SNAKE: &Tag = SPRITES.tags().get("snake");
static PACCRAB: &Tag = SPRITES.tags().get("paccrab");

pub enum SpriteTag {
    GameOver,
    Victory,
    Pause,
    Snake,
    Pong,
    PacCrab,
}
impl SpriteTag {
    pub fn tag(&self) -> &Tag {
        match self {
            SpriteTag::GameOver => GAMEOVER,
            SpriteTag::Victory => VICTORY,
            SpriteTag::Pause => PAUSE,
            SpriteTag::Pong => PONG,
            SpriteTag::Snake => SNAKE,
            SpriteTag::PacCrab => PACCRAB,
        }
    }
}
