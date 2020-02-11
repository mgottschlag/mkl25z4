#[doc = "Reader of register PDOR"]
pub type R = crate::R<u32, super::PDOR>;
#[doc = "Writer for register PDOR"]
pub type W = crate::W<u32, super::PDOR>;
#[doc = "Register PDOR `reset()`'s with value 0"]
impl crate::ResetValue for super::PDOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PDO_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO_A> for u32 {
    #[inline(always)]
    fn from(variant: PDO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PDO`"]
pub type PDO_R = crate::R<u32, PDO_A>;
impl PDO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PDO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PDO_A::_0),
            1 => Val(PDO_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO_A::_1
    }
}
#[doc = "Write proxy for field `PDO`"]
pub struct PDO_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo(&self) -> PDO_R {
        PDO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo(&mut self) -> PDO_W {
        PDO_W { w: self }
    }
}
