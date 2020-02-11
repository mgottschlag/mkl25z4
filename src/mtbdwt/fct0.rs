#[doc = "Reader of register FCT0"]
pub type R = crate::R<u32, super::FCT0>;
#[doc = "Writer for register FCT0"]
pub type W = crate::W<u32, super::FCT0>;
#[doc = "Register FCT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCT0 {
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
#[doc = "Data Value Match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAVMATCH_A {
    #[doc = "0: Perform address comparison."]
    _0 = 0,
    #[doc = "1: Perform data value comparison."]
    _1 = 1,
}
impl From<DATAVMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: DATAVMATCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATAVMATCH`"]
pub type DATAVMATCH_R = crate::R<bool, DATAVMATCH_A>;
impl DATAVMATCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAVMATCH_A {
        match self.bits {
            false => DATAVMATCH_A::_0,
            true => DATAVMATCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATAVMATCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATAVMATCH_A::_1
    }
}
#[doc = "Write proxy for field `DATAVMATCH`"]
pub struct DATAVMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVMATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAVMATCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Perform address comparison."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATAVMATCH_A::_0)
    }
    #[doc = "Perform data value comparison."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATAVMATCH_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Data Value Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATAVSIZE_A {
    #[doc = "0: Byte."]
    _00 = 0,
    #[doc = "1: Halfword."]
    _01 = 1,
    #[doc = "2: Word."]
    _10 = 2,
    #[doc = "3: Reserved. Any attempts to use this value results in UNPREDICTABLE behavior."]
    _11 = 3,
}
impl From<DATAVSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAVSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DATAVSIZE`"]
pub type DATAVSIZE_R = crate::R<u8, DATAVSIZE_A>;
impl DATAVSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAVSIZE_A {
        match self.bits {
            0 => DATAVSIZE_A::_00,
            1 => DATAVSIZE_A::_01,
            2 => DATAVSIZE_A::_10,
            3 => DATAVSIZE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DATAVSIZE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DATAVSIZE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DATAVSIZE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DATAVSIZE_A::_11
    }
}
#[doc = "Write proxy for field `DATAVSIZE`"]
pub struct DATAVSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAVSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DATAVSIZE_A::_00)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DATAVSIZE_A::_01)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DATAVSIZE_A::_10)
    }
    #[doc = "Reserved. Any attempts to use this value results in UNPREDICTABLE behavior."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DATAVSIZE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `DATAVADDR0`"]
pub type DATAVADDR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAVADDR0`"]
pub struct DATAVADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
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
    #[doc = "Bit 8 - Data Value Match"]
    #[inline(always)]
    pub fn datavmatch(&self) -> DATAVMATCH_R {
        DATAVMATCH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Data Value Size"]
    #[inline(always)]
    pub fn datavsize(&self) -> DATAVSIZE_R {
        DATAVSIZE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - Data Value Address 0"]
    #[inline(always)]
    pub fn datavaddr0(&self) -> DATAVADDR0_R {
        DATAVADDR0_R::new(((self.bits >> 12) & 0x0f) as u8)
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
    #[doc = "Bit 8 - Data Value Match"]
    #[inline(always)]
    pub fn datavmatch(&mut self) -> DATAVMATCH_W {
        DATAVMATCH_W { w: self }
    }
    #[doc = "Bits 10:11 - Data Value Size"]
    #[inline(always)]
    pub fn datavsize(&mut self) -> DATAVSIZE_W {
        DATAVSIZE_W { w: self }
    }
    #[doc = "Bits 12:15 - Data Value Address 0"]
    #[inline(always)]
    pub fn datavaddr0(&mut self) -> DATAVADDR0_W {
        DATAVADDR0_W { w: self }
    }
}
