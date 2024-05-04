use agb::Gba;


pub enum MetaGameState {
    START,
    RUNNING,
}


pub trait MetaGame {
    fn run(&self, gba: &mut Gba, state: &MetaGameState) -> MetaGameState;
}
