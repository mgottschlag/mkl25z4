#[doc = "Reader of register SOPT4"]
pub type R = crate::R<u32, super::SOPT4>;
#[doc = "Writer for register SOPT4"]
pub type W = crate::W<u32, super::SOPT4>;
#[doc = "Register SOPT4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TPM1 channel 0 input capture source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1CH0SRC_A {
    #[doc = "0: TPM1_CH0 signal"]
    _0 = 0,
    #[doc = "1: CMP0 output"]
    _1 = 1,
}
impl From<TPM1CH0SRC_A> for bool {
    #[inline(always)]
    fn from(variant: TPM1CH0SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TPM1CH0SRC`"]
pub type TPM1CH0SRC_R = crate::R<bool, TPM1CH0SRC_A>;
impl TPM1CH0SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM1CH0SRC_A {
        match self.bits {
            false => TPM1CH0SRC_A::_0,
            true => TPM1CH0SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPM1CH0SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPM1CH0SRC_A::_1
    }
}
#[doc = "Write proxy for field `TPM1CH0SRC`"]
pub struct TPM1CH0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM1CH0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM1CH0SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TPM1_CH0 signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM1CH0SRC_A::_0)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM1CH0SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "TPM2 channel 0 input capture source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM2CH0SRC_A {
    #[doc = "0: TPM2_CH0 signal"]
    _0 = 0,
    #[doc = "1: CMP0 output"]
    _1 = 1,
}
impl From<TPM2CH0SRC_A> for bool {
    #[inline(always)]
    fn from(variant: TPM2CH0SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TPM2CH0SRC`"]
pub type TPM2CH0SRC_R = crate::R<bool, TPM2CH0SRC_A>;
impl TPM2CH0SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM2CH0SRC_A {
        match self.bits {
            false => TPM2CH0SRC_A::_0,
            true => TPM2CH0SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPM2CH0SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPM2CH0SRC_A::_1
    }
}
#[doc = "Write proxy for field `TPM2CH0SRC`"]
pub struct TPM2CH0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM2CH0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM2CH0SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TPM2_CH0 signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM2CH0SRC_A::_0)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM2CH0SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "TPM0 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM0CLKSEL_A {
    #[doc = "0: TPM0 external clock driven by TPM_CLKIN0 pin."]
    _0 = 0,
    #[doc = "1: TPM0 external clock driven by TPM_CLKIN1 pin."]
    _1 = 1,
}
impl From<TPM0CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TPM0CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TPM0CLKSEL`"]
pub type TPM0CLKSEL_R = crate::R<bool, TPM0CLKSEL_A>;
impl TPM0CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM0CLKSEL_A {
        match self.bits {
            false => TPM0CLKSEL_A::_0,
            true => TPM0CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPM0CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPM0CLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `TPM0CLKSEL`"]
pub struct TPM0CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM0CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM0CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TPM0 external clock driven by TPM_CLKIN0 pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM0CLKSEL_A::_0)
    }
    #[doc = "TPM0 external clock driven by TPM_CLKIN1 pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM0CLKSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "TPM1 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1CLKSEL_A {
    #[doc = "0: TPM1 external clock driven by TPM_CLKIN0 pin."]
    _0 = 0,
    #[doc = "1: TPM1 external clock driven by TPM_CLKIN1 pin."]
    _1 = 1,
}
impl From<TPM1CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TPM1CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TPM1CLKSEL`"]
pub type TPM1CLKSEL_R = crate::R<bool, TPM1CLKSEL_A>;
impl TPM1CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM1CLKSEL_A {
        match self.bits {
            false => TPM1CLKSEL_A::_0,
            true => TPM1CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPM1CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPM1CLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `TPM1CLKSEL`"]
pub struct TPM1CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM1CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM1CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TPM1 external clock driven by TPM_CLKIN0 pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM1CLKSEL_A::_0)
    }
    #[doc = "TPM1 external clock driven by TPM_CLKIN1 pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM1CLKSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "TPM2 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM2CLKSEL_A {
    #[doc = "0: TPM2 external clock driven by TPM_CLKIN0 pin."]
    _0 = 0,
    #[doc = "1: TPM2 external clock driven by TPM_CLKIN1 pin."]
    _1 = 1,
}
impl From<TPM2CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TPM2CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TPM2CLKSEL`"]
pub type TPM2CLKSEL_R = crate::R<bool, TPM2CLKSEL_A>;
impl TPM2CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM2CLKSEL_A {
        match self.bits {
            false => TPM2CLKSEL_A::_0,
            true => TPM2CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPM2CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPM2CLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `TPM2CLKSEL`"]
pub struct TPM2CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM2CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM2CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TPM2 external clock driven by TPM_CLKIN0 pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM2CLKSEL_A::_0)
    }
    #[doc = "TPM2 external clock driven by TPM_CLKIN1 pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM2CLKSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - TPM1 channel 0 input capture source select"]
    #[inline(always)]
    pub fn tpm1ch0src(&self) -> TPM1CH0SRC_R {
        TPM1CH0SRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TPM2 channel 0 input capture source select"]
    #[inline(always)]
    pub fn tpm2ch0src(&self) -> TPM2CH0SRC_R {
        TPM2CH0SRC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TPM0 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm0clksel(&self) -> TPM0CLKSEL_R {
        TPM0CLKSEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TPM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm1clksel(&self) -> TPM1CLKSEL_R {
        TPM1CLKSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TPM2 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm2clksel(&self) -> TPM2CLKSEL_R {
        TPM2CLKSEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - TPM1 channel 0 input capture source select"]
    #[inline(always)]
    pub fn tpm1ch0src(&mut self) -> TPM1CH0SRC_W {
        TPM1CH0SRC_W { w: self }
    }
    #[doc = "Bit 20 - TPM2 channel 0 input capture source select"]
    #[inline(always)]
    pub fn tpm2ch0src(&mut self) -> TPM2CH0SRC_W {
        TPM2CH0SRC_W { w: self }
    }
    #[doc = "Bit 24 - TPM0 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm0clksel(&mut self) -> TPM0CLKSEL_W {
        TPM0CLKSEL_W { w: self }
    }
    #[doc = "Bit 25 - TPM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm1clksel(&mut self) -> TPM1CLKSEL_W {
        TPM1CLKSEL_W { w: self }
    }
    #[doc = "Bit 26 - TPM2 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm2clksel(&mut self) -> TPM2CLKSEL_W {
        TPM2CLKSEL_W { w: self }
    }
}
