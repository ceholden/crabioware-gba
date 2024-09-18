use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
};

// Graphics assets
static SPRITES: &Graphics = include_aseprite!("assets/sprites.aseprite");
static CRAB: &Tag = SPRITES.tags().get("crab");
static GHOST_PINK: &Tag = SPRITES.tags().get("pink");
static GHOST_YELLOW: &Tag = SPRITES.tags().get("yellow");
static GHOST_BLUE: &Tag = SPRITES.tags().get("blue");
static BERRY: &Tag = SPRITES.tags().get("berry");

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
