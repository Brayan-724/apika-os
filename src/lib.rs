#![no_std]
#![no_main]
#![feature(rustc_private)]

pub mod asm;
pub mod drivers;
pub mod games;

use core::panic::PanicInfo;

use crate::drivers::screen::{self, ApplyStyle, Color};
use crate::games::kernel_dash;

#[no_mangle]
extern "C" fn _start() -> ! {
    kernel_dash::run();
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    screen::put(0, 'P'.fg(Color::default()));
    screen::put(1, 'a'.fg(Color::default()));
    screen::put(2, 'n'.fg(Color::default()));
    screen::put(3, 'i'.fg(Color::default()));
    screen::put(4, 'c'.fg(Color::default()));
    loop {}
}
