#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MTB DWT Control Register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - MTB_DWT Comparator Register"]
    pub comp0: COMP,
    #[doc = "0x24 - MTB_DWT Comparator Mask Register"]
    pub mask0: MASK,
    #[doc = "0x28 - MTB_DWT Comparator Function Register 0"]
    pub fct0: FCT0,
    _reserved4: [u8; 4usize],
    #[doc = "0x30 - MTB_DWT Comparator Register"]
    pub comp1: COMP,
    #[doc = "0x34 - MTB_DWT Comparator Mask Register"]
    pub mask1: MASK,
    #[doc = "0x38 - MTB_DWT Comparator Function Register 1"]
    pub fct1: FCT1,
    _reserved7: [u8; 452usize],
    #[doc = "0x200 - MTB_DWT Trace Buffer Control Register"]
    pub tbctrl: TBCTRL,
    _reserved8: [u8; 3524usize],
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
#[doc = "MTB DWT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "MTB DWT Control Register"]
pub mod ctrl;
#[doc = "MTB_DWT Comparator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp](comp) module"]
pub type COMP = crate::Reg<u32, _COMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP;
#[doc = "`read()` method returns [comp::R](comp::R) reader structure"]
impl crate::Readable for COMP {}
#[doc = "`write(|w| ..)` method takes [comp::W](comp::W) writer structure"]
impl crate::Writable for COMP {}
#[doc = "MTB_DWT Comparator Register"]
pub mod comp;
#[doc = "MTB_DWT Comparator Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](mask) module"]
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "MTB_DWT Comparator Mask Register"]
pub mod mask;
#[doc = "MTB_DWT Comparator Function Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fct0](fct0) module"]
pub type FCT0 = crate::Reg<u32, _FCT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCT0;
#[doc = "`read()` method returns [fct0::R](fct0::R) reader structure"]
impl crate::Readable for FCT0 {}
#[doc = "`write(|w| ..)` method takes [fct0::W](fct0::W) writer structure"]
impl crate::Writable for FCT0 {}
#[doc = "MTB_DWT Comparator Function Register 0"]
pub mod fct0;
#[doc = "MTB_DWT Comparator Function Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fct1](fct1) module"]
pub type FCT1 = crate::Reg<u32, _FCT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCT1;
#[doc = "`read()` method returns [fct1::R](fct1::R) reader structure"]
impl crate::Readable for FCT1 {}
#[doc = "`write(|w| ..)` method takes [fct1::W](fct1::W) writer structure"]
impl crate::Writable for FCT1 {}
#[doc = "MTB_DWT Comparator Function Register 1"]
pub mod fct1;
#[doc = "MTB_DWT Trace Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbctrl](tbctrl) module"]
pub type TBCTRL = crate::Reg<u32, _TBCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBCTRL;
#[doc = "`read()` method returns [tbctrl::R](tbctrl::R) reader structure"]
impl crate::Readable for TBCTRL {}
#[doc = "`write(|w| ..)` method takes [tbctrl::W](tbctrl::W) writer structure"]
impl crate::Writable for TBCTRL {}
#[doc = "MTB_DWT Trace Buffer Control Register"]
pub mod tbctrl;
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
