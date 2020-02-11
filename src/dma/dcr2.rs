#[doc = "Reader of register DCR2"]
pub type R = crate::R<u32, super::DCR2>;
#[doc = "Writer for register DCR2"]
pub type W = crate::W<u32, super::DCR2>;
#[doc = "Register DCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Link channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCH2_A {
    #[doc = "0: DMA Channel 0"]
    _00 = 0,
    #[doc = "1: DMA Channel 1"]
    _01 = 1,
    #[doc = "2: DMA Channel 2"]
    _10 = 2,
    #[doc = "3: DMA Channel 3"]
    _11 = 3,
}
impl From<LCH2_A> for u8 {
    #[inline(always)]
    fn from(variant: LCH2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LCH2`"]
pub type LCH2_R = crate::R<u8, LCH2_A>;
impl LCH2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCH2_A {
        match self.bits {
            0 => LCH2_A::_00,
            1 => LCH2_A::_01,
            2 => LCH2_A::_10,
            3 => LCH2_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LCH2_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LCH2_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LCH2_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LCH2_A::_11
    }
}
#[doc = "Write proxy for field `LCH2`"]
pub struct LCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCH2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DMA Channel 0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LCH2_A::_00)
    }
    #[doc = "DMA Channel 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LCH2_A::_01)
    }
    #[doc = "DMA Channel 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LCH2_A::_10)
    }
    #[doc = "DMA Channel 3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LCH2_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Link channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCH1_A {
    #[doc = "0: DMA Channel 0"]
    _00 = 0,
    #[doc = "1: DMA Channel 1"]
    _01 = 1,
    #[doc = "2: DMA Channel 2"]
    _10 = 2,
    #[doc = "3: DMA Channel 3"]
    _11 = 3,
}
impl From<LCH1_A> for u8 {
    #[inline(always)]
    fn from(variant: LCH1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LCH1`"]
pub type LCH1_R = crate::R<u8, LCH1_A>;
impl LCH1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCH1_A {
        match self.bits {
            0 => LCH1_A::_00,
            1 => LCH1_A::_01,
            2 => LCH1_A::_10,
            3 => LCH1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LCH1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LCH1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LCH1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LCH1_A::_11
    }
}
#[doc = "Write proxy for field `LCH1`"]
pub struct LCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCH1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DMA Channel 0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LCH1_A::_00)
    }
    #[doc = "DMA Channel 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LCH1_A::_01)
    }
    #[doc = "DMA Channel 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LCH1_A::_10)
    }
    #[doc = "DMA Channel 3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LCH1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Link channel control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINKCC_A {
    #[doc = "0: No channel-to-channel linking"]
    _00 = 0,
    #[doc = "1: Perform a link to channel LCH1 after each cycle-steal transfer followed by a link to LCH2 after the BCR decrements to zero"]
    _01 = 1,
    #[doc = "2: Perform a link to channel LCH1 after each cycle-steal transfer"]
    _10 = 2,
    #[doc = "3: Perform a link to channel LCH1 after the BCR decrements to zero"]
    _11 = 3,
}
impl From<LINKCC_A> for u8 {
    #[inline(always)]
    fn from(variant: LINKCC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LINKCC`"]
pub type LINKCC_R = crate::R<u8, LINKCC_A>;
impl LINKCC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINKCC_A {
        match self.bits {
            0 => LINKCC_A::_00,
            1 => LINKCC_A::_01,
            2 => LINKCC_A::_10,
            3 => LINKCC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LINKCC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LINKCC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LINKCC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LINKCC_A::_11
    }
}
#[doc = "Write proxy for field `LINKCC`"]
pub struct LINKCC_W<'a> {
    w: &'a mut W,
}
impl<'a> LINKCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINKCC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No channel-to-channel linking"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LINKCC_A::_00)
    }
    #[doc = "Perform a link to channel LCH1 after each cycle-steal transfer followed by a link to LCH2 after the BCR decrements to zero"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LINKCC_A::_01)
    }
    #[doc = "Perform a link to channel LCH1 after each cycle-steal transfer"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LINKCC_A::_10)
    }
    #[doc = "Perform a link to channel LCH1 after the BCR decrements to zero"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LINKCC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Disable request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D_REQ_A {
    #[doc = "0: ERQ bit is not affected."]
    _0 = 0,
    #[doc = "1: ERQ bit is cleared when the BCR is exhausted."]
    _1 = 1,
}
impl From<D_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: D_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `D_REQ`"]
pub type D_REQ_R = crate::R<bool, D_REQ_A>;
impl D_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> D_REQ_A {
        match self.bits {
            false => D_REQ_A::_0,
            true => D_REQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == D_REQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == D_REQ_A::_1
    }
}
#[doc = "Write proxy for field `D_REQ`"]
pub struct D_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> D_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ERQ bit is not affected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(D_REQ_A::_0)
    }
    #[doc = "ERQ bit is cleared when the BCR is exhausted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(D_REQ_A::_1)
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
#[doc = "Destination address modulo\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMOD_A {
    #[doc = "0: Buffer disabled"]
    _0000 = 0,
    #[doc = "1: Circular buffer size is 16 bytes"]
    _0001 = 1,
    #[doc = "2: Circular buffer size is 32 bytes"]
    _0010 = 2,
    #[doc = "3: Circular buffer size is 64 bytes"]
    _0011 = 3,
    #[doc = "4: Circular buffer size is 128 bytes"]
    _0100 = 4,
    #[doc = "5: Circular buffer size is 256 bytes"]
    _0101 = 5,
    #[doc = "6: Circular buffer size is 512 bytes"]
    _0110 = 6,
    #[doc = "7: Circular buffer size is 1 KB"]
    _0111 = 7,
    #[doc = "8: Circular buffer size is 2 KB"]
    _1000 = 8,
    #[doc = "9: Circular buffer size is 4 KB"]
    _1001 = 9,
    #[doc = "10: Circular buffer size is 8 KB"]
    _1010 = 10,
    #[doc = "11: Circular buffer size is 16 KB"]
    _1011 = 11,
    #[doc = "12: Circular buffer size is 32 KB"]
    _1100 = 12,
    #[doc = "13: Circular buffer size is 64 KB"]
    _1101 = 13,
    #[doc = "14: Circular buffer size is 128 KB"]
    _1110 = 14,
    #[doc = "15: Circular buffer size is 256 KB"]
    _1111 = 15,
}
impl From<DMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: DMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMOD`"]
pub type DMOD_R = crate::R<u8, DMOD_A>;
impl DMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMOD_A {
        match self.bits {
            0 => DMOD_A::_0000,
            1 => DMOD_A::_0001,
            2 => DMOD_A::_0010,
            3 => DMOD_A::_0011,
            4 => DMOD_A::_0100,
            5 => DMOD_A::_0101,
            6 => DMOD_A::_0110,
            7 => DMOD_A::_0111,
            8 => DMOD_A::_1000,
            9 => DMOD_A::_1001,
            10 => DMOD_A::_1010,
            11 => DMOD_A::_1011,
            12 => DMOD_A::_1100,
            13 => DMOD_A::_1101,
            14 => DMOD_A::_1110,
            15 => DMOD_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DMOD_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DMOD_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DMOD_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DMOD_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DMOD_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DMOD_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == DMOD_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == DMOD_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == DMOD_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == DMOD_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == DMOD_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == DMOD_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == DMOD_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == DMOD_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == DMOD_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == DMOD_A::_1111
    }
}
#[doc = "Write proxy for field `DMOD`"]
pub struct DMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Buffer disabled"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DMOD_A::_0000)
    }
    #[doc = "Circular buffer size is 16 bytes"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DMOD_A::_0001)
    }
    #[doc = "Circular buffer size is 32 bytes"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DMOD_A::_0010)
    }
    #[doc = "Circular buffer size is 64 bytes"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DMOD_A::_0011)
    }
    #[doc = "Circular buffer size is 128 bytes"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DMOD_A::_0100)
    }
    #[doc = "Circular buffer size is 256 bytes"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DMOD_A::_0101)
    }
    #[doc = "Circular buffer size is 512 bytes"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DMOD_A::_0110)
    }
    #[doc = "Circular buffer size is 1 KB"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DMOD_A::_0111)
    }
    #[doc = "Circular buffer size is 2 KB"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(DMOD_A::_1000)
    }
    #[doc = "Circular buffer size is 4 KB"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(DMOD_A::_1001)
    }
    #[doc = "Circular buffer size is 8 KB"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(DMOD_A::_1010)
    }
    #[doc = "Circular buffer size is 16 KB"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(DMOD_A::_1011)
    }
    #[doc = "Circular buffer size is 32 KB"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(DMOD_A::_1100)
    }
    #[doc = "Circular buffer size is 64 KB"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(DMOD_A::_1101)
    }
    #[doc = "Circular buffer size is 128 KB"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DMOD_A::_1110)
    }
    #[doc = "Circular buffer size is 256 KB"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(DMOD_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Source address modulo\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD_A {
    #[doc = "0: Buffer disabled"]
    _0000 = 0,
    #[doc = "1: Circular buffer size is 16 bytes"]
    _0001 = 1,
    #[doc = "2: Circular buffer size is 32 bytes"]
    _0010 = 2,
    #[doc = "3: Circular buffer size is 64 bytes"]
    _0011 = 3,
    #[doc = "4: Circular buffer size is 128 bytes"]
    _0100 = 4,
    #[doc = "5: Circular buffer size is 256 bytes"]
    _0101 = 5,
    #[doc = "6: Circular buffer size is 512 bytes"]
    _0110 = 6,
    #[doc = "7: Circular buffer size is 1 KB"]
    _0111 = 7,
    #[doc = "8: Circular buffer size is 2 KB"]
    _1000 = 8,
    #[doc = "9: Circular buffer size is 4 KB"]
    _1001 = 9,
    #[doc = "10: Circular buffer size is 8 KB"]
    _1010 = 10,
    #[doc = "11: Circular buffer size is 16 KB"]
    _1011 = 11,
    #[doc = "12: Circular buffer size is 32 KB"]
    _1100 = 12,
    #[doc = "13: Circular buffer size is 64 KB"]
    _1101 = 13,
    #[doc = "14: Circular buffer size is 128 KB"]
    _1110 = 14,
    #[doc = "15: Circular buffer size is 256 KB"]
    _1111 = 15,
}
impl From<SMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMOD`"]
pub type SMOD_R = crate::R<u8, SMOD_A>;
impl SMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMOD_A {
        match self.bits {
            0 => SMOD_A::_0000,
            1 => SMOD_A::_0001,
            2 => SMOD_A::_0010,
            3 => SMOD_A::_0011,
            4 => SMOD_A::_0100,
            5 => SMOD_A::_0101,
            6 => SMOD_A::_0110,
            7 => SMOD_A::_0111,
            8 => SMOD_A::_1000,
            9 => SMOD_A::_1001,
            10 => SMOD_A::_1010,
            11 => SMOD_A::_1011,
            12 => SMOD_A::_1100,
            13 => SMOD_A::_1101,
            14 => SMOD_A::_1110,
            15 => SMOD_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SMOD_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SMOD_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SMOD_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SMOD_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == SMOD_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == SMOD_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SMOD_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == SMOD_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == SMOD_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == SMOD_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == SMOD_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == SMOD_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == SMOD_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == SMOD_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == SMOD_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == SMOD_A::_1111
    }
}
#[doc = "Write proxy for field `SMOD`"]
pub struct SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Buffer disabled"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(SMOD_A::_0000)
    }
    #[doc = "Circular buffer size is 16 bytes"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(SMOD_A::_0001)
    }
    #[doc = "Circular buffer size is 32 bytes"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(SMOD_A::_0010)
    }
    #[doc = "Circular buffer size is 64 bytes"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(SMOD_A::_0011)
    }
    #[doc = "Circular buffer size is 128 bytes"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(SMOD_A::_0100)
    }
    #[doc = "Circular buffer size is 256 bytes"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(SMOD_A::_0101)
    }
    #[doc = "Circular buffer size is 512 bytes"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(SMOD_A::_0110)
    }
    #[doc = "Circular buffer size is 1 KB"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(SMOD_A::_0111)
    }
    #[doc = "Circular buffer size is 2 KB"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(SMOD_A::_1000)
    }
    #[doc = "Circular buffer size is 4 KB"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(SMOD_A::_1001)
    }
    #[doc = "Circular buffer size is 8 KB"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(SMOD_A::_1010)
    }
    #[doc = "Circular buffer size is 16 KB"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(SMOD_A::_1011)
    }
    #[doc = "Circular buffer size is 32 KB"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(SMOD_A::_1100)
    }
    #[doc = "Circular buffer size is 64 KB"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(SMOD_A::_1101)
    }
    #[doc = "Circular buffer size is 128 KB"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(SMOD_A::_1110)
    }
    #[doc = "Circular buffer size is 256 KB"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(SMOD_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Start transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_AW {
    #[doc = "0: DMA inactive"]
    _0 = 0,
    #[doc = "1: The DMA begins the transfer in accordance to the values in the TCDn. START is cleared automatically after one module clock and always reads as logic 0."]
    _1 = 1,
}
impl From<START_AW> for bool {
    #[inline(always)]
    fn from(variant: START_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA inactive"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(START_AW::_0)
    }
    #[doc = "The DMA begins the transfer in accordance to the values in the TCDn. START is cleared automatically after one module clock and always reads as logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(START_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Destination size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSIZE_A {
    #[doc = "0: 32-bit"]
    _00 = 0,
    #[doc = "1: 8-bit"]
    _01 = 1,
    #[doc = "2: 16-bit"]
    _10 = 2,
    #[doc = "3: Reserved (generates a configuration error (DSRn\\[CE\\]) if incorrectly specified at time of channel activation)"]
    _11 = 3,
}
impl From<DSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSIZE`"]
pub type DSIZE_R = crate::R<u8, DSIZE_A>;
impl DSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSIZE_A {
        match self.bits {
            0 => DSIZE_A::_00,
            1 => DSIZE_A::_01,
            2 => DSIZE_A::_10,
            3 => DSIZE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DSIZE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DSIZE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DSIZE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DSIZE_A::_11
    }
}
#[doc = "Write proxy for field `DSIZE`"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DSIZE_A::_00)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DSIZE_A::_01)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DSIZE_A::_10)
    }
    #[doc = "Reserved (generates a configuration error (DSRn\\[CE\\]) if incorrectly specified at time of channel activation)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DSIZE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Destination increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINC_A {
    #[doc = "0: No change to the DAR after a successful transfer."]
    _0 = 0,
    #[doc = "1: The DAR increments by 1, 2, 4 depending upon the size of the transfer."]
    _1 = 1,
}
impl From<DINC_A> for bool {
    #[inline(always)]
    fn from(variant: DINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DINC`"]
pub type DINC_R = crate::R<bool, DINC_A>;
impl DINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINC_A {
        match self.bits {
            false => DINC_A::_0,
            true => DINC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DINC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DINC_A::_1
    }
}
#[doc = "Write proxy for field `DINC`"]
pub struct DINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No change to the DAR after a successful transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINC_A::_0)
    }
    #[doc = "The DAR increments by 1, 2, 4 depending upon the size of the transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Source size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSIZE_A {
    #[doc = "0: 32-bit"]
    _00 = 0,
    #[doc = "1: 8-bit"]
    _01 = 1,
    #[doc = "2: 16-bit"]
    _10 = 2,
    #[doc = "3: Reserved (generates a configuration error (DSRn\\[CE\\]) if incorrectly specified at time of channel activation)"]
    _11 = 3,
}
impl From<SSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSIZE`"]
pub type SSIZE_R = crate::R<u8, SSIZE_A>;
impl SSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSIZE_A {
        match self.bits {
            0 => SSIZE_A::_00,
            1 => SSIZE_A::_01,
            2 => SSIZE_A::_10,
            3 => SSIZE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SSIZE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SSIZE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SSIZE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SSIZE_A::_11
    }
}
#[doc = "Write proxy for field `SSIZE`"]
pub struct SSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SSIZE_A::_00)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SSIZE_A::_01)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SSIZE_A::_10)
    }
    #[doc = "Reserved (generates a configuration error (DSRn\\[CE\\]) if incorrectly specified at time of channel activation)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SSIZE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Source increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINC_A {
    #[doc = "0: No change to SAR after a successful transfer."]
    _0 = 0,
    #[doc = "1: The SAR increments by 1, 2, 4 as determined by the transfer size."]
    _1 = 1,
}
impl From<SINC_A> for bool {
    #[inline(always)]
    fn from(variant: SINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SINC`"]
pub type SINC_R = crate::R<bool, SINC_A>;
impl SINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINC_A {
        match self.bits {
            false => SINC_A::_0,
            true => SINC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SINC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SINC_A::_1
    }
}
#[doc = "Write proxy for field `SINC`"]
pub struct SINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SINC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No change to SAR after a successful transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SINC_A::_0)
    }
    #[doc = "The SAR increments by 1, 2, 4 as determined by the transfer size."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SINC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Enable asynchronous DMA requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EADREQ_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<EADREQ_A> for bool {
    #[inline(always)]
    fn from(variant: EADREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EADREQ`"]
pub type EADREQ_R = crate::R<bool, EADREQ_A>;
impl EADREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EADREQ_A {
        match self.bits {
            false => EADREQ_A::_0,
            true => EADREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EADREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EADREQ_A::_1
    }
}
#[doc = "Write proxy for field `EADREQ`"]
pub struct EADREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EADREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EADREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EADREQ_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EADREQ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Auto-align\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AA_A {
    #[doc = "0: Auto-align disabled"]
    _0 = 0,
    #[doc = "1: If SSIZE indicates a transfer no smaller than DSIZE, source accesses are auto-aligned; otherwise, destination accesses are auto-aligned. Source alignment takes precedence over destination alignment. If auto-alignment is enabled, the appropriate address register increments, regardless of DINC or SINC."]
    _1 = 1,
}
impl From<AA_A> for bool {
    #[inline(always)]
    fn from(variant: AA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AA`"]
pub type AA_R = crate::R<bool, AA_A>;
impl AA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AA_A {
        match self.bits {
            false => AA_A::_0,
            true => AA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AA_A::_1
    }
}
#[doc = "Write proxy for field `AA`"]
pub struct AA_W<'a> {
    w: &'a mut W,
}
impl<'a> AA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Auto-align disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AA_A::_0)
    }
    #[doc = "If SSIZE indicates a transfer no smaller than DSIZE, source accesses are auto-aligned; otherwise, destination accesses are auto-aligned. Source alignment takes precedence over destination alignment. If auto-alignment is enabled, the appropriate address register increments, regardless of DINC or SINC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Cycle steal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_A {
    #[doc = "0: DMA continuously makes read/write transfers until the BCR decrements to 0."]
    _0 = 0,
    #[doc = "1: Forces a single read/write transfer per request."]
    _1 = 1,
}
impl From<CS_A> for bool {
    #[inline(always)]
    fn from(variant: CS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CS`"]
pub type CS_R = crate::R<bool, CS_A>;
impl CS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS_A {
        match self.bits {
            false => CS_A::_0,
            true => CS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CS_A::_1
    }
}
#[doc = "Write proxy for field `CS`"]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA continuously makes read/write transfers until the BCR decrements to 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CS_A::_0)
    }
    #[doc = "Forces a single read/write transfer per request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Enable peripheral request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ_A {
    #[doc = "0: Peripheral request is ignored."]
    _0 = 0,
    #[doc = "1: Enables peripheral request to initiate transfer. A software-initiated request (setting the START bit) is always enabled."]
    _1 = 1,
}
impl From<ERQ_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ`"]
pub type ERQ_R = crate::R<bool, ERQ_A>;
impl ERQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ_A {
        match self.bits {
            false => ERQ_A::_0,
            true => ERQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ_A::_1
    }
}
#[doc = "Write proxy for field `ERQ`"]
pub struct ERQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Peripheral request is ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ_A::_0)
    }
    #[doc = "Enables peripheral request to initiate transfer. A software-initiated request (setting the START bit) is always enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Enable interrupt on completion of transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT_A {
    #[doc = "0: No interrupt is generated."]
    _0 = 0,
    #[doc = "1: Interrupt signal is enabled."]
    _1 = 1,
}
impl From<EINT_A> for bool {
    #[inline(always)]
    fn from(variant: EINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EINT`"]
pub type EINT_R = crate::R<bool, EINT_A>;
impl EINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT_A {
        match self.bits {
            false => EINT_A::_0,
            true => EINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EINT_A::_1
    }
}
#[doc = "Write proxy for field `EINT`"]
pub struct EINT_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EINT_A::_0)
    }
    #[doc = "Interrupt signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EINT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Link channel 2"]
    #[inline(always)]
    pub fn lch2(&self) -> LCH2_R {
        LCH2_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Link channel 1"]
    #[inline(always)]
    pub fn lch1(&self) -> LCH1_R {
        LCH1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Link channel control"]
    #[inline(always)]
    pub fn linkcc(&self) -> LINKCC_R {
        LINKCC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Disable request"]
    #[inline(always)]
    pub fn d_req(&self) -> D_REQ_R {
        D_REQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Destination address modulo"]
    #[inline(always)]
    pub fn dmod(&self) -> DMOD_R {
        DMOD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Source address modulo"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Destination size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Destination increment"]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Source size"]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Source increment"]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable asynchronous DMA requests"]
    #[inline(always)]
    pub fn eadreq(&self) -> EADREQ_R {
        EADREQ_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Auto-align"]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Cycle steal"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable peripheral request"]
    #[inline(always)]
    pub fn erq(&self) -> ERQ_R {
        ERQ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable interrupt on completion of transfer"]
    #[inline(always)]
    pub fn eint(&self) -> EINT_R {
        EINT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Link channel 2"]
    #[inline(always)]
    pub fn lch2(&mut self) -> LCH2_W {
        LCH2_W { w: self }
    }
    #[doc = "Bits 2:3 - Link channel 1"]
    #[inline(always)]
    pub fn lch1(&mut self) -> LCH1_W {
        LCH1_W { w: self }
    }
    #[doc = "Bits 4:5 - Link channel control"]
    #[inline(always)]
    pub fn linkcc(&mut self) -> LINKCC_W {
        LINKCC_W { w: self }
    }
    #[doc = "Bit 7 - Disable request"]
    #[inline(always)]
    pub fn d_req(&mut self) -> D_REQ_W {
        D_REQ_W { w: self }
    }
    #[doc = "Bits 8:11 - Destination address modulo"]
    #[inline(always)]
    pub fn dmod(&mut self) -> DMOD_W {
        DMOD_W { w: self }
    }
    #[doc = "Bits 12:15 - Source address modulo"]
    #[inline(always)]
    pub fn smod(&mut self) -> SMOD_W {
        SMOD_W { w: self }
    }
    #[doc = "Bit 16 - Start transfer"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bits 17:18 - Destination size"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
    #[doc = "Bit 19 - Destination increment"]
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W {
        DINC_W { w: self }
    }
    #[doc = "Bits 20:21 - Source size"]
    #[inline(always)]
    pub fn ssize(&mut self) -> SSIZE_W {
        SSIZE_W { w: self }
    }
    #[doc = "Bit 22 - Source increment"]
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W {
        SINC_W { w: self }
    }
    #[doc = "Bit 23 - Enable asynchronous DMA requests"]
    #[inline(always)]
    pub fn eadreq(&mut self) -> EADREQ_W {
        EADREQ_W { w: self }
    }
    #[doc = "Bit 28 - Auto-align"]
    #[inline(always)]
    pub fn aa(&mut self) -> AA_W {
        AA_W { w: self }
    }
    #[doc = "Bit 29 - Cycle steal"]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
    #[doc = "Bit 30 - Enable peripheral request"]
    #[inline(always)]
    pub fn erq(&mut self) -> ERQ_W {
        ERQ_W { w: self }
    }
    #[doc = "Bit 31 - Enable interrupt on completion of transfer"]
    #[inline(always)]
    pub fn eint(&mut self) -> EINT_W {
        EINT_W { w: self }
    }
}
