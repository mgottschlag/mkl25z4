#[doc = "Writer for register PSOR"]
pub type W = crate::W<u32, super::PSOR>;
#[doc = "Register PSOR `reset()`'s with value 0"]
impl crate::ResetValue for super::PSOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PTSO_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO_AW> for u32 {
    #[inline(always)]
    fn from(variant: PTSO_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `PTSO`"]
pub struct PTSO_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO_AW::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Set Output"]
    #[inline(always)]
    pub fn ptso(&mut self) -> PTSO_W {
        PTSO_W { w: self }
    }
}
