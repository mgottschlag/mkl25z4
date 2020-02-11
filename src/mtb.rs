#[doc = r"Register block"]
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
    _reserved4: [u8; 3824usize],
    #[doc = "0xf00 - Integration Mode Control Register"]
    pub modectrl: MODECTRL,
    _reserved5: [u8; 156usize],
    #[doc = "0xfa0 - Claim TAG Set Register"]
    pub tagset: TAGSET,
    #[doc = "0xfa4 - Claim TAG Clear Register"]
    pub tagclear: TAGCLEAR,
    _reserved7: [u8; 8usize],
    #[doc = "0xfb0 - Lock Access Register"]
    pub lockaccess: LOCKACCESS,
    #[doc = "0xfb4 - Lock Status Register"]
    pub lockstat: LOCKSTAT,
    #[doc = "0xfb8 - Authentication Status Register"]
    pub authstat: AUTHSTAT,
    #[doc = "0xfbc - Device Architecture Register"]
    pub devicearch: DEVICEARCH,
    _reserved11: [u8; 8usize],
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
#[doc = "MTB Position Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [position](position) module"]
pub type POSITION = crate::Reg<u32, _POSITION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POSITION;
#[doc = "`read()` method returns [position::R](position::R) reader structure"]
impl crate::Readable for POSITION {}
#[doc = "`write(|w| ..)` method takes [position::W](position::W) writer structure"]
impl crate::Writable for POSITION {}
#[doc = "MTB Position Register"]
pub mod position;
#[doc = "MTB Master Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master](master) module"]
pub type MASTER = crate::Reg<u32, _MASTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASTER;
#[doc = "`read()` method returns [master::R](master::R) reader structure"]
impl crate::Readable for MASTER {}
#[doc = "`write(|w| ..)` method takes [master::W](master::W) writer structure"]
impl crate::Writable for MASTER {}
#[doc = "MTB Master Register"]
pub mod master;
#[doc = "MTB Flow Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow](flow) module"]
pub type FLOW = crate::Reg<u32, _FLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOW;
#[doc = "`read()` method returns [flow::R](flow::R) reader structure"]
impl crate::Readable for FLOW {}
#[doc = "`write(|w| ..)` method takes [flow::W](flow::W) writer structure"]
impl crate::Writable for FLOW {}
#[doc = "MTB Flow Register"]
pub mod flow;
#[doc = "MTB Base Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base](base) module"]
pub type BASE = crate::Reg<u32, _BASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE;
#[doc = "`read()` method returns [base::R](base::R) reader structure"]
impl crate::Readable for BASE {}
#[doc = "MTB Base Register"]
pub mod base;
#[doc = "Integration Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modectrl](modectrl) module"]
pub type MODECTRL = crate::Reg<u32, _MODECTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODECTRL;
#[doc = "`read()` method returns [modectrl::R](modectrl::R) reader structure"]
impl crate::Readable for MODECTRL {}
#[doc = "Integration Mode Control Register"]
pub mod modectrl;
#[doc = "Claim TAG Set Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tagset](tagset) module"]
pub type TAGSET = crate::Reg<u32, _TAGSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAGSET;
#[doc = "`read()` method returns [tagset::R](tagset::R) reader structure"]
impl crate::Readable for TAGSET {}
#[doc = "Claim TAG Set Register"]
pub mod tagset;
#[doc = "Claim TAG Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tagclear](tagclear) module"]
pub type TAGCLEAR = crate::Reg<u32, _TAGCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAGCLEAR;
#[doc = "`read()` method returns [tagclear::R](tagclear::R) reader structure"]
impl crate::Readable for TAGCLEAR {}
#[doc = "Claim TAG Clear Register"]
pub mod tagclear;
#[doc = "Lock Access Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockaccess](lockaccess) module"]
pub type LOCKACCESS = crate::Reg<u32, _LOCKACCESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKACCESS;
#[doc = "`read()` method returns [lockaccess::R](lockaccess::R) reader structure"]
impl crate::Readable for LOCKACCESS {}
#[doc = "Lock Access Register"]
pub mod lockaccess;
#[doc = "Lock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockstat](lockstat) module"]
pub type LOCKSTAT = crate::Reg<u32, _LOCKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKSTAT;
#[doc = "`read()` method returns [lockstat::R](lockstat::R) reader structure"]
impl crate::Readable for LOCKSTAT {}
#[doc = "Lock Status Register"]
pub mod lockstat;
#[doc = "Authentication Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [authstat](authstat) module"]
pub type AUTHSTAT = crate::Reg<u32, _AUTHSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTHSTAT;
#[doc = "`read()` method returns [authstat::R](authstat::R) reader structure"]
impl crate::Readable for AUTHSTAT {}
#[doc = "Authentication Status Register"]
pub mod authstat;
#[doc = "Device Architecture Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devicearch](devicearch) module"]
pub type DEVICEARCH = crate::Reg<u32, _DEVICEARCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICEARCH;
#[doc = "`read()` method returns [devicearch::R](devicearch::R) reader structure"]
impl crate::Readable for DEVICEARCH {}
#[doc = "Device Architecture Register"]
pub mod devicearch;
#[doc = "Device Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devicecfg](devicecfg) module"]
pub type DEVICECFG = crate::Reg<u32, _DEVICECFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICECFG;
#[doc = "`read()` method returns [devicecfg::R](devicecfg::R) reader structure"]
impl crate::Readable for DEVICECFG {}
#[doc = "Device Configuration Register"]
pub mod devicecfg;
#[doc = "Device Type Identifier Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devicetypid](devicetypid) module"]
pub type DEVICETYPID = crate::Reg<u32, _DEVICETYPID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICETYPID;
#[doc = "`read()` method returns [devicetypid::R](devicetypid::R) reader structure"]
impl crate::Readable for DEVICETYPID {}
#[doc = "Device Type Identifier Register"]
pub mod devicetypid;
#[doc = "Peripheral ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periphid](periphid) module"]
pub type PERIPHID = crate::Reg<u32, _PERIPHID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPHID;
#[doc = "`read()` method returns [periphid::R](periphid::R) reader structure"]
impl crate::Readable for PERIPHID {}
#[doc = "Peripheral ID Register"]
pub mod periphid;
#[doc = "Component ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compid](compid) module"]
pub type COMPID = crate::Reg<u32, _COMPID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPID;
#[doc = "`read()` method returns [compid::R](compid::R) reader structure"]
impl crate::Readable for COMPID {}
#[doc = "Component ID Register"]
pub mod compid;
