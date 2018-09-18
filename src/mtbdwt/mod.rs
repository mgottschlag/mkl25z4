#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MTB DWT Control Register"]
    pub ctrl: CTRL,
    _reserved0: [u8; 28usize],
    #[doc = "0x20 - MTB_DWT Comparator Register"]
    pub comp0: COMP,
    #[doc = "0x24 - MTB_DWT Comparator Mask Register"]
    pub mask0: MASK,
    #[doc = "0x28 - MTB_DWT Comparator Function Register 0"]
    pub fct0: FCT0,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - MTB_DWT Comparator Register"]
    pub comp1: COMP,
    #[doc = "0x34 - MTB_DWT Comparator Mask Register"]
    pub mask1: MASK,
    #[doc = "0x38 - MTB_DWT Comparator Function Register 1"]
    pub fct1: FCT1,
    _reserved2: [u8; 452usize],
    #[doc = "0x200 - MTB_DWT Trace Buffer Control Register"]
    pub tbctrl: TBCTRL,
    _reserved3: [u8; 3524usize],
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
#[doc = "MTB DWT Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB DWT Control Register"]
pub mod ctrl;
#[doc = "MTB_DWT Comparator Register"]
pub struct COMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB_DWT Comparator Register"]
pub mod comp;
#[doc = "MTB_DWT Comparator Mask Register"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB_DWT Comparator Mask Register"]
pub mod mask;
#[doc = "MTB_DWT Comparator Function Register 0"]
pub struct FCT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB_DWT Comparator Function Register 0"]
pub mod fct0;
#[doc = "MTB_DWT Comparator Function Register 1"]
pub struct FCT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB_DWT Comparator Function Register 1"]
pub mod fct1;
#[doc = "MTB_DWT Trace Buffer Control Register"]
pub struct TBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB_DWT Trace Buffer Control Register"]
pub mod tbctrl;
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
