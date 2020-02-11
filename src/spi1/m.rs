#[doc = "Reader of register M"]
pub type R = crate::R<u8, super::M>;
#[doc = "Writer for register M"]
pub type W = crate::W<u8, super::M>;
#[doc = "Register M `reset()`'s with value 0"]
impl crate::ResetValue for super::M {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Bits`"]
pub type BITS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Bits`"]
pub struct BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Hardware compare value (low byte)"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Hardware compare value (low byte)"]
    #[inline(always)]
    pub fn bits_(&mut self) -> BITS_W {
        BITS_W { w: self }
    }
}
