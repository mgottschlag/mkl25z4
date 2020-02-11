#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Crossbar Switch (AXBS) Slave Configuration"]
    pub plasc: PLASC,
    #[doc = "0x0a - Crossbar Switch (AXBS) Master Configuration"]
    pub plamc: PLAMC,
    #[doc = "0x0c - Platform Control Register"]
    pub placr: PLACR,
    _reserved3: [u8; 48usize],
    #[doc = "0x40 - Compute Operation Control Register"]
    pub cpo: CPO,
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plasc](plasc) module"]
pub type PLASC = crate::Reg<u16, _PLASC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLASC;
#[doc = "`read()` method returns [plasc::R](plasc::R) reader structure"]
impl crate::Readable for PLASC {}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "Crossbar Switch (AXBS) Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plamc](plamc) module"]
pub type PLAMC = crate::Reg<u16, _PLAMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLAMC;
#[doc = "`read()` method returns [plamc::R](plamc::R) reader structure"]
impl crate::Readable for PLAMC {}
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "Platform Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [placr](placr) module"]
pub type PLACR = crate::Reg<u32, _PLACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLACR;
#[doc = "`read()` method returns [placr::R](placr::R) reader structure"]
impl crate::Readable for PLACR {}
#[doc = "`write(|w| ..)` method takes [placr::W](placr::W) writer structure"]
impl crate::Writable for PLACR {}
#[doc = "Platform Control Register"]
pub mod placr;
#[doc = "Compute Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpo](cpo) module"]
pub type CPO = crate::Reg<u32, _CPO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPO;
#[doc = "`read()` method returns [cpo::R](cpo::R) reader structure"]
impl crate::Readable for CPO {}
#[doc = "`write(|w| ..)` method takes [cpo::W](cpo::W) writer structure"]
impl crate::Writable for CPO {}
#[doc = "Compute Operation Control Register"]
pub mod cpo;
