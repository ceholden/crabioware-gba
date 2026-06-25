use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
};

// Graphics assets
static SPRITES: &Graphics = include_aseprite!("assets/sprites.aseprite");
static CRAB: &Tag = SPRITES.tags().get("crab");
static SUPERCRAB: &Tag = SPRITES.tags().get("supercrab");
static GHOST_PINK: &Tag = SPRITES.tags().get("pink");
static GHOST_YELLOW: &Tag = SPRITES.tags().get("yellow");
static GHOST_BLUE: &Tag = SPRITES.tags().get("blue");
static GHOST_ORANGE: &Tag = SPRITES.tags().get("orange");
static GHOST_RED: &Tag = SPRITES.tags().get("red");
static GHOST_SCARED: &Tag = SPRITES.tags().get("scared");
static BERRY: &Tag = SPRITES.tags().get("berry");
static WARP: &Tag = SPRITES.tags().get("warp");

#[derive(Clone, Copy, Debug)]
pub enum SpriteTag {
    Crab,
    SuperCrab,
    GhostPink,
    GhostYellow,
    GhostBlue,
    GhostOrange,
    GhostRed,
    GhostScared,
    Berry,
    Warp,
}
impl SpriteTag {
    pub fn tag(&self) -> &Tag {
        match self {
            SpriteTag::Crab => CRAB,
            SpriteTag::SuperCrab => SUPERCRAB,
            SpriteTag::GhostPink => GHOST_PINK,
            SpriteTag::GhostYellow => GHOST_YELLOW,
            SpriteTag::GhostBlue => GHOST_BLUE,
            SpriteTag::GhostOrange => GHOST_ORANGE,
            SpriteTag::GhostRed => GHOST_RED,
            SpriteTag::GhostScared => GHOST_SCARED,
            SpriteTag::Berry => BERRY,
            SpriteTag::Warp => WARP,
        }
    }
}
