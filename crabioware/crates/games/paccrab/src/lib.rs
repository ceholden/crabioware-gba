#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
extern crate alloc;

mod graphics;
mod levels;
mod paccrab;
pub use paccrab::PacCrabGame;

#[cfg(test)]
#[agb::entry]
fn agb_test_main(gba: agb::Gba) -> ! {
    loop {
        // full implementation provided by the #[entry]
        agb::syscall::halt();
    }
}
