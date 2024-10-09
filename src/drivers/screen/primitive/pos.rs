use core::ops;

use crate::drivers::screen::{VGA_HEIGHT, VGA_WIDTH};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos(pub isize, pub isize);

impl Pos {
    pub fn to_screen(&self) -> Option<(usize, usize)> {
        if self.is_inside_screen() {
            Some((self.0 as usize, self.1 as usize))
        } else {
            None
        }
    }

    pub fn to_offset_unchecked(&self) -> isize {
        self.1 * VGA_WIDTH as isize + self.0
    }

    pub fn is_inside_screen(&self) -> bool {
        (0 <= self.0 && self.0 <= VGA_WIDTH as isize)
            && (0 <= self.1 && self.1 <= VGA_HEIGHT as isize)
    }
}

impl ops::Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos(rhs.0 - self.0, rhs.1 - self.1)
    }
}

impl ops::Sub for &Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos(rhs.0 - self.0, rhs.1 - self.1)
    }
}
