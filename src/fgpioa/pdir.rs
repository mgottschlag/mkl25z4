#[doc = "Reader of register PDIR"]
pub type R = crate::R<u32, super::PDIR>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PDI_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI_A> for u32 {
    #[inline(always)]
    fn from(variant: PDI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PDI`"]
pub type PDI_R = crate::R<u32, PDI_A>;
impl PDI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PDI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PDI_A::_0),
            1 => Val(PDI_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI_A::_1
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Input"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
