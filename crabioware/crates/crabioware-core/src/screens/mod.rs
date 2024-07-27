// mod gameover;
mod graphics;
mod pause;
mod start;
// mod victory;
// pub use gameover::GameOverScreen;
pub use pause::PauseScreen;
pub use start::StartScreen;
// pub use victory::VictoryScreen;

pub enum Screens {
    GameOver,
    Pause,
    Start,
    Victory,
}
