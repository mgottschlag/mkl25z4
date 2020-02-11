#[doc = "Writer for register SRVCOP"]
pub type W = crate::W<u32, super::SRVCOP>;
#[doc = "Register SRVCOP `reset()`'s with value 0"]
impl crate::ResetValue for super::SRVCOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SRVCOP`"]
pub struct SRVCOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRVCOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Sevice COP Register"]
    #[inline(always)]
    pub fn srvcop(&mut self) -> SRVCOP_W {
        SRVCOP_W { w: self }
    }
}
