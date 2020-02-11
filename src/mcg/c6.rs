#[doc = "Reader of register C6"]
pub type R = crate::R<u8, super::C6>;
#[doc = "Writer for register C6"]
pub type W = crate::W<u8, super::C6>;
#[doc = "Register C6 `reset()`'s with value 0"]
impl crate::ResetValue for super::C6 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VDIV0`"]
pub type VDIV0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDIV0`"]
pub struct VDIV0_W<'a> {
    w: &'a mut W,
}
impl<'a> VDIV0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Clock Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CME0_A {
    #[doc = "0: External clock monitor is disabled for OSC0."]
    _0 = 0,
    #[doc = "1: External clock monitor is enabled for OSC0."]
    _1 = 1,
}
impl From<CME0_A> for bool {
    #[inline(always)]
    fn from(variant: CME0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CME0`"]
pub type CME0_R = crate::R<bool, CME0_A>;
impl CME0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CME0_A {
        match self.bits {
            false => CME0_A::_0,
            true => CME0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CME0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CME0_A::_1
    }
}
#[doc = "Write proxy for field `CME0`"]
pub struct CME0_W<'a> {
    w: &'a mut W,
}
impl<'a> CME0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CME0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External clock monitor is disabled for OSC0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CME0_A::_0)
    }
    #[doc = "External clock monitor is enabled for OSC0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CME0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "PLL Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLS_A {
    #[doc = "0: FLL is selected."]
    _0 = 0,
    #[doc = "1: PLL is selected (PRDIV 0 need to be programmed to the correct divider to generate a PLL reference clock in the range of 2-4 MHz prior to setting the PLLS bit)."]
    _1 = 1,
}
impl From<PLLS_A> for bool {
    #[inline(always)]
    fn from(variant: PLLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLS`"]
pub type PLLS_R = crate::R<bool, PLLS_A>;
impl PLLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLS_A {
        match self.bits {
            false => PLLS_A::_0,
            true => PLLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLS_A::_1
    }
}
#[doc = "Write proxy for field `PLLS`"]
pub struct PLLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLL is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLS_A::_0)
    }
    #[doc = "PLL is selected (PRDIV 0 need to be programmed to the correct divider to generate a PLL reference clock in the range of 2-4 MHz prior to setting the PLLS bit)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Loss of Lock Interrrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLIE0_A {
    #[doc = "0: No interrupt request is generated on loss of lock."]
    _0 = 0,
    #[doc = "1: Generate an interrupt request on loss of lock."]
    _1 = 1,
}
impl From<LOLIE0_A> for bool {
    #[inline(always)]
    fn from(variant: LOLIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOLIE0`"]
pub type LOLIE0_R = crate::R<bool, LOLIE0_A>;
impl LOLIE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOLIE0_A {
        match self.bits {
            false => LOLIE0_A::_0,
            true => LOLIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOLIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOLIE0_A::_1
    }
}
#[doc = "Write proxy for field `LOLIE0`"]
pub struct LOLIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOLIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOLIE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt request is generated on loss of lock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLIE0_A::_0)
    }
    #[doc = "Generate an interrupt request on loss of lock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLIE0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - VCO 0 Divider"]
    #[inline(always)]
    pub fn vdiv0(&self) -> VDIV0_R {
        VDIV0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline(always)]
    pub fn cme0(&self) -> CME0_R {
        CME0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Select"]
    #[inline(always)]
    pub fn plls(&self) -> PLLS_R {
        PLLS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loss of Lock Interrrupt Enable"]
    #[inline(always)]
    pub fn lolie0(&self) -> LOLIE0_R {
        LOLIE0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - VCO 0 Divider"]
    #[inline(always)]
    pub fn vdiv0(&mut self) -> VDIV0_W {
        VDIV0_W { w: self }
    }
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline(always)]
    pub fn cme0(&mut self) -> CME0_W {
        CME0_W { w: self }
    }
    #[doc = "Bit 6 - PLL Select"]
    #[inline(always)]
    pub fn plls(&mut self) -> PLLS_W {
        PLLS_W { w: self }
    }
    #[doc = "Bit 7 - Loss of Lock Interrrupt Enable"]
    #[inline(always)]
    pub fn lolie0(&mut self) -> LOLIE0_W {
        LOLIE0_W { w: self }
    }
}
