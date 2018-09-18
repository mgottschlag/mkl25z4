#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Crossbar Switch (AXBS) Slave Configuration"]
    pub plasc: PLASC,
    #[doc = "0x0a - Crossbar Switch (AXBS) Master Configuration"]
    pub plamc: PLAMC,
    #[doc = "0x0c - Platform Control Register"]
    pub placr: PLACR,
    _reserved1: [u8; 48usize],
    #[doc = "0x40 - Compute Operation Control Register"]
    pub cpo: CPO,
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub struct PLASC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub struct PLAMC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "Platform Control Register"]
pub struct PLACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Platform Control Register"]
pub mod placr;
#[doc = "Compute Operation Control Register"]
pub struct CPO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compute Operation Control Register"]
pub mod cpo;
