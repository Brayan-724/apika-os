pub mod line;
pub mod pos;
pub mod rect;

pub trait DrawPrimitive {
    fn draw(&self);
}
