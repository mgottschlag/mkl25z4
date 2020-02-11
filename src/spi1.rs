#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI control register 1"]
    pub c1: C1,
    #[doc = "0x01 - SPI control register 2"]
    pub c2: C2,
    #[doc = "0x02 - SPI baud rate register"]
    pub br: BR,
    #[doc = "0x03 - SPI status register"]
    pub s: S,
    _reserved4: [u8; 1usize],
    #[doc = "0x05 - SPI data register"]
    pub d: D,
    _reserved5: [u8; 1usize],
    #[doc = "0x07 - SPI match register"]
    pub m: M,
}
#[doc = "SPI control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](c1) module"]
pub type C1 = crate::Reg<u8, _C1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1;
#[doc = "`read()` method returns [c1::R](c1::R) reader structure"]
impl crate::Readable for C1 {}
#[doc = "`write(|w| ..)` method takes [c1::W](c1::W) writer structure"]
impl crate::Writable for C1 {}
#[doc = "SPI control register 1"]
pub mod c1;
#[doc = "SPI control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](c2) module"]
pub type C2 = crate::Reg<u8, _C2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2;
#[doc = "`read()` method returns [c2::R](c2::R) reader structure"]
impl crate::Readable for C2 {}
#[doc = "`write(|w| ..)` method takes [c2::W](c2::W) writer structure"]
impl crate::Writable for C2 {}
#[doc = "SPI control register 2"]
pub mod c2;
#[doc = "SPI baud rate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br](br) module"]
pub type BR = crate::Reg<u8, _BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR;
#[doc = "`read()` method returns [br::R](br::R) reader structure"]
impl crate::Readable for BR {}
#[doc = "`write(|w| ..)` method takes [br::W](br::W) writer structure"]
impl crate::Writable for BR {}
#[doc = "SPI baud rate register"]
pub mod br;
#[doc = "SPI status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s](s) module"]
pub type S = crate::Reg<u8, _S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S;
#[doc = "`read()` method returns [s::R](s::R) reader structure"]
impl crate::Readable for S {}
#[doc = "`write(|w| ..)` method takes [s::W](s::W) writer structure"]
impl crate::Writable for S {}
#[doc = "SPI status register"]
pub mod s;
#[doc = "SPI data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d](d) module"]
pub type D = crate::Reg<u8, _D>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D;
#[doc = "`read()` method returns [d::R](d::R) reader structure"]
impl crate::Readable for D {}
#[doc = "`write(|w| ..)` method takes [d::W](d::W) writer structure"]
impl crate::Writable for D {}
#[doc = "SPI data register"]
pub mod d;
#[doc = "SPI match register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m](m) module"]
pub type M = crate::Reg<u8, _M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M;
#[doc = "`read()` method returns [m::R](m::R) reader structure"]
impl crate::Readable for M {}
#[doc = "`write(|w| ..)` method takes [m::W](m::W) writer structure"]
impl crate::Writable for M {}
#[doc = "SPI match register"]
pub mod m;
