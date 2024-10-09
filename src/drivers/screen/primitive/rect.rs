use crate::drivers::screen::{self, Color};

use super::pos::Pos;

pub fn draw_rect(pos: Pos, size: Pos, color: Color) {
    for y in pos.1..=pos.1 + size.1 {
        for x in pos.0..=pos.0 + size.0 {
            screen::put_pixel(x, y, color);
        }
    }
}
