#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Entry"]
    pub entry: [ENTRY; 3],
    #[doc = "0x0c - End of Table Marker Register"]
    pub tablemark: TABLEMARK,
    _reserved0: [u8; 4028usize],
    #[doc = "0xfcc - System Access Register"]
    pub sysaccess: SYSACCESS,
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
#[doc = "Entry"]
pub struct ENTRY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Entry"]
pub mod entry;
#[doc = "End of Table Marker Register"]
pub struct TABLEMARK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End of Table Marker Register"]
pub mod tablemark;
#[doc = "System Access Register"]
pub struct SYSACCESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Access Register"]
pub mod sysaccess;
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
