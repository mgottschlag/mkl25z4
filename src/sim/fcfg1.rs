#[doc = "Reader of register FCFG1"]
pub type R = crate::R<u32, super::FCFG1>;
#[doc = "Writer for register FCFG1"]
pub type W = crate::W<u32, super::FCFG1>;
#[doc = "Register FCFG1 `reset()`'s with value 0x0f00_0000"]
impl crate::ResetValue for super::FCFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f00_0000
    }
}
#[doc = "Flash Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHDIS_A {
    #[doc = "0: Flash is enabled"]
    _0 = 0,
    #[doc = "1: Flash is disabled"]
    _1 = 1,
}
impl From<FLASHDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASHDIS`"]
pub type FLASHDIS_R = crate::R<bool, FLASHDIS_A>;
impl FLASHDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHDIS_A {
        match self.bits {
            false => FLASHDIS_A::_0,
            true => FLASHDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLASHDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLASHDIS_A::_1
    }
}
#[doc = "Write proxy for field `FLASHDIS`"]
pub struct FLASHDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLASHDIS_A::_0)
    }
    #[doc = "Flash is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLASHDIS_A::_1)
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
#[doc = "Flash Doze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHDOZE_A {
    #[doc = "0: Flash remains enabled during Doze mode"]
    _0 = 0,
    #[doc = "1: Flash is disabled for the duration of Doze mode"]
    _1 = 1,
}
impl From<FLASHDOZE_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHDOZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASHDOZE`"]
pub type FLASHDOZE_R = crate::R<bool, FLASHDOZE_A>;
impl FLASHDOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHDOZE_A {
        match self.bits {
            false => FLASHDOZE_A::_0,
            true => FLASHDOZE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLASHDOZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLASHDOZE_A::_1
    }
}
#[doc = "Write proxy for field `FLASHDOZE`"]
pub struct FLASHDOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHDOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHDOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash remains enabled during Doze mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLASHDOZE_A::_0)
    }
    #[doc = "Flash is disabled for the duration of Doze mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLASHDOZE_A::_1)
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
#[doc = "Program flash size\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PFSIZE_A {
    #[doc = "0: 8 KB of program flash memory, 0.25 KB protection region"]
    _0000 = 0,
    #[doc = "1: 16 KB of program flash memory, 0.5 KB protection region"]
    _0001 = 1,
    #[doc = "3: 32 KB of program flash memory, 1 KB protection region"]
    _0011 = 3,
    #[doc = "5: 64 KB of program flash memory, 2 KB protection region"]
    _0101 = 5,
    #[doc = "7: 128 KB of program flash memory, 4 KB protection region"]
    _0111 = 7,
    #[doc = "9: 256 KB of program flash memory, 8 KB protection region"]
    _1001 = 9,
    #[doc = "15: 128 KB of program flash memory, 4 KB protection region"]
    _1111 = 15,
}
impl From<PFSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PFSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PFSIZE`"]
pub type PFSIZE_R = crate::R<u8, PFSIZE_A>;
impl PFSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PFSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PFSIZE_A::_0000),
            1 => Val(PFSIZE_A::_0001),
            3 => Val(PFSIZE_A::_0011),
            5 => Val(PFSIZE_A::_0101),
            7 => Val(PFSIZE_A::_0111),
            9 => Val(PFSIZE_A::_1001),
            15 => Val(PFSIZE_A::_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == PFSIZE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == PFSIZE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == PFSIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PFSIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PFSIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PFSIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == PFSIZE_A::_1111
    }
}
impl R {
    #[doc = "Bit 0 - Flash Disable"]
    #[inline(always)]
    pub fn flashdis(&self) -> FLASHDIS_R {
        FLASHDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash Doze"]
    #[inline(always)]
    pub fn flashdoze(&self) -> FLASHDOZE_R {
        FLASHDOZE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Program flash size"]
    #[inline(always)]
    pub fn pfsize(&self) -> PFSIZE_R {
        PFSIZE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Disable"]
    #[inline(always)]
    pub fn flashdis(&mut self) -> FLASHDIS_W {
        FLASHDIS_W { w: self }
    }
    #[doc = "Bit 1 - Flash Doze"]
    #[inline(always)]
    pub fn flashdoze(&mut self) -> FLASHDOZE_W {
        FLASHDOZE_W { w: self }
    }
}
