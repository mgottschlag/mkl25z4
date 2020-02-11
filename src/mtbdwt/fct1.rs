#[doc = "Reader of register FCT1"]
pub type R = crate::R<u32, super::FCT1>;
#[doc = "Writer for register FCT1"]
pub type W = crate::W<u32, super::FCT1>;
#[doc = "Register FCT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNCTION_A {
    #[doc = "0: Disabled."]
    _0000 = 0,
    #[doc = "4: Instruction fetch."]
    _0100 = 4,
    #[doc = "5: Data operand read."]
    _0101 = 5,
    #[doc = "6: Data operand write."]
    _0110 = 6,
    #[doc = "7: Data operand (read + write)."]
    _0111 = 7,
}
impl From<FUNCTION_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNCTION_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FUNCTION`"]
pub type FUNCTION_R = crate::R<u8, FUNCTION_A>;
impl FUNCTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FUNCTION_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FUNCTION_A::_0000),
            4 => Val(FUNCTION_A::_0100),
            5 => Val(FUNCTION_A::_0101),
            6 => Val(FUNCTION_A::_0110),
            7 => Val(FUNCTION_A::_0111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == FUNCTION_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == FUNCTION_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == FUNCTION_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == FUNCTION_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == FUNCTION_A::_0111
    }
}
#[doc = "Write proxy for field `FUNCTION`"]
pub struct FUNCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNCTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNCTION_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(FUNCTION_A::_0000)
    }
    #[doc = "Instruction fetch."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(FUNCTION_A::_0100)
    }
    #[doc = "Data operand read."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(FUNCTION_A::_0101)
    }
    #[doc = "Data operand write."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(FUNCTION_A::_0110)
    }
    #[doc = "Data operand (read + write)."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(FUNCTION_A::_0111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Comparator match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCHED_A {
    #[doc = "0: No match."]
    _0 = 0,
    #[doc = "1: Match occurred."]
    _1 = 1,
}
impl From<MATCHED_A> for bool {
    #[inline(always)]
    fn from(variant: MATCHED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MATCHED`"]
pub type MATCHED_R = crate::R<bool, MATCHED_A>;
impl MATCHED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MATCHED_A {
        match self.bits {
            false => MATCHED_A::_0,
            true => MATCHED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MATCHED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MATCHED_A::_1
    }
}
impl R {
    #[doc = "Bits 0:3 - Function"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Comparator match"]
    #[inline(always)]
    pub fn matched(&self) -> MATCHED_R {
        MATCHED_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function"]
    #[inline(always)]
    pub fn function(&mut self) -> FUNCTION_W {
        FUNCTION_W { w: self }
    }
}
