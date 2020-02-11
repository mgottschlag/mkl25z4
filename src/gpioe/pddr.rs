#[doc = "Reader of register PDDR"]
pub type R = crate::R<u32, super::PDDR>;
#[doc = "Writer for register PDDR"]
pub type W = crate::W<u32, super::PDDR>;
#[doc = "Register PDDR `reset()`'s with value 0"]
impl crate::ResetValue for super::PDDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PDD_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD_A> for u32 {
    #[inline(always)]
    fn from(variant: PDD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PDD`"]
pub type PDD_R = crate::R<u32, PDD_A>;
impl PDD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PDD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PDD_A::_0),
            1 => Val(PDD_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD_A::_1
    }
}
#[doc = "Write proxy for field `PDD`"]
pub struct PDD_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd(&self) -> PDD_R {
        PDD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd(&mut self) -> PDD_W {
        PDD_W { w: self }
    }
}
