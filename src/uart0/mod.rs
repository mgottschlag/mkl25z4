#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Baud Rate Register High"]
    pub bdh: BDH,
    #[doc = "0x01 - UART Baud Rate Register Low"]
    pub bdl: BDL,
    #[doc = "0x02 - UART Control Register 1"]
    pub c1: C1,
    #[doc = "0x03 - UART Control Register 2"]
    pub c2: C2,
    #[doc = "0x04 - UART Status Register 1"]
    pub s1: S1,
    #[doc = "0x05 - UART Status Register 2"]
    pub s2: S2,
    #[doc = "0x06 - UART Control Register 3"]
    pub c3: C3,
    #[doc = "0x07 - UART Data Register"]
    pub d: D,
    #[doc = "0x08 - UART Match Address Registers 1"]
    pub ma1: MA1,
    #[doc = "0x09 - UART Match Address Registers 2"]
    pub ma2: MA2,
    #[doc = "0x0a - UART Control Register 4"]
    pub c4: C4,
    #[doc = "0x0b - UART Control Register 5"]
    pub c5: C5,
}
#[doc = "UART Baud Rate Register High"]
pub struct BDH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Baud Rate Register High"]
pub mod bdh;
#[doc = "UART Baud Rate Register Low"]
pub struct BDL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Baud Rate Register Low"]
pub mod bdl;
#[doc = "UART Control Register 1"]
pub struct C1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Control Register 1"]
pub mod c1;
#[doc = "UART Control Register 2"]
pub struct C2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Control Register 2"]
pub mod c2;
#[doc = "UART Status Register 1"]
pub struct S1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Status Register 1"]
pub mod s1;
#[doc = "UART Status Register 2"]
pub struct S2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Status Register 2"]
pub mod s2;
#[doc = "UART Control Register 3"]
pub struct C3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Control Register 3"]
pub mod c3;
#[doc = "UART Data Register"]
pub struct D {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Data Register"]
pub mod d;
#[doc = "UART Match Address Registers 1"]
pub struct MA1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Match Address Registers 1"]
pub mod ma1;
#[doc = "UART Match Address Registers 2"]
pub struct MA2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Match Address Registers 2"]
pub mod ma2;
#[doc = "UART Control Register 4"]
pub struct C4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Control Register 4"]
pub mod c4;
#[doc = "UART Control Register 5"]
pub struct C5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Control Register 5"]
pub mod c5;
