#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SDID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `PINID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINIDR {
    #[doc = "16-pin"]
    _0000,
    #[doc = "24-pin"]
    _0001,
    #[doc = "32-pin"]
    _0010,
    #[doc = "48-pin"]
    _0100,
    #[doc = "64-pin"]
    _0101,
    #[doc = "80-pin"]
    _0110,
    #[doc = "100-pin"]
    _1000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PINIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINIDR::_0000 => 0,
            PINIDR::_0001 => 1,
            PINIDR::_0010 => 2,
            PINIDR::_0100 => 4,
            PINIDR::_0101 => 5,
            PINIDR::_0110 => 6,
            PINIDR::_1000 => 8,
            PINIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINIDR {
        match value {
            0 => PINIDR::_0000,
            1 => PINIDR::_0001,
            2 => PINIDR::_0010,
            4 => PINIDR::_0100,
            5 => PINIDR::_0101,
            6 => PINIDR::_0110,
            8 => PINIDR::_1000,
            i => PINIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == PINIDR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == PINIDR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == PINIDR::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == PINIDR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == PINIDR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == PINIDR::_0110
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == PINIDR::_1000
    }
}
#[doc = r" Value of the field"]
pub struct DIEIDR {
    bits: u8,
}
impl DIEIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REVIDR {
    bits: u8,
}
impl REVIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SRAMSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMSIZER {
    #[doc = "0.5 KB"]
    _0000,
    #[doc = "1 KB"]
    _0001,
    #[doc = "2 KB"]
    _0010,
    #[doc = "4 KB"]
    _0011,
    #[doc = "8 KB"]
    _0100,
    #[doc = "16 KB"]
    _0101,
    #[doc = "32 KB"]
    _0110,
    #[doc = "64 KB"]
    _0111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRAMSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRAMSIZER::_0000 => 0,
            SRAMSIZER::_0001 => 1,
            SRAMSIZER::_0010 => 2,
            SRAMSIZER::_0011 => 3,
            SRAMSIZER::_0100 => 4,
            SRAMSIZER::_0101 => 5,
            SRAMSIZER::_0110 => 6,
            SRAMSIZER::_0111 => 7,
            SRAMSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRAMSIZER {
        match value {
            0 => SRAMSIZER::_0000,
            1 => SRAMSIZER::_0001,
            2 => SRAMSIZER::_0010,
            3 => SRAMSIZER::_0011,
            4 => SRAMSIZER::_0100,
            5 => SRAMSIZER::_0101,
            6 => SRAMSIZER::_0110,
            7 => SRAMSIZER::_0111,
            i => SRAMSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == SRAMSIZER::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == SRAMSIZER::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == SRAMSIZER::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == SRAMSIZER::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == SRAMSIZER::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == SRAMSIZER::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == SRAMSIZER::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == SRAMSIZER::_0111
    }
}
#[doc = "Possible values of the field `SERIESID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SERIESIDR {
    #[doc = "KL family"]
    _0001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SERIESIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SERIESIDR::_0001 => 1,
            SERIESIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SERIESIDR {
        match value {
            1 => SERIESIDR::_0001,
            i => SERIESIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == SERIESIDR::_0001
    }
}
#[doc = "Possible values of the field `SUBFAMID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBFAMIDR {
    #[doc = "KLx2 Subfamily (low end)"]
    _0010,
    #[doc = "KLx4 Subfamily (basic analog)"]
    _0100,
    #[doc = "KLx5 Subfamily (advanced analog)"]
    _0101,
    #[doc = "KLx6 Subfamily (advanced analog with I2S)"]
    _0110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SUBFAMIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SUBFAMIDR::_0010 => 2,
            SUBFAMIDR::_0100 => 4,
            SUBFAMIDR::_0101 => 5,
            SUBFAMIDR::_0110 => 6,
            SUBFAMIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SUBFAMIDR {
        match value {
            2 => SUBFAMIDR::_0010,
            4 => SUBFAMIDR::_0100,
            5 => SUBFAMIDR::_0101,
            6 => SUBFAMIDR::_0110,
            i => SUBFAMIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == SUBFAMIDR::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == SUBFAMIDR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == SUBFAMIDR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == SUBFAMIDR::_0110
    }
}
#[doc = "Possible values of the field `FAMID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAMIDR {
    #[doc = "KL0x Family (low end)"]
    _0000,
    #[doc = "KL1x Family (basic)"]
    _0001,
    #[doc = "KL2x Family (USB)"]
    _0010,
    #[doc = "KL3x Family (Segment LCD)"]
    _0011,
    #[doc = "KL4x Family (USB and Segment LCD)"]
    _0100,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FAMIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FAMIDR::_0000 => 0,
            FAMIDR::_0001 => 1,
            FAMIDR::_0010 => 2,
            FAMIDR::_0011 => 3,
            FAMIDR::_0100 => 4,
            FAMIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FAMIDR {
        match value {
            0 => FAMIDR::_0000,
            1 => FAMIDR::_0001,
            2 => FAMIDR::_0010,
            3 => FAMIDR::_0011,
            4 => FAMIDR::_0100,
            i => FAMIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == FAMIDR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == FAMIDR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == FAMIDR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == FAMIDR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == FAMIDR::_0100
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline]
    pub fn pinid(&self) -> PINIDR {
        PINIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:11 - Device die number"]
    #[inline]
    pub fn dieid(&self) -> DIEIDR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIEIDR { bits }
    }
    #[doc = "Bits 12:15 - Device revision number"]
    #[inline]
    pub fn revid(&self) -> REVIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REVIDR { bits }
    }
    #[doc = "Bits 16:19 - System SRAM Size"]
    #[inline]
    pub fn sramsize(&self) -> SRAMSIZER {
        SRAMSIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Kinetis Series ID"]
    #[inline]
    pub fn seriesid(&self) -> SERIESIDR {
        SERIESIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Kinetis Sub-Family ID"]
    #[inline]
    pub fn subfamid(&self) -> SUBFAMIDR {
        SUBFAMIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Kinetis family ID"]
    #[inline]
    pub fn famid(&self) -> FAMIDR {
        FAMIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
