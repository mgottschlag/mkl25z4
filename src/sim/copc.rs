#[doc = "Reader of register COPC"]
pub type R = crate::R<u32, super::COPC>;
#[doc = "Writer for register COPC"]
pub type W = crate::W<u32, super::COPC>;
#[doc = "Register COPC `reset()`'s with value 0x0c"]
impl crate::ResetValue for super::COPC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c
    }
}
#[doc = "COP windowed mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPW_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Windowed mode"]
    _1 = 1,
}
impl From<COPW_A> for bool {
    #[inline(always)]
    fn from(variant: COPW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COPW`"]
pub type COPW_R = crate::R<bool, COPW_A>;
impl COPW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COPW_A {
        match self.bits {
            false => COPW_A::_0,
            true => COPW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COPW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COPW_A::_1
    }
}
#[doc = "Write proxy for field `COPW`"]
pub struct COPW_W<'a> {
    w: &'a mut W,
}
impl<'a> COPW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COPW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COPW_A::_0)
    }
    #[doc = "Windowed mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COPW_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "COP Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPCLKS_A {
    #[doc = "0: Internal 1 kHz clock is source to COP"]
    _0 = 0,
    #[doc = "1: Bus clock is source to COP"]
    _1 = 1,
}
impl From<COPCLKS_A> for bool {
    #[inline(always)]
    fn from(variant: COPCLKS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COPCLKS`"]
pub type COPCLKS_R = crate::R<bool, COPCLKS_A>;
impl COPCLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COPCLKS_A {
        match self.bits {
            false => COPCLKS_A::_0,
            true => COPCLKS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COPCLKS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COPCLKS_A::_1
    }
}
#[doc = "Write proxy for field `COPCLKS`"]
pub struct COPCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> COPCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COPCLKS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal 1 kHz clock is source to COP"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COPCLKS_A::_0)
    }
    #[doc = "Bus clock is source to COP"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COPCLKS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "COP Watchdog Timeout\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COPT_A {
    #[doc = "0: COP disabled"]
    _00 = 0,
    #[doc = "1: COP timeout after 2^5 LPO cycles or 213 bus clock cycles"]
    _01 = 1,
    #[doc = "2: COP timeout after 2^8 LPO cycles or 216 bus clock cycles"]
    _10 = 2,
    #[doc = "3: COP timeout after 2^10 LPO cycles or 218 bus clock cycles"]
    _11 = 3,
}
impl From<COPT_A> for u8 {
    #[inline(always)]
    fn from(variant: COPT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COPT`"]
pub type COPT_R = crate::R<u8, COPT_A>;
impl COPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COPT_A {
        match self.bits {
            0 => COPT_A::_00,
            1 => COPT_A::_01,
            2 => COPT_A::_10,
            3 => COPT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == COPT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == COPT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == COPT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == COPT_A::_11
    }
}
#[doc = "Write proxy for field `COPT`"]
pub struct COPT_W<'a> {
    w: &'a mut W,
}
impl<'a> COPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COPT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "COP disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(COPT_A::_00)
    }
    #[doc = "COP timeout after 2^5 LPO cycles or 213 bus clock cycles"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(COPT_A::_01)
    }
    #[doc = "COP timeout after 2^8 LPO cycles or 216 bus clock cycles"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(COPT_A::_10)
    }
    #[doc = "COP timeout after 2^10 LPO cycles or 218 bus clock cycles"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(COPT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - COP windowed mode"]
    #[inline(always)]
    pub fn copw(&self) -> COPW_R {
        COPW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - COP Clock Select"]
    #[inline(always)]
    pub fn copclks(&self) -> COPCLKS_R {
        COPCLKS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - COP Watchdog Timeout"]
    #[inline(always)]
    pub fn copt(&self) -> COPT_R {
        COPT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - COP windowed mode"]
    #[inline(always)]
    pub fn copw(&mut self) -> COPW_W {
        COPW_W { w: self }
    }
    #[doc = "Bit 1 - COP Clock Select"]
    #[inline(always)]
    pub fn copclks(&mut self) -> COPCLKS_W {
        COPCLKS_W { w: self }
    }
    #[doc = "Bits 2:3 - COP Watchdog Timeout"]
    #[inline(always)]
    pub fn copt(&mut self) -> COPT_W {
        COPT_W { w: self }
    }
}
