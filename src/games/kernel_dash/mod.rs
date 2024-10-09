pub(super) mod map;
pub(super) mod player;

use map::main;
use player::Player;

use crate::asm::in_byte;
use crate::drivers::clock::IntervalClock;
use crate::drivers::keyboard;
use crate::drivers::screen::primitive::pos::Pos;
use crate::drivers::screen::primitive::rect::draw_rect;
use crate::drivers::screen::Color;

pub fn run() -> ! {
    let mut player = Player::new();

    let mut update_clock = IntervalClock::new(400_000);

    let mut advace_offset = 0;
    let mut advance_clock = IntervalClock::new(2);

    draw_rect(Pos(0, 23), Pos(79, 2), Color::LightCyan);

    let map_base = 22 - main::MAP_HEIGHT as isize;
    for (y, row) in main::MAP.into_iter().enumerate() {
        for (cell_x, cell) in row.into_iter().enumerate() {
            let cell_x = cell_x * 8;
            for i in 0..8 {
                if cell & (1 << (7 - i)) != 0 {
                    let x = cell_x + i + 10;

                    draw_rect(
                        Pos(x as isize * 3, y as isize + map_base),
                        Pos(2, 1),
                        Color::Purple,
                    );
                }
            }
        }
    }

    loop {
        while (in_byte(0x64) & 0x8) == 1 {}

        if update_clock.update() {
            player.clear();
            // draw_rect(Pos(80 - advace_offset, 10), Pos(2, 1), Color::Black);

            let key = keyboard::get_press();

            match key {
                keyboard::KeyboardState::Pressed(' ') => player.jump(),
                _ => {}
            }

            if advance_clock.update() {
                // advace_offset += 1;
            }

            player.update();
            player.draw();
            draw_rect(Pos(80 - advace_offset, 10), Pos(2, 1), Color::Red);
        }
    }
}
