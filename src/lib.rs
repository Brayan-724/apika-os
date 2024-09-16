#![no_std]
#![no_main]
#![feature(rustc_private)]

use core::panic::PanicInfo;

const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;
const WIDTH: usize = 80;

#[no_mangle]
extern "C" fn _start() -> ! {
    let a: (usize, usize) = (0, 0);
    let b: (usize, usize) = (10, 20);

    // put_pixel(a.0, a.1);
    // put_pixel(b.0, b.1);

    // for i in 1..(80) {
    //     put_char(i, 0, None);
    // }

    for i in 0..25 {
        put_pixel(i, i);
        put_pixel(i + 25, i);
        put_pixel(i + 25 * 2, i);
        put_pixel(i + 25 * 3, i);
    }

    loop {}
}

fn put_pixel(x: usize, y: usize) {
    let offset = y * WIDTH + x;

    put_char(offset, b' ', None);
    // put_char(offset + 1, b'0' + (y as u8 / 10), None);
    // put_char(offset + 2, b'0' + (y as u8 % 10), None);
}

fn put_char(offset: usize, char: u8, color: Option<u8>) {
    if offset >= 79 * 24 {
        return;
    }

    unsafe {
        *VGA_BUFFER.add(offset) = vga_entry(char, color.unwrap_or(23));
    }
}

pub fn vga_entry_color(fg: u8, bg: u8) -> u8 {
    return (fg) | (bg) << 4;
}

pub fn vga_entry(c: u8, color: u8) -> u16 {
    return (c as u16) | (color as u16) << 8;
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    put_char(0, b'P', None);
    put_char(1, b'a', None);
    put_char(2, b'n', None);
    put_char(3, b'i', None);
    put_char(4, b'c', None);
    loop {}
}
