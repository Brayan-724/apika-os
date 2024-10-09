pub mod color;
pub mod primitive;
pub mod style;

pub use color::Color;
pub use style::{ApplyStyle, Styled};

pub const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;
pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;

static mut VIRTUAL_VGA_BUFFER: [u8; VGA_WIDTH * VGA_HEIGHT] = [0; VGA_WIDTH * VGA_HEIGHT];

pub fn put(offset: usize, value: Styled) {
    if offset >= (VGA_WIDTH * VGA_HEIGHT) as usize {
        return;
    }

    unsafe {
        *VGA_BUFFER.add(offset) = value.into_raw();
    }
}

pub fn put_pixel(x: isize, y: isize, color: Color) {
    if !is_inside(x, y) {
        return;
    }

    let offset = y * VGA_WIDTH as isize + x;
    let offset = offset as usize;

    let is_same = unsafe { VIRTUAL_VGA_BUFFER[offset] == color as u8 };

    if !is_same {
        unsafe { VIRTUAL_VGA_BUFFER[offset] = color as u8 };
        put(offset, 219u8.fg(color));
    }
}

pub fn is_inside(x: isize, y: isize) -> bool {
    (0 <= x && x < VGA_WIDTH as isize) && (0 <= y && y < VGA_HEIGHT as isize)
}
