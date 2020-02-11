#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Status Register"]
    pub fstat: FSTAT,
    #[doc = "0x01 - Flash Configuration Register"]
    pub fcnfg: FCNFG,
    #[doc = "0x02 - Flash Security Register"]
    pub fsec: FSEC,
    #[doc = "0x03 - Flash Option Register"]
    pub fopt: FOPT,
    #[doc = "0x04 - Flash Common Command Object Registers"]
    pub fccob3: FCCOB,
    #[doc = "0x05 - Flash Common Command Object Registers"]
    pub fccob2: FCCOB,
    #[doc = "0x06 - Flash Common Command Object Registers"]
    pub fccob1: FCCOB,
    #[doc = "0x07 - Flash Common Command Object Registers"]
    pub fccob0: FCCOB,
    #[doc = "0x08 - Flash Common Command Object Registers"]
    pub fccob7: FCCOB,
    #[doc = "0x09 - Flash Common Command Object Registers"]
    pub fccob6: FCCOB,
    #[doc = "0x0a - Flash Common Command Object Registers"]
    pub fccob5: FCCOB,
    #[doc = "0x0b - Flash Common Command Object Registers"]
    pub fccob4: FCCOB,
    #[doc = "0x0c - Flash Common Command Object Registers"]
    pub fccobb: FCCOB,
    #[doc = "0x0d - Flash Common Command Object Registers"]
    pub fccoba: FCCOB,
    #[doc = "0x0e - Flash Common Command Object Registers"]
    pub fccob9: FCCOB,
    #[doc = "0x0f - Flash Common Command Object Registers"]
    pub fccob8: FCCOB,
    #[doc = "0x10 - Program Flash Protection Registers"]
    pub fprot3: FPROT,
    #[doc = "0x11 - Program Flash Protection Registers"]
    pub fprot2: FPROT,
    #[doc = "0x12 - Program Flash Protection Registers"]
    pub fprot1: FPROT,
    #[doc = "0x13 - Program Flash Protection Registers"]
    pub fprot0: FPROT,
}
#[doc = "Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fstat](fstat) module"]
pub type FSTAT = crate::Reg<u8, _FSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSTAT;
#[doc = "`read()` method returns [fstat::R](fstat::R) reader structure"]
impl crate::Readable for FSTAT {}
#[doc = "`write(|w| ..)` method takes [fstat::W](fstat::W) writer structure"]
impl crate::Writable for FSTAT {}
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcnfg](fcnfg) module"]
pub type FCNFG = crate::Reg<u8, _FCNFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCNFG;
#[doc = "`read()` method returns [fcnfg::R](fcnfg::R) reader structure"]
impl crate::Readable for FCNFG {}
#[doc = "`write(|w| ..)` method takes [fcnfg::W](fcnfg::W) writer structure"]
impl crate::Writable for FCNFG {}
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "Flash Security Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsec](fsec) module"]
pub type FSEC = crate::Reg<u8, _FSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSEC;
#[doc = "`read()` method returns [fsec::R](fsec::R) reader structure"]
impl crate::Readable for FSEC {}
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "Flash Option Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fopt](fopt) module"]
pub type FOPT = crate::Reg<u8, _FOPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FOPT;
#[doc = "`read()` method returns [fopt::R](fopt::R) reader structure"]
impl crate::Readable for FOPT {}
#[doc = "Flash Option Register"]
pub mod fopt;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccob](fccob) module"]
pub type FCCOB = crate::Reg<u8, _FCCOB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOB;
#[doc = "`read()` method returns [fccob::R](fccob::R) reader structure"]
impl crate::Readable for FCCOB {}
#[doc = "`write(|w| ..)` method takes [fccob::W](fccob::W) writer structure"]
impl crate::Writable for FCCOB {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob;
#[doc = "Program Flash Protection Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot](fprot) module"]
pub type FPROT = crate::Reg<u8, _FPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPROT;
#[doc = "`read()` method returns [fprot::R](fprot::R) reader structure"]
impl crate::Readable for FPROT {}
#[doc = "`write(|w| ..)` method takes [fprot::W](fprot::W) writer structure"]
impl crate::Writable for FPROT {}
#[doc = "Program Flash Protection Registers"]
pub mod fprot;
