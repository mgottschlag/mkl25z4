#[doc = "Reader of register SOPT7"]
pub type R = crate::R<u32, super::SOPT7>;
#[doc = "Writer for register SOPT7"]
pub type W = crate::W<u32, super::SOPT7>;
#[doc = "Register SOPT7 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC0 trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC0TRGSEL_A {
    #[doc = "0: External trigger pin input (EXTRG_IN)"]
    _0000 = 0,
    #[doc = "1: CMP0 output"]
    _0001 = 1,
    #[doc = "4: PIT trigger 0"]
    _0100 = 4,
    #[doc = "5: PIT trigger 1"]
    _0101 = 5,
    #[doc = "8: TPM0 overflow"]
    _1000 = 8,
    #[doc = "9: TPM1 overflow"]
    _1001 = 9,
    #[doc = "10: TPM2 overflow"]
    _1010 = 10,
    #[doc = "12: RTC alarm"]
    _1100 = 12,
    #[doc = "13: RTC seconds"]
    _1101 = 13,
    #[doc = "14: LPTMR0 trigger"]
    _1110 = 14,
}
impl From<ADC0TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0TRGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC0TRGSEL`"]
pub type ADC0TRGSEL_R = crate::R<u8, ADC0TRGSEL_A>;
impl ADC0TRGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC0TRGSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC0TRGSEL_A::_0000),
            1 => Val(ADC0TRGSEL_A::_0001),
            4 => Val(ADC0TRGSEL_A::_0100),
            5 => Val(ADC0TRGSEL_A::_0101),
            8 => Val(ADC0TRGSEL_A::_1000),
            9 => Val(ADC0TRGSEL_A::_1001),
            10 => Val(ADC0TRGSEL_A::_1010),
            12 => Val(ADC0TRGSEL_A::_1100),
            13 => Val(ADC0TRGSEL_A::_1101),
            14 => Val(ADC0TRGSEL_A::_1110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == ADC0TRGSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == ADC0TRGSEL_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == ADC0TRGSEL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == ADC0TRGSEL_A::_0101
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == ADC0TRGSEL_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == ADC0TRGSEL_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == ADC0TRGSEL_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == ADC0TRGSEL_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == ADC0TRGSEL_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == ADC0TRGSEL_A::_1110
    }
}
#[doc = "Write proxy for field `ADC0TRGSEL`"]
pub struct ADC0TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0TRGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "External trigger pin input (EXTRG_IN)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0000)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0001)
    }
    #[doc = "PIT trigger 0"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0100)
    }
    #[doc = "PIT trigger 1"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0101)
    }
    #[doc = "TPM0 overflow"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1000)
    }
    #[doc = "TPM1 overflow"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1001)
    }
    #[doc = "TPM2 overflow"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1010)
    }
    #[doc = "RTC alarm"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1100)
    }
    #[doc = "RTC seconds"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1101)
    }
    #[doc = "LPTMR0 trigger"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "ADC0 pretrigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0PRETRGSEL_A {
    #[doc = "0: Pre-trigger A"]
    _0 = 0,
    #[doc = "1: Pre-trigger B"]
    _1 = 1,
}
impl From<ADC0PRETRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0PRETRGSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC0PRETRGSEL`"]
pub type ADC0PRETRGSEL_R = crate::R<bool, ADC0PRETRGSEL_A>;
impl ADC0PRETRGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0PRETRGSEL_A {
        match self.bits {
            false => ADC0PRETRGSEL_A::_0,
            true => ADC0PRETRGSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC0PRETRGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC0PRETRGSEL_A::_1
    }
}
#[doc = "Write proxy for field `ADC0PRETRGSEL`"]
pub struct ADC0PRETRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0PRETRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0PRETRGSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pre-trigger A"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0PRETRGSEL_A::_0)
    }
    #[doc = "Pre-trigger B"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0PRETRGSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "ADC0 alternate trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0ALTTRGEN_A {
    #[doc = "0: TPM1 channel 0 (A) and channel 1 (B) triggers selected for ADC0."]
    _0 = 0,
    #[doc = "1: Alternate trigger selected for ADC0."]
    _1 = 1,
}
impl From<ADC0ALTTRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0ALTTRGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC0ALTTRGEN`"]
pub type ADC0ALTTRGEN_R = crate::R<bool, ADC0ALTTRGEN_A>;
impl ADC0ALTTRGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0ALTTRGEN_A {
        match self.bits {
            false => ADC0ALTTRGEN_A::_0,
            true => ADC0ALTTRGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC0ALTTRGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC0ALTTRGEN_A::_1
    }
}
#[doc = "Write proxy for field `ADC0ALTTRGEN`"]
pub struct ADC0ALTTRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0ALTTRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0ALTTRGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TPM1 channel 0 (A) and channel 1 (B) triggers selected for ADC0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0ALTTRGEN_A::_0)
    }
    #[doc = "Alternate trigger selected for ADC0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0ALTTRGEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC0 trigger select"]
    #[inline(always)]
    pub fn adc0trgsel(&self) -> ADC0TRGSEL_R {
        ADC0TRGSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - ADC0 pretrigger select"]
    #[inline(always)]
    pub fn adc0pretrgsel(&self) -> ADC0PRETRGSEL_R {
        ADC0PRETRGSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC0 alternate trigger enable"]
    #[inline(always)]
    pub fn adc0alttrgen(&self) -> ADC0ALTTRGEN_R {
        ADC0ALTTRGEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC0 trigger select"]
    #[inline(always)]
    pub fn adc0trgsel(&mut self) -> ADC0TRGSEL_W {
        ADC0TRGSEL_W { w: self }
    }
    #[doc = "Bit 4 - ADC0 pretrigger select"]
    #[inline(always)]
    pub fn adc0pretrgsel(&mut self) -> ADC0PRETRGSEL_W {
        ADC0PRETRGSEL_W { w: self }
    }
    #[doc = "Bit 7 - ADC0 alternate trigger enable"]
    #[inline(always)]
    pub fn adc0alttrgen(&mut self) -> ADC0ALTTRGEN_W {
        ADC0ALTTRGEN_W { w: self }
    }
}
