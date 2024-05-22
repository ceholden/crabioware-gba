#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
extern crate alloc;


// TODO
mod game_picker;
pub mod graphics;
mod metagame;
mod resources;

pub use metagame::{MetaGame, MetaGameState, MetaGameType};

#[cfg(test)]
#[agb::entry]
fn agb_test_main(gba: agb::Gba) -> ! {
    loop {
        // full implementation provided by the #[entry]
        agb::syscall::halt();
    }
}
