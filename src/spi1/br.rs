#[doc = "Reader of register BR"]
pub type R = crate::R<u8, super::BR>;
#[doc = "Writer for register BR"]
pub type W = crate::W<u8, super::BR>;
#[doc = "Register BR `reset()`'s with value 0"]
impl crate::ResetValue for super::BR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SPI baud rate divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPR_A {
    #[doc = "0: Baud rate divisor is 2"]
    _0000 = 0,
    #[doc = "1: Baud rate divisor is 4"]
    _0001 = 1,
    #[doc = "2: Baud rate divisor is 8"]
    _0010 = 2,
    #[doc = "3: Baud rate divisor is 16"]
    _0011 = 3,
    #[doc = "4: Baud rate divisor is 32"]
    _0100 = 4,
    #[doc = "5: Baud rate divisor is 64"]
    _0101 = 5,
    #[doc = "6: Baud rate divisor is 128"]
    _0110 = 6,
    #[doc = "7: Baud rate divisor is 256"]
    _0111 = 7,
    #[doc = "8: Baud rate divisor is 512"]
    _1000 = 8,
}
impl From<SPR_A> for u8 {
    #[inline(always)]
    fn from(variant: SPR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPR`"]
pub type SPR_R = crate::R<u8, SPR_A>;
impl SPR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPR_A::_0000),
            1 => Val(SPR_A::_0001),
            2 => Val(SPR_A::_0010),
            3 => Val(SPR_A::_0011),
            4 => Val(SPR_A::_0100),
            5 => Val(SPR_A::_0101),
            6 => Val(SPR_A::_0110),
            7 => Val(SPR_A::_0111),
            8 => Val(SPR_A::_1000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SPR_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SPR_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SPR_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SPR_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == SPR_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == SPR_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SPR_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == SPR_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == SPR_A::_1000
    }
}
#[doc = "Write proxy for field `SPR`"]
pub struct SPR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Baud rate divisor is 2"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(SPR_A::_0000)
    }
    #[doc = "Baud rate divisor is 4"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(SPR_A::_0001)
    }
    #[doc = "Baud rate divisor is 8"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(SPR_A::_0010)
    }
    #[doc = "Baud rate divisor is 16"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(SPR_A::_0011)
    }
    #[doc = "Baud rate divisor is 32"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(SPR_A::_0100)
    }
    #[doc = "Baud rate divisor is 64"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(SPR_A::_0101)
    }
    #[doc = "Baud rate divisor is 128"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(SPR_A::_0110)
    }
    #[doc = "Baud rate divisor is 256"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(SPR_A::_0111)
    }
    #[doc = "Baud rate divisor is 512"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(SPR_A::_1000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "SPI baud rate prescale divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPPR_A {
    #[doc = "0: Baud rate prescaler divisor is 1"]
    _000 = 0,
    #[doc = "1: Baud rate prescaler divisor is 2"]
    _001 = 1,
    #[doc = "2: Baud rate prescaler divisor is 3"]
    _010 = 2,
    #[doc = "3: Baud rate prescaler divisor is 4"]
    _011 = 3,
    #[doc = "4: Baud rate prescaler divisor is 5"]
    _100 = 4,
    #[doc = "5: Baud rate prescaler divisor is 6"]
    _101 = 5,
    #[doc = "6: Baud rate prescaler divisor is 7"]
    _110 = 6,
    #[doc = "7: Baud rate prescaler divisor is 8"]
    _111 = 7,
}
impl From<SPPR_A> for u8 {
    #[inline(always)]
    fn from(variant: SPPR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPPR`"]
pub type SPPR_R = crate::R<u8, SPPR_A>;
impl SPPR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPPR_A {
        match self.bits {
            0 => SPPR_A::_000,
            1 => SPPR_A::_001,
            2 => SPPR_A::_010,
            3 => SPPR_A::_011,
            4 => SPPR_A::_100,
            5 => SPPR_A::_101,
            6 => SPPR_A::_110,
            7 => SPPR_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPPR_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPPR_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPPR_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPPR_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPPR_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPPR_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPPR_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPPR_A::_111
    }
}
#[doc = "Write proxy for field `SPPR`"]
pub struct SPPR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPPR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPPR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Baud rate prescaler divisor is 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SPPR_A::_000)
    }
    #[doc = "Baud rate prescaler divisor is 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SPPR_A::_001)
    }
    #[doc = "Baud rate prescaler divisor is 3"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SPPR_A::_010)
    }
    #[doc = "Baud rate prescaler divisor is 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SPPR_A::_011)
    }
    #[doc = "Baud rate prescaler divisor is 5"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SPPR_A::_100)
    }
    #[doc = "Baud rate prescaler divisor is 6"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SPPR_A::_101)
    }
    #[doc = "Baud rate prescaler divisor is 7"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SPPR_A::_110)
    }
    #[doc = "Baud rate prescaler divisor is 8"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SPPR_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u8) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SPI baud rate divisor"]
    #[inline(always)]
    pub fn spr(&self) -> SPR_R {
        SPR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - SPI baud rate prescale divisor"]
    #[inline(always)]
    pub fn sppr(&self) -> SPPR_R {
        SPPR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SPI baud rate divisor"]
    #[inline(always)]
    pub fn spr(&mut self) -> SPR_W {
        SPR_W { w: self }
    }
    #[doc = "Bits 4:6 - SPI baud rate prescale divisor"]
    #[inline(always)]
    pub fn sppr(&mut self) -> SPPR_W {
        SPPR_W { w: self }
    }
}
