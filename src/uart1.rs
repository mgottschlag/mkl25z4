#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Baud Rate Register: High"]
    pub bdh: BDH,
    #[doc = "0x01 - UART Baud Rate Register: Low"]
    pub bdl: BDL,
    #[doc = "0x02 - UART Control Register 1"]
    pub c1: C1,
    #[doc = "0x03 - UART Control Register 2"]
    pub c2: C2,
    #[doc = "0x04 - UART Status Register 1"]
    pub s1: S1,
    #[doc = "0x05 - UART Status Register 2"]
    pub s2: S2,
    #[doc = "0x06 - UART Control Register 3"]
    pub c3: C3,
    #[doc = "0x07 - UART Data Register"]
    pub d: D,
    #[doc = "0x08 - UART Control Register 4"]
    pub c4: C4,
}
#[doc = "UART Baud Rate Register: High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdh](bdh) module"]
pub type BDH = crate::Reg<u8, _BDH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDH;
#[doc = "`read()` method returns [bdh::R](bdh::R) reader structure"]
impl crate::Readable for BDH {}
#[doc = "`write(|w| ..)` method takes [bdh::W](bdh::W) writer structure"]
impl crate::Writable for BDH {}
#[doc = "UART Baud Rate Register: High"]
pub mod bdh;
#[doc = "UART Baud Rate Register: Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdl](bdl) module"]
pub type BDL = crate::Reg<u8, _BDL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDL;
#[doc = "`read()` method returns [bdl::R](bdl::R) reader structure"]
impl crate::Readable for BDL {}
#[doc = "`write(|w| ..)` method takes [bdl::W](bdl::W) writer structure"]
impl crate::Writable for BDL {}
#[doc = "UART Baud Rate Register: Low"]
pub mod bdl;
#[doc = "UART Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](c1) module"]
pub type C1 = crate::Reg<u8, _C1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1;
#[doc = "`read()` method returns [c1::R](c1::R) reader structure"]
impl crate::Readable for C1 {}
#[doc = "`write(|w| ..)` method takes [c1::W](c1::W) writer structure"]
impl crate::Writable for C1 {}
#[doc = "UART Control Register 1"]
pub mod c1;
#[doc = "UART Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](c2) module"]
pub type C2 = crate::Reg<u8, _C2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2;
#[doc = "`read()` method returns [c2::R](c2::R) reader structure"]
impl crate::Readable for C2 {}
#[doc = "`write(|w| ..)` method takes [c2::W](c2::W) writer structure"]
impl crate::Writable for C2 {}
#[doc = "UART Control Register 2"]
pub mod c2;
#[doc = "UART Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s1](s1) module"]
pub type S1 = crate::Reg<u8, _S1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S1;
#[doc = "`read()` method returns [s1::R](s1::R) reader structure"]
impl crate::Readable for S1 {}
#[doc = "UART Status Register 1"]
pub mod s1;
#[doc = "UART Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2](s2) module"]
pub type S2 = crate::Reg<u8, _S2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S2;
#[doc = "`read()` method returns [s2::R](s2::R) reader structure"]
impl crate::Readable for S2 {}
#[doc = "`write(|w| ..)` method takes [s2::W](s2::W) writer structure"]
impl crate::Writable for S2 {}
#[doc = "UART Status Register 2"]
pub mod s2;
#[doc = "UART Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3](c3) module"]
pub type C3 = crate::Reg<u8, _C3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3;
#[doc = "`read()` method returns [c3::R](c3::R) reader structure"]
impl crate::Readable for C3 {}
#[doc = "`write(|w| ..)` method takes [c3::W](c3::W) writer structure"]
impl crate::Writable for C3 {}
#[doc = "UART Control Register 3"]
pub mod c3;
#[doc = "UART Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d](d) module"]
pub type D = crate::Reg<u8, _D>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D;
#[doc = "`read()` method returns [d::R](d::R) reader structure"]
impl crate::Readable for D {}
#[doc = "`write(|w| ..)` method takes [d::W](d::W) writer structure"]
impl crate::Writable for D {}
#[doc = "UART Data Register"]
pub mod d;
#[doc = "UART Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4](c4) module"]
pub type C4 = crate::Reg<u8, _C4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4;
#[doc = "`read()` method returns [c4::R](c4::R) reader structure"]
impl crate::Readable for C4 {}
#[doc = "`write(|w| ..)` method takes [c4::W](c4::W) writer structure"]
impl crate::Writable for C4 {}
#[doc = "UART Control Register 4"]
pub mod c4;
