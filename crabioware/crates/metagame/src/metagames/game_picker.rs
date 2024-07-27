use agb::{input::ButtonController, interrupt::VBlank, rng::RandomNumberGenerator};
use alloc::vec;
use alloc::vec::Vec;

use crabioware_core::{games::{Game, GameDifficulty, GameLoader, GameState, Games}, screens::StartScreen};

use crate::metagame::{MetaGame, MetaGameState};

// TODO -- this is where we have up/down menu to select minigames
pub struct GamePicker {
    games: Vec<Games>,
}

impl GamePicker {
    pub fn new() -> GamePicker {
        GamePicker {
            games: vec![Games::Pong, Games::PacCrab, Games::Snake],
        }
    }

    fn run_game(
        &self,
        selected_game: &Games,
        difficulty: &GameDifficulty,
        gba: &mut agb::Gba,
        buttons: &mut ButtonController,
        rng: &mut RandomNumberGenerator,
        vblank: &VBlank,
        loader: &impl GameLoader,
    ) {
        let mut game = loader.load_game(&selected_game, &difficulty, rng);

        let mut game_state = GameState::Running(*selected_game);

        let (mut graphics, mut vram, mut unmanaged, mut sprite_loader) =
            game.renderer().create(gba);
        game.init_tiles(&mut graphics, &mut vram);

        let mut timer = 0;
        loop {
            buttons.update();

            game_state = game.advance(1i32, &buttons);
            game.render(&mut vram, &mut unmanaged, &mut sprite_loader);
            vblank.wait_for_vblank();

            match game_state {
                GameState::GameOver => {
                    game.clear(&mut vram);
                    drop(game);
                    return
                },
                _ => {}
            }
            timer += 1;
        }
    }
}

impl MetaGame for GamePicker {
    fn run(
        &self,
        gba: &mut agb::Gba,
        vblank: &agb::interrupt::VBlank,
        buttons: &mut ButtonController,
        loader: &impl GameLoader,
    ) -> MetaGameState {

        let mut rng = RandomNumberGenerator::new();
        let difficulty = GameDifficulty::HARD;

        loop {
            let selected_game = StartScreen::pick_game(gba, buttons, vblank);

            self.run_game(
                &selected_game,
                &difficulty,
                gba,
                buttons,
                &mut rng,
                &vblank,
                loader,
            );
        }
    }
}
