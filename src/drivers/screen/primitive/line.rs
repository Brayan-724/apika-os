use crate::drivers::screen::{self, Color};

use super::pos::Pos;

pub fn draw_line(pos_a: Pos, pos_b: Pos, color: screen::Color) {
    if (pos_b.0 - pos_a.0).abs() > (pos_b.1 - pos_a.1).abs() {
        draw_line_h(pos_a, pos_b, color);
    } else {
        draw_line_v(pos_a, pos_b, color);
    }
}

fn draw_line_h(pos_a: Pos, pos_b: Pos, color: screen::Color) {
    // Both points are outside screen
    if !pos_a.is_inside_screen() && !pos_b.is_inside_screen() {
        return;
    }

    let (pos_a, pos_b) = if pos_a.0 > pos_b.0 {
        (pos_b, pos_a)
    } else {
        (pos_a, pos_b)
    };

    // Always positive
    let dx = pos_b.0 - pos_a.0;
    // Maybe negative
    let dy = pos_b.1 - pos_a.1;

    let dir = dy.is_negative().then_some(-1).unwrap_or(1);
    let dy = dy.abs();

    if dx != 0 {
        let mut y = pos_a.1;
        let mut p = 2 * dy - dx;

        for i in 0..dx + 1 {
            screen::put_pixel(pos_a.0 + i, y, color);

            if p >= 0 {
                y += dir;
                p = p - 2 * dx;
            }

            p = p + 2 * dy;
        }
    }
}

fn draw_line_v(pos_a: Pos, pos_b: Pos, color: screen::Color) {
    // Both points are outside screen
    if !pos_a.is_inside_screen() && !pos_b.is_inside_screen() {
        return;
    }

    let (pos_a, pos_b) = if pos_a.1 > pos_b.1 {
        (pos_b, pos_a)
    } else {
        (pos_a, pos_b)
    };

    // Maybe positive
    let dx = pos_b.0 - pos_a.0;
    // Always negative
    let dy = pos_b.1 - pos_a.1;

    let dir = dx.is_negative().then_some(-1).unwrap_or(1);
    let dx = dx.abs();

    if dy != 0 {
        let mut x = pos_a.0;
        let mut p = 2 * dx - dx;

        for i in 0..dy + 1 {
            screen::put_pixel(x, pos_a.1 + i, color);

            if p >= 0 {
                x += dir;
                p = p - 2 * dy;
            }

            p = p + 2 * dx;
        }
    }
}
