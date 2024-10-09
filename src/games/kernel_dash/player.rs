use crate::drivers::screen::primitive::pos::Pos;
use crate::drivers::screen::primitive::rect::draw_rect;
use crate::drivers::screen::Color;

#[derive(Default)]
pub struct Player {
    pub x: isize,
    pub y: isize,

    pub start_jumping: isize,
    pub jumping: bool,
    pub falling: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 40,
            y: 21,

            ..Default::default()
        }
    }

    pub fn jump(&mut self) {
        if !self.jumping && !self.falling {
            self.jumping = true;
            self.start_jumping = self.y;
        }
    }

    pub fn clear(&self) {
        draw_rect(Pos(self.x, self.y), Pos(2, 1), Color::Black);
    }

    pub fn update(&mut self) {
        if self.jumping {
            self.y -= 1;

            if self.y <= self.start_jumping - 5 {
                self.jumping = false;
                self.falling = true;
            }
        } else if self.falling {
            self.y += 1;

            if self.y >= 21 {
                self.y = 21;
                self.jumping = false;
                self.falling = false;
            }
        }
    }

    pub fn draw(&self) {
        draw_rect(Pos(self.x, self.y), Pos(2, 1), Color::Blue);
    }
}
