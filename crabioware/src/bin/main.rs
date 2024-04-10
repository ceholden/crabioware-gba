#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;
use alloc::boxed::Box;

use agb::println;
use crabioware::games::{Game, GameDifficulty, GameState, Games, PauseScreen};

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    extern crate alloc;

    // FIXME: implement difficulty selector
    let difficulty = GameDifficulty::HARD;

    // FIXME: background! and which type?
    let (_background, _) = gba.display.video.tiled0();
    let (mut unmanaged, mut sprite_loader) = gba.display.object.get_unmanaged();

    let vblank = agb::interrupt::VBlank::get();

    let mut buttons = agb::input::ButtonController::new();
    let mut rng = agb::rng::RandomNumberGenerator::new();

    let mut selected_game: Box<dyn Game> =
        Games::Start.new(&difficulty, &mut sprite_loader, &mut rng);
    let mut pause = PauseScreen::unpaused(Games::Start);

    let mut game_state = GameState::Start(Games::Start);
    loop {
        rng.gen(); // helps feel more random by introducing time

        buttons.update();

        match game_state {
            GameState::Start(game) => {
                println!("Starting game = {:?}", game);
                selected_game = game.new(&difficulty, &mut sprite_loader, &mut rng);
                game_state = selected_game.advance(1i32, &buttons);
            }
            GameState::GameOver => {
                println!("GAME OVER");
                selected_game = Games::GameOver.new(&difficulty, &mut sprite_loader, &mut rng);
                game_state = selected_game.advance(1i32, &buttons);
            }
            GameState::Running(_) => {
                game_state = match pause.advance(1i32, &buttons) {
                    GameState::Pause(game) => {
                        pause = PauseScreen::paused(game);
                        GameState::Pause(game)
                    }
                    _ => selected_game.advance(1i32, &buttons),
                };
            }
            GameState::Pause(_) => {
                game_state = pause.advance(1i32, &buttons);
            }
            GameState::Win(_) => {
                // If we win, go back to start for now
                selected_game = Games::Victory.new(&difficulty, &mut sprite_loader, &mut rng);
                game_state = selected_game.advance(1i32, &buttons);
            }
        }
        vblank.wait_for_vblank();
        let mut oam_frame = &mut unmanaged.iter();
        selected_game.render(&mut sprite_loader, &mut oam_frame);
        pause.render(&mut sprite_loader, &mut oam_frame);
    }
}
