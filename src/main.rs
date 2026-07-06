//! An example of using blending to make an object darken. Only the objects marked with
//! GraphicsMode::AlphaBlending will show as darkened.
#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{
    display::{
        Priority,
        object::{GraphicsMode, Object},
        tiled::{RegularBackground, RegularBackgroundSize},
    },
    fixnum::{Num, num},
    include_aseprite, include_background_gfx,
};

include_aseprite!(mod sprites, "gfx/crab.aseprite");
include_background_gfx!(mod background, BEACH => deduplicate "gfx/beach-background.aseprite");

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();
    gfx.set_background_palettes(background::PALETTES);

    let mut bg = RegularBackground::new(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        background::BEACH.tiles.format(),
    );
    bg.fill_with(&background::BEACH);

    let mut darken_amount = num!(0);

    loop {
        let mut frame = gfx.frame();
        bg.show(&mut frame);

        Object::new(sprites::IDLE.sprite(0))
            .set_graphics_mode(GraphicsMode::Normal)
            .set_pos((100, 100))
            .show(&mut frame);

        Object::new(sprites::IDLE.sprite(0))
            .set_graphics_mode(GraphicsMode::AlphaBlending)
            .set_pos((150, 100))
            .show(&mut frame);

        frame
            .blend()
            .darken(darken_amount)
            .enable_object();
        darken_amount = (darken_amount + Num::from_raw(1)) % num!(1);

        frame.commit();
    }
}