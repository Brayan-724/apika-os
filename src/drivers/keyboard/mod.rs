use crate::asm::in_byte;
use crate::asm::interrupts::{InterruptsIbmIrq, InterruptsPorts};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyboardState {
    Pressed(char),
    Released(char),
    Nothing,
    Event(KeyboardEvents),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyboardEvents {
    Backspace = 8,
    Enter = 13,
}

pub fn get_press() -> KeyboardState {
    KeyboardState::from(InterruptsPorts::Data.read_byte())
}

pub fn wait_press() -> KeyboardState {
    while !InterruptsIbmIrq::KeyboardInterrupt.read_bool() {}

    get_press()
}

impl From<u8> for KeyboardState {
    fn from(value: u8) -> Self {
        match value {
            0x1E => KeyboardState::Pressed('a'),
            0x9E => KeyboardState::Released('a'),

            0x30 => KeyboardState::Pressed('b'),
            0xB0 => KeyboardState::Released('b'),

            0x2E => KeyboardState::Pressed('c'),
            0xAE => KeyboardState::Released('c'),

            0x20 => KeyboardState::Pressed('d'),
            0xA0 => KeyboardState::Released('d'),

            0x12 => KeyboardState::Pressed('e'),
            0x92 => KeyboardState::Released('e'),

            0x21 => KeyboardState::Pressed('f'),
            0xA1 => KeyboardState::Released('f'),

            0x22 => KeyboardState::Pressed('g'),
            0xA2 => KeyboardState::Released('g'),

            0x23 => KeyboardState::Pressed('h'),
            0xA3 => KeyboardState::Released('h'),

            0x17 => KeyboardState::Pressed('i'),
            0x97 => KeyboardState::Released('i'),

            0x24 => KeyboardState::Pressed('j'),
            0xA4 => KeyboardState::Released('j'),

            0x25 => KeyboardState::Pressed('k'),
            0xA5 => KeyboardState::Released('k'),

            0x26 => KeyboardState::Pressed('l'),
            0xA6 => KeyboardState::Released('l'),

            0x32 => KeyboardState::Pressed('m'),
            0x3A => KeyboardState::Released('m'),

            0x31 => KeyboardState::Pressed('n'),
            0xB1 => KeyboardState::Released('n'),

            0x18 => KeyboardState::Pressed('o'),
            0x98 => KeyboardState::Released('o'),

            0x19 => KeyboardState::Pressed('p'),
            0x99 => KeyboardState::Released('p'),

            0x10 => KeyboardState::Pressed('q'),
            0x90 => KeyboardState::Released('q'),

            0x13 => KeyboardState::Pressed('r'),
            0x93 => KeyboardState::Released('r'),

            0x1F => KeyboardState::Pressed('s'),
            0x9F => KeyboardState::Released('s'),

            0x14 => KeyboardState::Pressed('t'),
            0x94 => KeyboardState::Released('t'),

            0x16 => KeyboardState::Pressed('u'),
            0x96 => KeyboardState::Released('u'),

            0x2F => KeyboardState::Pressed('v'),
            0xAF => KeyboardState::Released('v'),

            0x11 => KeyboardState::Pressed('w'),
            0x91 => KeyboardState::Released('w'),

            0x2D => KeyboardState::Pressed('x'),
            0xAD => KeyboardState::Released('x'),

            0x15 => KeyboardState::Pressed('y'),
            0x95 => KeyboardState::Released('y'),

            0x02 => KeyboardState::Pressed('1'),  // 1 is pressed
            0x82 => KeyboardState::Released('1'), // 1 is released

            0x2C => KeyboardState::Pressed('z'),
            0xAC => KeyboardState::Released('z'),

            0x03 => KeyboardState::Pressed('2'),
            0x83 => KeyboardState::Released('2'),

            0x04 => KeyboardState::Pressed('3'),
            0x84 => KeyboardState::Released('3'),

            0x05 => KeyboardState::Pressed('4'),
            0x85 => KeyboardState::Released('4'),

            0x06 => KeyboardState::Pressed('5'),
            0x86 => KeyboardState::Released('5'),

            0x07 => KeyboardState::Pressed('6'),
            0x87 => KeyboardState::Released('6'),

            0x08 => KeyboardState::Pressed('7'),
            0x88 => KeyboardState::Released('7'),

            0x09 => KeyboardState::Pressed('8'),
            0x89 => KeyboardState::Released('8'),

            0x0A => KeyboardState::Pressed('9'),
            0x8A => KeyboardState::Released('9'),

            0x0B => KeyboardState::Pressed('0'),
            0x8B => KeyboardState::Released('0'),

            0x39 => KeyboardState::Pressed(' '),
            0xB9 => KeyboardState::Released(' '),

            0x0E => KeyboardState::Event(KeyboardEvents::Backspace),
            0x1C => KeyboardState::Event(KeyboardEvents::Enter),

            _ => KeyboardState::Nothing,
        }
    }
}
