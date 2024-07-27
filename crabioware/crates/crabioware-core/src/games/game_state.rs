use super::games::Games;

#[derive(Debug)]
pub enum GameState {
    // Start screen / game selection
    Start(Games),
    // Common pause
    Pause(Games),
    // Common death screen
    GameOver,
    // Continue running same game
    Running(Games),
    // Win condition
    Win(Games),
}
