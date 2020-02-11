#[doc = "Reader of register STOPCTRL"]
pub type R = crate::R<u8, super::STOPCTRL>;
#[doc = "Writer for register STOPCTRL"]
pub type W = crate::W<u8, super::STOPCTRL>;
#[doc = "Register STOPCTRL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::STOPCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "VLLS Mode Control.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VLLSM_A {
    #[doc = "0: VLLS0"]
    _000 = 0,
    #[doc = "1: VLLS1"]
    _001 = 1,
    #[doc = "3: VLLS3"]
    _011 = 3,
}
impl From<VLLSM_A> for u8 {
    #[inline(always)]
    fn from(variant: VLLSM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VLLSM`"]
pub type VLLSM_R = crate::R<u8, VLLSM_A>;
impl VLLSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VLLSM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VLLSM_A::_000),
            1 => Val(VLLSM_A::_001),
            3 => Val(VLLSM_A::_011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == VLLSM_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == VLLSM_A::_001
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == VLLSM_A::_011
    }
}
#[doc = "Write proxy for field `VLLSM`"]
pub struct VLLSM_W<'a> {
    w: &'a mut W,
}
impl<'a> VLLSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLLSM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VLLS0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(VLLSM_A::_000)
    }
    #[doc = "VLLS1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(VLLSM_A::_001)
    }
    #[doc = "VLLS3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(VLLSM_A::_011)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "POR Power Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORPO_A {
    #[doc = "0: POR detect circuit is enabled in VLLS0"]
    _0 = 0,
    #[doc = "1: POR detect circuit is disabled in VLLS0"]
    _1 = 1,
}
impl From<PORPO_A> for bool {
    #[inline(always)]
    fn from(variant: PORPO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORPO`"]
pub type PORPO_R = crate::R<bool, PORPO_A>;
impl PORPO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORPO_A {
        match self.bits {
            false => PORPO_A::_0,
            true => PORPO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORPO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORPO_A::_1
    }
}
#[doc = "Write proxy for field `PORPO`"]
pub struct PORPO_W<'a> {
    w: &'a mut W,
}
impl<'a> PORPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORPO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "POR detect circuit is enabled in VLLS0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORPO_A::_0)
    }
    #[doc = "POR detect circuit is disabled in VLLS0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORPO_A::_1)
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
#[doc = "Partial Stop Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSTOPO_A {
    #[doc = "0: STOP - Normal Stop mode"]
    _00 = 0,
    #[doc = "1: PSTOP1 - Partial Stop with both system and bus clocks disabled"]
    _01 = 1,
    #[doc = "2: PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
    _10 = 2,
}
impl From<PSTOPO_A> for u8 {
    #[inline(always)]
    fn from(variant: PSTOPO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSTOPO`"]
pub type PSTOPO_R = crate::R<u8, PSTOPO_A>;
impl PSTOPO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PSTOPO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PSTOPO_A::_00),
            1 => Val(PSTOPO_A::_01),
            2 => Val(PSTOPO_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PSTOPO_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PSTOPO_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PSTOPO_A::_10
    }
}
#[doc = "Write proxy for field `PSTOPO`"]
pub struct PSTOPO_W<'a> {
    w: &'a mut W,
}
impl<'a> PSTOPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSTOPO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "STOP - Normal Stop mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PSTOPO_A::_00)
    }
    #[doc = "PSTOP1 - Partial Stop with both system and bus clocks disabled"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PSTOPO_A::_01)
    }
    #[doc = "PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PSTOPO_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - VLLS Mode Control."]
    #[inline(always)]
    pub fn vllsm(&self) -> VLLSM_R {
        VLLSM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline(always)]
    pub fn porpo(&self) -> PORPO_R {
        PORPO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Partial Stop Option"]
    #[inline(always)]
    pub fn pstopo(&self) -> PSTOPO_R {
        PSTOPO_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - VLLS Mode Control."]
    #[inline(always)]
    pub fn vllsm(&mut self) -> VLLSM_W {
        VLLSM_W { w: self }
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline(always)]
    pub fn porpo(&mut self) -> PORPO_W {
        PORPO_W { w: self }
    }
    #[doc = "Bits 6:7 - Partial Stop Option"]
    #[inline(always)]
    pub fn pstopo(&mut self) -> PSTOPO_W {
        PSTOPO_W { w: self }
    }
}
