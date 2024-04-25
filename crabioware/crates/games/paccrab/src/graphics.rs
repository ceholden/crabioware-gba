use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
};

// Graphics assets
const SPRITES: &Graphics = include_aseprite!("assets/sprites.aseprite");
const CRAB: &Tag = SPRITES.tags().get("crab");
const GHOST_PINK: &Tag = SPRITES.tags().get("pink");
const GHOST_YELLOW: &Tag = SPRITES.tags().get("yellow");
const GHOST_BLUE: &Tag = SPRITES.tags().get("blue");
const BERRY: &Tag = SPRITES.tags().get("berry");

pub enum SpriteTag {
    Crab,
    GhostPink,
    GhostYellow,
    GhostBlue,
    Berry,
}
impl SpriteTag {
    pub fn tag(&self) -> &Tag {
        match self {
            SpriteTag::Crab => CRAB,
            SpriteTag::GhostPink => GHOST_PINK,
            SpriteTag::GhostYellow => GHOST_YELLOW,
            SpriteTag::GhostBlue => GHOST_BLUE,
            SpriteTag::Berry => BERRY,
        }
    }
}
