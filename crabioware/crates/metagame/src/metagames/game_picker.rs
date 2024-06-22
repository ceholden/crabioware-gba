use agb::{input::ButtonController, interrupt::VBlank, rng::RandomNumberGenerator};
use alloc::vec;
use alloc::vec::Vec;

use crabioware_core::games::{Game, GameDifficulty, GameLoader, Games};

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

        let (mut graphics, mut vram, mut unmanaged, mut sprite_loader) =
            game.renderer().create(gba);

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
}

impl MetaGame for GamePicker {
    fn run(
        &self,
        gba: &mut agb::Gba,
        vblank: &agb::interrupt::VBlank,
        buttons: &mut ButtonController,
        loader: &impl GameLoader,
    ) -> MetaGameState {
        let mut selected_game = Games::PacCrab;

        let mut rng = RandomNumberGenerator::new();
        let difficulty = GameDifficulty::HARD;

        loop {
            self.run_game(
                &selected_game,
                &difficulty,
                gba,
                buttons,
                &mut rng,
                &vblank,
                loader,
            );
            selected_game = match selected_game {
                Games::Snake => Games::PacCrab,
                Games::PacCrab => Games::Pong,
                Games::Pong => Games::Snake,
            };
        }
    }
}
