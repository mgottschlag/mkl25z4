#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC Data Low Register"]
    pub dat0l: DATL,
    #[doc = "0x01 - DAC Data High Register"]
    pub dat0h: DATH,
    #[doc = "0x02 - DAC Data Low Register"]
    pub dat1l: DATL,
    #[doc = "0x03 - DAC Data High Register"]
    pub dat1h: DATH,
    _reserved0: [u8; 28usize],
    #[doc = "0x20 - DAC Status Register"]
    pub sr: SR,
    #[doc = "0x21 - DAC Control Register"]
    pub c0: C0,
    #[doc = "0x22 - DAC Control Register 1"]
    pub c1: C1,
    #[doc = "0x23 - DAC Control Register 2"]
    pub c2: C2,
}
#[doc = "DAC Data Low Register"]
pub struct DATL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DAC Data Low Register"]
pub mod datl;
#[doc = "DAC Data High Register"]
pub struct DATH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DAC Data High Register"]
pub mod dath;
#[doc = "DAC Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DAC Status Register"]
pub mod sr;
#[doc = "DAC Control Register"]
pub struct C0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DAC Control Register"]
pub mod c0;
#[doc = "DAC Control Register 1"]
pub struct C1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DAC Control Register 1"]
pub mod c1;
#[doc = "DAC Control Register 2"]
pub struct C2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DAC Control Register 2"]
pub mod c2;
