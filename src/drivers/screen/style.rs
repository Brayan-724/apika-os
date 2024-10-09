use super::color::Color;

#[derive(Default, Clone, Copy)]
pub enum Style {
    #[default]
    None,
    Fg(Color),
    Bg(Color),
    FgBg(Color, Color),
}

impl Style {
    pub fn apply(&self, c: char) -> Styled {
        Styled(c as u16 | self.to_u16())
    }

    pub fn apply_u8(&self, c: u8) -> Styled {
        Styled(c as u16 | self.to_u16())
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::None => 0,
            Style::Fg(fg) => *fg as u8,
            Style::Bg(bg) => (*bg as u8) << 4,
            Style::FgBg(fg, bg) => (*fg as u8) | (*bg as u8) << 4,
        }
    }

    pub fn to_u16(&self) -> u16 {
        (self.into_u8() as u16) << 8
    }

    pub fn into_u8(self) -> u8 {
        match self {
            Self::None => 0,
            Style::Fg(fg) => fg as u8,
            Style::Bg(bg) => (bg as u8) << 4,
            Style::FgBg(fg, bg) => (fg as u8) | (bg as u8) << 4,
        }
    }

    pub fn into_u16(self) -> u16 {
        (self.into_u8() as u16) << 8
    }
}

pub trait ApplyStyle {
    fn fg(&self, color: Color) -> Styled {
        self.style(&Style::Fg(color))
    }

    fn bg(&self, color: Color) -> Styled {
        self.style(&Style::Bg(color))
    }

    fn fg_bg(&self, fg: Color, bg: Color) -> Styled {
        self.style(&Style::FgBg(fg, bg))
    }

    fn style(&self, style: &Style) -> Styled;
}

impl ApplyStyle for u8 {
    fn style(&self, style: &Style) -> Styled {
        style.apply_u8(*self)
    }
}

impl ApplyStyle for &u8 {
    fn style(&self, style: &Style) -> Styled {
        style.apply_u8(**self)
    }
}

impl ApplyStyle for char {
    fn style(&self, style: &Style) -> Styled {
        style.apply(*self)
    }
}

impl ApplyStyle for &char {
    fn style(&self, style: &Style) -> Styled {
        style.apply(**self)
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Styled(u16);

impl Styled {
    pub fn get_raw(&self) -> &u16 {
        &self.0
    }

    pub fn into_raw(self) -> u16 {
        self.0
    }
}
