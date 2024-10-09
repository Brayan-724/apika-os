use super::in_byte;

/// https://wiki.osdev.org/Interrupts#Ports
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum InterruptsPorts {
    MasterPic0 = 0x20,
    MasterPic1 = 0x21,
    SlavePic0 = 0xA0,
    SlavePic1 = 0xA1,
    Data = 0x60,
    Command = 0x64,
}

/// https://wiki.osdev.org/Interrupts#Standard_ISA_IRQs
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum InterruptsIbmIrq {
    ProgrammableInterruptTimerInterrupt,
    KeyboardInterrupt,
    /// used internally by the two PICs. never raised
    Cascade,
    COM2,
    COM1,
    LPT2,
    FloppyDisk,
    /// Unreliable "spurious" interrupt (usually)
    LPT1,
    CmosRealTimeClock,
    /// Free for peripherals
    LegacySCSI,
    /// Free for peripherals
    SCSI,
    /// Free for peripherals
    NIC,
    Ps2Mouse,
    FPU,
    PrimaryAtaHardDisk,
    SecondaryAtaHardDisk,
}

impl InterruptsPorts {
    pub fn read_byte(&self) -> u8 {
        in_byte(*self as u16)
    }
}

impl InterruptsIbmIrq {
    pub fn read_bool(&self) -> bool {
        InterruptsPorts::Command.read_byte() & *self as u8 == 1
    }
}
