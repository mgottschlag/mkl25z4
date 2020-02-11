#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status Register 0"]
    pub srs0: SRS0,
    #[doc = "0x01 - System Reset Status Register 1"]
    pub srs1: SRS1,
    _reserved2: [u8; 2usize],
    #[doc = "0x04 - Reset Pin Filter Control register"]
    pub rpfc: RPFC,
    #[doc = "0x05 - Reset Pin Filter Width register"]
    pub rpfw: RPFW,
}
#[doc = "System Reset Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srs0](srs0) module"]
pub type SRS0 = crate::Reg<u8, _SRS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRS0;
#[doc = "`read()` method returns [srs0::R](srs0::R) reader structure"]
impl crate::Readable for SRS0 {}
#[doc = "System Reset Status Register 0"]
pub mod srs0;
#[doc = "System Reset Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srs1](srs1) module"]
pub type SRS1 = crate::Reg<u8, _SRS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRS1;
#[doc = "`read()` method returns [srs1::R](srs1::R) reader structure"]
impl crate::Readable for SRS1 {}
#[doc = "System Reset Status Register 1"]
pub mod srs1;
#[doc = "Reset Pin Filter Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpfc](rpfc) module"]
pub type RPFC = crate::Reg<u8, _RPFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPFC;
#[doc = "`read()` method returns [rpfc::R](rpfc::R) reader structure"]
impl crate::Readable for RPFC {}
#[doc = "`write(|w| ..)` method takes [rpfc::W](rpfc::W) writer structure"]
impl crate::Writable for RPFC {}
#[doc = "Reset Pin Filter Control register"]
pub mod rpfc;
#[doc = "Reset Pin Filter Width register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpfw](rpfw) module"]
pub type RPFW = crate::Reg<u8, _RPFW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPFW;
#[doc = "`read()` method returns [rpfw::R](rpfw::R) reader structure"]
impl crate::Readable for RPFW {}
#[doc = "`write(|w| ..)` method takes [rpfw::W](rpfw::W) writer structure"]
impl crate::Writable for RPFW {}
#[doc = "Reset Pin Filter Width register"]
pub mod rpfw;
