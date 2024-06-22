#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
extern crate alloc;

use agb::{input::ButtonController, interrupt::VBlank};

use crabioware::Registry;

use crabioware_metagame::{GamePicker, MetaGame};

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    extern crate alloc;

    let mut vblank = VBlank::get();
    let mut buttons = ButtonController::new();

    let metagame = GamePicker::new();
    let loader = Registry::new();

    loop {
        // FIXME: allow changing metagame
        metagame.run(&mut gba, &mut vblank, &mut buttons, &loader);
    }
}
