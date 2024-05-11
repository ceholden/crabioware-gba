#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{
    display::{
        palette16::Palette16,
        tiled::{RegularBackgroundSize, TileFormat, TiledMap},
        Priority,
    },
    println,
};

use crabioware_metagame::{graphics::{GamePicker, Games}, MetaGame};
use crabioware_metagame::{MetaGameState, MetaGameType};

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let vblank = agb::interrupt::VBlank::get();

    let picker = GamePicker::new();
    let mut state = MetaGameState::START(MetaGameType::PICKER);

    let mut idx = 0;
    loop {
        println!("MAIN LOOP ITERATION {idx}");
        state = picker.run(&mut gba, &vblank);
        idx += 1;
    }
}
