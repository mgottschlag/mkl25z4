#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MTB Position Register"]
    pub position: POSITION,
    #[doc = "0x04 - MTB Master Register"]
    pub master: MASTER,
    #[doc = "0x08 - MTB Flow Register"]
    pub flow: FLOW,
    #[doc = "0x0c - MTB Base Register"]
    pub base: BASE,
    _reserved0: [u8; 3824usize],
    #[doc = "0xf00 - Integration Mode Control Register"]
    pub modectrl: MODECTRL,
    _reserved1: [u8; 156usize],
    #[doc = "0xfa0 - Claim TAG Set Register"]
    pub tagset: TAGSET,
    #[doc = "0xfa4 - Claim TAG Clear Register"]
    pub tagclear: TAGCLEAR,
    _reserved2: [u8; 8usize],
    #[doc = "0xfb0 - Lock Access Register"]
    pub lockaccess: LOCKACCESS,
    #[doc = "0xfb4 - Lock Status Register"]
    pub lockstat: LOCKSTAT,
    #[doc = "0xfb8 - Authentication Status Register"]
    pub authstat: AUTHSTAT,
    #[doc = "0xfbc - Device Architecture Register"]
    pub devicearch: DEVICEARCH,
    _reserved3: [u8; 8usize],
    #[doc = "0xfc8 - Device Configuration Register"]
    pub devicecfg: DEVICECFG,
    #[doc = "0xfcc - Device Type Identifier Register"]
    pub devicetypid: DEVICETYPID,
    #[doc = "0xfd0 - Peripheral ID Register"]
    pub periphid4: PERIPHID,
    #[doc = "0xfd4 - Peripheral ID Register"]
    pub periphid5: PERIPHID,
    #[doc = "0xfd8 - Peripheral ID Register"]
    pub periphid6: PERIPHID,
    #[doc = "0xfdc - Peripheral ID Register"]
    pub periphid7: PERIPHID,
    #[doc = "0xfe0 - Peripheral ID Register"]
    pub periphid0: PERIPHID,
    #[doc = "0xfe4 - Peripheral ID Register"]
    pub periphid1: PERIPHID,
    #[doc = "0xfe8 - Peripheral ID Register"]
    pub periphid2: PERIPHID,
    #[doc = "0xfec - Peripheral ID Register"]
    pub periphid3: PERIPHID,
    #[doc = "0xff0 - Component ID Register"]
    pub compid: [COMPID; 4],
}
#[doc = "MTB Position Register"]
pub struct POSITION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Position Register"]
pub mod position;
#[doc = "MTB Master Register"]
pub struct MASTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Master Register"]
pub mod master;
#[doc = "MTB Flow Register"]
pub struct FLOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Flow Register"]
pub mod flow;
#[doc = "MTB Base Register"]
pub struct BASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Base Register"]
pub mod base;
#[doc = "Integration Mode Control Register"]
pub struct MODECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Integration Mode Control Register"]
pub mod modectrl;
#[doc = "Claim TAG Set Register"]
pub struct TAGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Claim TAG Set Register"]
pub mod tagset;
#[doc = "Claim TAG Clear Register"]
pub struct TAGCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Claim TAG Clear Register"]
pub mod tagclear;
#[doc = "Lock Access Register"]
pub struct LOCKACCESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock Access Register"]
pub mod lockaccess;
#[doc = "Lock Status Register"]
pub struct LOCKSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock Status Register"]
pub mod lockstat;
#[doc = "Authentication Status Register"]
pub struct AUTHSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Authentication Status Register"]
pub mod authstat;
#[doc = "Device Architecture Register"]
pub struct DEVICEARCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Architecture Register"]
pub mod devicearch;
#[doc = "Device Configuration Register"]
pub struct DEVICECFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Configuration Register"]
pub mod devicecfg;
#[doc = "Device Type Identifier Register"]
pub struct DEVICETYPID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Type Identifier Register"]
pub mod devicetypid;
#[doc = "Peripheral ID Register"]
pub struct PERIPHID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral ID Register"]
pub mod periphid;
#[doc = "Component ID Register"]
pub struct COMPID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component ID Register"]
pub mod compid;
