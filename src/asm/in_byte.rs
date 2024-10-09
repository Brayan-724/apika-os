use core::arch::asm;

pub fn in_byte(port: u16) -> u8 {
    let result: u8;
    unsafe {
        asm!(
            "in al, dx",
            inout("dx") port => _,
            lateout("al") result
        );
    }
    result
}
