#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
extern crate alloc;

use agb::{input::ButtonController, interrupt::VBlank, println, rng::RandomNumberGenerator};
use crabioware_core::games::{Game, GameDifficulty, GameState, Games};
// use crabioware_core::screens::PauseScreen;

use crabioware::GameRunner;

fn run_game(
    selected_game: Games,
    difficulty: &GameDifficulty,
    gba: &mut agb::Gba,
    buttons: &mut ButtonController,
    rng: &mut RandomNumberGenerator,
    vblank: &VBlank,
) {
    let mut game = selected_game.load_game(&difficulty, rng);

    let (mut graphics, mut vram, mut unmanaged, mut sprite_loader) = game.renderer().create(gba);

    game.init_tiles(&mut graphics, &mut vram);
    let mut timer = 0;
    loop {
        buttons.update();

        game.advance(1i32, &buttons);
        game.render(&mut vram, &mut unmanaged, &mut sprite_loader);
        vblank.wait_for_vblank();

        timer += 1;
        if timer > 100 {
            game.clear(&mut vram);
            drop(game);
            return;
        }
    }
}

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    extern crate alloc;

    let vblank = VBlank::get();
    let mut buttons = ButtonController::new();
    let mut rng = RandomNumberGenerator::new();

    // FIXME: implement difficulty selector
    let difficulty = GameDifficulty::HARD;

    let mut selected_game = Games::PacCrab;
    loop {
        run_game(
            selected_game,
            &difficulty,
            &mut gba,
            &mut buttons,
            &mut rng,
            &vblank,
        );
        selected_game = match selected_game {
            Games::Snake => Games::PacCrab,
            Games::PacCrab => Games::Pong,
            Games::Pong => Games::Snake,
        };
    }
}
