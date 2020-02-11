#[doc = "Reader of register SDID"]
pub type R = crate::R<u32, super::SDID>;
#[doc = "Pincount identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINID_A {
    #[doc = "0: 16-pin"]
    _0000 = 0,
    #[doc = "1: 24-pin"]
    _0001 = 1,
    #[doc = "2: 32-pin"]
    _0010 = 2,
    #[doc = "4: 48-pin"]
    _0100 = 4,
    #[doc = "5: 64-pin"]
    _0101 = 5,
    #[doc = "6: 80-pin"]
    _0110 = 6,
    #[doc = "8: 100-pin"]
    _1000 = 8,
}
impl From<PINID_A> for u8 {
    #[inline(always)]
    fn from(variant: PINID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PINID`"]
pub type PINID_R = crate::R<u8, PINID_A>;
impl PINID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PINID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PINID_A::_0000),
            1 => Val(PINID_A::_0001),
            2 => Val(PINID_A::_0010),
            4 => Val(PINID_A::_0100),
            5 => Val(PINID_A::_0101),
            6 => Val(PINID_A::_0110),
            8 => Val(PINID_A::_1000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == PINID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == PINID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == PINID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PINID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PINID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PINID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PINID_A::_1000
    }
}
#[doc = "Reader of field `DIEID`"]
pub type DIEID_R = crate::R<u8, u8>;
#[doc = "Reader of field `REVID`"]
pub type REVID_R = crate::R<u8, u8>;
#[doc = "System SRAM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAMSIZE_A {
    #[doc = "0: 0.5 KB"]
    _0000 = 0,
    #[doc = "1: 1 KB"]
    _0001 = 1,
    #[doc = "2: 2 KB"]
    _0010 = 2,
    #[doc = "3: 4 KB"]
    _0011 = 3,
    #[doc = "4: 8 KB"]
    _0100 = 4,
    #[doc = "5: 16 KB"]
    _0101 = 5,
    #[doc = "6: 32 KB"]
    _0110 = 6,
    #[doc = "7: 64 KB"]
    _0111 = 7,
}
impl From<SRAMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRAMSIZE`"]
pub type SRAMSIZE_R = crate::R<u8, SRAMSIZE_A>;
impl SRAMSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRAMSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRAMSIZE_A::_0000),
            1 => Val(SRAMSIZE_A::_0001),
            2 => Val(SRAMSIZE_A::_0010),
            3 => Val(SRAMSIZE_A::_0011),
            4 => Val(SRAMSIZE_A::_0100),
            5 => Val(SRAMSIZE_A::_0101),
            6 => Val(SRAMSIZE_A::_0110),
            7 => Val(SRAMSIZE_A::_0111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SRAMSIZE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SRAMSIZE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SRAMSIZE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SRAMSIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == SRAMSIZE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == SRAMSIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SRAMSIZE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == SRAMSIZE_A::_0111
    }
}
#[doc = "Kinetis Series ID\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SERIESID_A {
    #[doc = "1: KL family"]
    _0001 = 1,
}
impl From<SERIESID_A> for u8 {
    #[inline(always)]
    fn from(variant: SERIESID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SERIESID`"]
pub type SERIESID_R = crate::R<u8, SERIESID_A>;
impl SERIESID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SERIESID_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SERIESID_A::_0001),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SERIESID_A::_0001
    }
}
#[doc = "Kinetis Sub-Family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBFAMID_A {
    #[doc = "2: KLx2 Subfamily (low end)"]
    _0010 = 2,
    #[doc = "4: KLx4 Subfamily (basic analog)"]
    _0100 = 4,
    #[doc = "5: KLx5 Subfamily (advanced analog)"]
    _0101 = 5,
    #[doc = "6: KLx6 Subfamily (advanced analog with I2S)"]
    _0110 = 6,
}
impl From<SUBFAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBFAMID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBFAMID`"]
pub type SUBFAMID_R = crate::R<u8, SUBFAMID_A>;
impl SUBFAMID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBFAMID_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(SUBFAMID_A::_0010),
            4 => Val(SUBFAMID_A::_0100),
            5 => Val(SUBFAMID_A::_0101),
            6 => Val(SUBFAMID_A::_0110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SUBFAMID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == SUBFAMID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == SUBFAMID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SUBFAMID_A::_0110
    }
}
#[doc = "Kinetis family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FAMID_A {
    #[doc = "0: KL0x Family (low end)"]
    _0000 = 0,
    #[doc = "1: KL1x Family (basic)"]
    _0001 = 1,
    #[doc = "2: KL2x Family (USB)"]
    _0010 = 2,
    #[doc = "3: KL3x Family (Segment LCD)"]
    _0011 = 3,
    #[doc = "4: KL4x Family (USB and Segment LCD)"]
    _0100 = 4,
}
impl From<FAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FAMID`"]
pub type FAMID_R = crate::R<u8, FAMID_A>;
impl FAMID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FAMID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FAMID_A::_0000),
            1 => Val(FAMID_A::_0001),
            2 => Val(FAMID_A::_0010),
            3 => Val(FAMID_A::_0011),
            4 => Val(FAMID_A::_0100),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == FAMID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == FAMID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == FAMID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == FAMID_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == FAMID_A::_0100
    }
}
impl R {
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline(always)]
    pub fn pinid(&self) -> PINID_R {
        PINID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 7:11 - Device die number"]
    #[inline(always)]
    pub fn dieid(&self) -> DIEID_R {
        DIEID_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Device revision number"]
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - System SRAM Size"]
    #[inline(always)]
    pub fn sramsize(&self) -> SRAMSIZE_R {
        SRAMSIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Kinetis Series ID"]
    #[inline(always)]
    pub fn seriesid(&self) -> SERIESID_R {
        SERIESID_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Kinetis Sub-Family ID"]
    #[inline(always)]
    pub fn subfamid(&self) -> SUBFAMID_R {
        SUBFAMID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Kinetis family ID"]
    #[inline(always)]
    pub fn famid(&self) -> FAMID_R {
        FAMID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
