#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LLWU Pin Enable 1 register"]
    pub pe1: PE1,
    #[doc = "0x01 - LLWU Pin Enable 2 register"]
    pub pe2: PE2,
    #[doc = "0x02 - LLWU Pin Enable 3 register"]
    pub pe3: PE3,
    #[doc = "0x03 - LLWU Pin Enable 4 register"]
    pub pe4: PE4,
    #[doc = "0x04 - LLWU Module Enable register"]
    pub me: ME,
    #[doc = "0x05 - LLWU Flag 1 register"]
    pub f1: F1,
    #[doc = "0x06 - LLWU Flag 2 register"]
    pub f2: F2,
    #[doc = "0x07 - LLWU Flag 3 register"]
    pub f3: F3,
    #[doc = "0x08 - LLWU Pin Filter 1 register"]
    pub filt1: FILT1,
    #[doc = "0x09 - LLWU Pin Filter 2 register"]
    pub filt2: FILT2,
}
#[doc = "LLWU Pin Enable 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe1](pe1) module"]
pub type PE1 = crate::Reg<u8, _PE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE1;
#[doc = "`read()` method returns [pe1::R](pe1::R) reader structure"]
impl crate::Readable for PE1 {}
#[doc = "`write(|w| ..)` method takes [pe1::W](pe1::W) writer structure"]
impl crate::Writable for PE1 {}
#[doc = "LLWU Pin Enable 1 register"]
pub mod pe1;
#[doc = "LLWU Pin Enable 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe2](pe2) module"]
pub type PE2 = crate::Reg<u8, _PE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE2;
#[doc = "`read()` method returns [pe2::R](pe2::R) reader structure"]
impl crate::Readable for PE2 {}
#[doc = "`write(|w| ..)` method takes [pe2::W](pe2::W) writer structure"]
impl crate::Writable for PE2 {}
#[doc = "LLWU Pin Enable 2 register"]
pub mod pe2;
#[doc = "LLWU Pin Enable 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe3](pe3) module"]
pub type PE3 = crate::Reg<u8, _PE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE3;
#[doc = "`read()` method returns [pe3::R](pe3::R) reader structure"]
impl crate::Readable for PE3 {}
#[doc = "`write(|w| ..)` method takes [pe3::W](pe3::W) writer structure"]
impl crate::Writable for PE3 {}
#[doc = "LLWU Pin Enable 3 register"]
pub mod pe3;
#[doc = "LLWU Pin Enable 4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe4](pe4) module"]
pub type PE4 = crate::Reg<u8, _PE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE4;
#[doc = "`read()` method returns [pe4::R](pe4::R) reader structure"]
impl crate::Readable for PE4 {}
#[doc = "`write(|w| ..)` method takes [pe4::W](pe4::W) writer structure"]
impl crate::Writable for PE4 {}
#[doc = "LLWU Pin Enable 4 register"]
pub mod pe4;
#[doc = "LLWU Module Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [me](me) module"]
pub type ME = crate::Reg<u8, _ME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ME;
#[doc = "`read()` method returns [me::R](me::R) reader structure"]
impl crate::Readable for ME {}
#[doc = "`write(|w| ..)` method takes [me::W](me::W) writer structure"]
impl crate::Writable for ME {}
#[doc = "LLWU Module Enable register"]
pub mod me;
#[doc = "LLWU Flag 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f1](f1) module"]
pub type F1 = crate::Reg<u8, _F1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F1;
#[doc = "`read()` method returns [f1::R](f1::R) reader structure"]
impl crate::Readable for F1 {}
#[doc = "`write(|w| ..)` method takes [f1::W](f1::W) writer structure"]
impl crate::Writable for F1 {}
#[doc = "LLWU Flag 1 register"]
pub mod f1;
#[doc = "LLWU Flag 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f2](f2) module"]
pub type F2 = crate::Reg<u8, _F2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F2;
#[doc = "`read()` method returns [f2::R](f2::R) reader structure"]
impl crate::Readable for F2 {}
#[doc = "`write(|w| ..)` method takes [f2::W](f2::W) writer structure"]
impl crate::Writable for F2 {}
#[doc = "LLWU Flag 2 register"]
pub mod f2;
#[doc = "LLWU Flag 3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f3](f3) module"]
pub type F3 = crate::Reg<u8, _F3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F3;
#[doc = "`read()` method returns [f3::R](f3::R) reader structure"]
impl crate::Readable for F3 {}
#[doc = "LLWU Flag 3 register"]
pub mod f3;
#[doc = "LLWU Pin Filter 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filt1](filt1) module"]
pub type FILT1 = crate::Reg<u8, _FILT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILT1;
#[doc = "`read()` method returns [filt1::R](filt1::R) reader structure"]
impl crate::Readable for FILT1 {}
#[doc = "`write(|w| ..)` method takes [filt1::W](filt1::W) writer structure"]
impl crate::Writable for FILT1 {}
#[doc = "LLWU Pin Filter 1 register"]
pub mod filt1;
#[doc = "LLWU Pin Filter 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filt2](filt2) module"]
pub type FILT2 = crate::Reg<u8, _FILT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILT2;
#[doc = "`read()` method returns [filt2::R](filt2::R) reader structure"]
impl crate::Readable for FILT2 {}
#[doc = "`write(|w| ..)` method takes [filt2::W](filt2::W) writer structure"]
impl crate::Writable for FILT2 {}
#[doc = "LLWU Pin Filter 2 register"]
pub mod filt2;
