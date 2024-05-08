#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::display::{
    palette16::Palette16,
    tiled::{RegularBackgroundSize, TileFormat, TiledMap},
    Priority,
};

use crabioware_metagame::graphics::test;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {

    test(&mut gba);

//    let (gfx, mut vram) = gba.display.video.tiled0();
//
//    vram.set_background_palettes(&[Palette16::new([
//        0xff00, 0x0ff0, 0x00ff, 0xf00f, 0xf0f0, 0x0f0f, 0xaaaa, 0x5555, 0x0000, 0x0000, 0x0000,
//        0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
//    ])]);
//
//    let mut bg = gfx.background(
//        Priority::P0,
//        RegularBackgroundSize::Background32x32,
//        TileFormat::FourBpp,
//    );
//
//    for y in 0..20u32 {
//        for x in 0..30u32 {
//            let dynamic_tile = vram.new_dynamic_tile();
//
//            for (i, bit) in dynamic_tile.tile_data.iter_mut().enumerate() {
//                let i = i as u32;
//                let mut value = 0;
//
//                for j in 0..4 {
//                    value |= (value << 8) | ((x + i) % 8) | ((y + j) % 8) << 4;
//                }
//
//                *bit = value;
//            }
//
//            bg.set_tile(
//                &mut vram,
//                (x as u16, y as u16),
//                &dynamic_tile.tile_set(),
//                dynamic_tile.tile_setting(),
//            );
//
//            vram.remove_dynamic_tile(dynamic_tile);
//        }
//    }
//
//    bg.commit(&mut vram);
//    bg.set_visible(true);


}


