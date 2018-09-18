#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCR2 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `LCH2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCH2R {
    #[doc = "DMA Channel 0"]
    _00,
    #[doc = "DMA Channel 1"]
    _01,
    #[doc = "DMA Channel 2"]
    _10,
    #[doc = "DMA Channel 3"]
    _11,
}
impl LCH2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LCH2R::_00 => 0,
            LCH2R::_01 => 1,
            LCH2R::_10 => 2,
            LCH2R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LCH2R {
        match value {
            0 => LCH2R::_00,
            1 => LCH2R::_01,
            2 => LCH2R::_10,
            3 => LCH2R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LCH2R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LCH2R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LCH2R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == LCH2R::_11
    }
}
#[doc = "Possible values of the field `LCH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCH1R {
    #[doc = "DMA Channel 0"]
    _00,
    #[doc = "DMA Channel 1"]
    _01,
    #[doc = "DMA Channel 2"]
    _10,
    #[doc = "DMA Channel 3"]
    _11,
}
impl LCH1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LCH1R::_00 => 0,
            LCH1R::_01 => 1,
            LCH1R::_10 => 2,
            LCH1R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LCH1R {
        match value {
            0 => LCH1R::_00,
            1 => LCH1R::_01,
            2 => LCH1R::_10,
            3 => LCH1R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LCH1R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LCH1R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LCH1R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == LCH1R::_11
    }
}
#[doc = "Possible values of the field `LINKCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINKCCR {
    #[doc = "No channel-to-channel linking"]
    _00,
    #[doc = "Perform a link to channel LCH1 after each cycle-steal transfer followed by a link to LCH2 after the BCR decrements to zero"]
    _01,
    #[doc = "Perform a link to channel LCH1 after each cycle-steal transfer"]
    _10,
    #[doc = "Perform a link to channel LCH1 after the BCR decrements to zero"]
    _11,
}
impl LINKCCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LINKCCR::_00 => 0,
            LINKCCR::_01 => 1,
            LINKCCR::_10 => 2,
            LINKCCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LINKCCR {
        match value {
            0 => LINKCCR::_00,
            1 => LINKCCR::_01,
            2 => LINKCCR::_10,
            3 => LINKCCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LINKCCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LINKCCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LINKCCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == LINKCCR::_11
    }
}
#[doc = "Possible values of the field `D_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D_REQR {
    #[doc = "ERQ bit is not affected."]
    _0,
    #[doc = "ERQ bit is cleared when the BCR is exhausted."]
    _1,
}
impl D_REQR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            D_REQR::_0 => false,
            D_REQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D_REQR {
        match value {
            false => D_REQR::_0,
            true => D_REQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == D_REQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == D_REQR::_1
    }
}
#[doc = "Possible values of the field `DMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMODR {
    #[doc = "Buffer disabled"]
    _0000,
    #[doc = "Circular buffer size is 16 bytes"]
    _0001,
    #[doc = "Circular buffer size is 32 bytes"]
    _0010,
    #[doc = "Circular buffer size is 64 bytes"]
    _0011,
    #[doc = "Circular buffer size is 128 bytes"]
    _0100,
    #[doc = "Circular buffer size is 256 bytes"]
    _0101,
    #[doc = "Circular buffer size is 512 bytes"]
    _0110,
    #[doc = "Circular buffer size is 1 KB"]
    _0111,
    #[doc = "Circular buffer size is 2 KB"]
    _1000,
    #[doc = "Circular buffer size is 4 KB"]
    _1001,
    #[doc = "Circular buffer size is 8 KB"]
    _1010,
    #[doc = "Circular buffer size is 16 KB"]
    _1011,
    #[doc = "Circular buffer size is 32 KB"]
    _1100,
    #[doc = "Circular buffer size is 64 KB"]
    _1101,
    #[doc = "Circular buffer size is 128 KB"]
    _1110,
    #[doc = "Circular buffer size is 256 KB"]
    _1111,
}
impl DMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMODR::_0000 => 0,
            DMODR::_0001 => 1,
            DMODR::_0010 => 2,
            DMODR::_0011 => 3,
            DMODR::_0100 => 4,
            DMODR::_0101 => 5,
            DMODR::_0110 => 6,
            DMODR::_0111 => 7,
            DMODR::_1000 => 8,
            DMODR::_1001 => 9,
            DMODR::_1010 => 10,
            DMODR::_1011 => 11,
            DMODR::_1100 => 12,
            DMODR::_1101 => 13,
            DMODR::_1110 => 14,
            DMODR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMODR {
        match value {
            0 => DMODR::_0000,
            1 => DMODR::_0001,
            2 => DMODR::_0010,
            3 => DMODR::_0011,
            4 => DMODR::_0100,
            5 => DMODR::_0101,
            6 => DMODR::_0110,
            7 => DMODR::_0111,
            8 => DMODR::_1000,
            9 => DMODR::_1001,
            10 => DMODR::_1010,
            11 => DMODR::_1011,
            12 => DMODR::_1100,
            13 => DMODR::_1101,
            14 => DMODR::_1110,
            15 => DMODR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == DMODR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == DMODR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == DMODR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == DMODR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == DMODR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == DMODR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == DMODR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == DMODR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == DMODR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == DMODR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == DMODR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == DMODR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == DMODR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == DMODR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == DMODR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == DMODR::_1111
    }
}
#[doc = "Possible values of the field `SMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMODR {
    #[doc = "Buffer disabled"]
    _0000,
    #[doc = "Circular buffer size is 16 bytes"]
    _0001,
    #[doc = "Circular buffer size is 32 bytes"]
    _0010,
    #[doc = "Circular buffer size is 64 bytes"]
    _0011,
    #[doc = "Circular buffer size is 128 bytes"]
    _0100,
    #[doc = "Circular buffer size is 256 bytes"]
    _0101,
    #[doc = "Circular buffer size is 512 bytes"]
    _0110,
    #[doc = "Circular buffer size is 1 KB"]
    _0111,
    #[doc = "Circular buffer size is 2 KB"]
    _1000,
    #[doc = "Circular buffer size is 4 KB"]
    _1001,
    #[doc = "Circular buffer size is 8 KB"]
    _1010,
    #[doc = "Circular buffer size is 16 KB"]
    _1011,
    #[doc = "Circular buffer size is 32 KB"]
    _1100,
    #[doc = "Circular buffer size is 64 KB"]
    _1101,
    #[doc = "Circular buffer size is 128 KB"]
    _1110,
    #[doc = "Circular buffer size is 256 KB"]
    _1111,
}
impl SMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMODR::_0000 => 0,
            SMODR::_0001 => 1,
            SMODR::_0010 => 2,
            SMODR::_0011 => 3,
            SMODR::_0100 => 4,
            SMODR::_0101 => 5,
            SMODR::_0110 => 6,
            SMODR::_0111 => 7,
            SMODR::_1000 => 8,
            SMODR::_1001 => 9,
            SMODR::_1010 => 10,
            SMODR::_1011 => 11,
            SMODR::_1100 => 12,
            SMODR::_1101 => 13,
            SMODR::_1110 => 14,
            SMODR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMODR {
        match value {
            0 => SMODR::_0000,
            1 => SMODR::_0001,
            2 => SMODR::_0010,
            3 => SMODR::_0011,
            4 => SMODR::_0100,
            5 => SMODR::_0101,
            6 => SMODR::_0110,
            7 => SMODR::_0111,
            8 => SMODR::_1000,
            9 => SMODR::_1001,
            10 => SMODR::_1010,
            11 => SMODR::_1011,
            12 => SMODR::_1100,
            13 => SMODR::_1101,
            14 => SMODR::_1110,
            15 => SMODR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == SMODR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == SMODR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == SMODR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == SMODR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == SMODR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == SMODR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == SMODR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == SMODR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == SMODR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == SMODR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == SMODR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == SMODR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == SMODR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == SMODR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == SMODR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == SMODR::_1111
    }
}
#[doc = "Possible values of the field `DSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSIZER {
    #[doc = "32-bit"]
    _00,
    #[doc = "8-bit"]
    _01,
    #[doc = "16-bit"]
    _10,
    #[doc = "Reserved (generates a configuration error (DSRn[CE]) if incorrectly specified at time of channel activation)"]
    _11,
}
impl DSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSIZER::_00 => 0,
            DSIZER::_01 => 1,
            DSIZER::_10 => 2,
            DSIZER::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSIZER {
        match value {
            0 => DSIZER::_00,
            1 => DSIZER::_01,
            2 => DSIZER::_10,
            3 => DSIZER::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DSIZER::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DSIZER::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DSIZER::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DSIZER::_11
    }
}
#[doc = "Possible values of the field `DINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINCR {
    #[doc = "No change to the DAR after a successful transfer."]
    _0,
    #[doc = "The DAR increments by 1, 2, 4 depending upon the size of the transfer."]
    _1,
}
impl DINCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DINCR::_0 => false,
            DINCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DINCR {
        match value {
            false => DINCR::_0,
            true => DINCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DINCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DINCR::_1
    }
}
#[doc = "Possible values of the field `SSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSIZER {
    #[doc = "32-bit"]
    _00,
    #[doc = "8-bit"]
    _01,
    #[doc = "16-bit"]
    _10,
    #[doc = "Reserved (generates a configuration error (DSRn[CE]) if incorrectly specified at time of channel activation)"]
    _11,
}
impl SSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSIZER::_00 => 0,
            SSIZER::_01 => 1,
            SSIZER::_10 => 2,
            SSIZER::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSIZER {
        match value {
            0 => SSIZER::_00,
            1 => SSIZER::_01,
            2 => SSIZER::_10,
            3 => SSIZER::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == SSIZER::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == SSIZER::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SSIZER::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SSIZER::_11
    }
}
#[doc = "Possible values of the field `SINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINCR {
    #[doc = "No change to SAR after a successful transfer."]
    _0,
    #[doc = "The SAR increments by 1, 2, 4 as determined by the transfer size."]
    _1,
}
impl SINCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SINCR::_0 => false,
            SINCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SINCR {
        match value {
            false => SINCR::_0,
            true => SINCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SINCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SINCR::_1
    }
}
#[doc = "Possible values of the field `EADREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EADREQR {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl EADREQR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EADREQR::_0 => false,
            EADREQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EADREQR {
        match value {
            false => EADREQR::_0,
            true => EADREQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EADREQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EADREQR::_1
    }
}
#[doc = "Possible values of the field `AA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AAR {
    #[doc = "Auto-align disabled"]
    _0,
    #[doc = "If SSIZE indicates a transfer no smaller than DSIZE, source accesses are auto-aligned; otherwise, destination accesses are auto-aligned. Source alignment takes precedence over destination alignment. If auto-alignment is enabled, the appropriate address register increments, regardless of DINC or SINC."]
    _1,
}
impl AAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AAR::_0 => false,
            AAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AAR {
        match value {
            false => AAR::_0,
            true => AAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AAR::_1
    }
}
#[doc = "Possible values of the field `CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSR {
    #[doc = "DMA continuously makes read/write transfers until the BCR decrements to 0."]
    _0,
    #[doc = "Forces a single read/write transfer per request."]
    _1,
}
impl CSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CSR::_0 => false,
            CSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSR {
        match value {
            false => CSR::_0,
            true => CSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CSR::_1
    }
}
#[doc = "Possible values of the field `ERQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQR {
    #[doc = "Peripheral request is ignored."]
    _0,
    #[doc = "Enables peripheral request to initiate transfer. A software-initiated request (setting the START bit) is always enabled."]
    _1,
}
impl ERQR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ERQR::_0 => false,
            ERQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQR {
        match value {
            false => ERQR::_0,
            true => ERQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQR::_1
    }
}
#[doc = "Possible values of the field `EINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINTR {
    #[doc = "No interrupt is generated."]
    _0,
    #[doc = "Interrupt signal is enabled."]
    _1,
}
impl EINTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EINTR::_0 => false,
            EINTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EINTR {
        match value {
            false => EINTR::_0,
            true => EINTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EINTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EINTR::_1
    }
}
#[doc = "Values that can be written to the field `LCH2`"]
pub enum LCH2W {
    #[doc = "DMA Channel 0"]
    _00,
    #[doc = "DMA Channel 1"]
    _01,
    #[doc = "DMA Channel 2"]
    _10,
    #[doc = "DMA Channel 3"]
    _11,
}
impl LCH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LCH2W::_00 => 0,
            LCH2W::_01 => 1,
            LCH2W::_10 => 2,
            LCH2W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCH2W<'a> {
    w: &'a mut W,
}
impl<'a> _LCH2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCH2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DMA Channel 0"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LCH2W::_00)
    }
    #[doc = "DMA Channel 1"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LCH2W::_01)
    }
    #[doc = "DMA Channel 2"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LCH2W::_10)
    }
    #[doc = "DMA Channel 3"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(LCH2W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCH1`"]
pub enum LCH1W {
    #[doc = "DMA Channel 0"]
    _00,
    #[doc = "DMA Channel 1"]
    _01,
    #[doc = "DMA Channel 2"]
    _10,
    #[doc = "DMA Channel 3"]
    _11,
}
impl LCH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LCH1W::_00 => 0,
            LCH1W::_01 => 1,
            LCH1W::_10 => 2,
            LCH1W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCH1W<'a> {
    w: &'a mut W,
}
impl<'a> _LCH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCH1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DMA Channel 0"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LCH1W::_00)
    }
    #[doc = "DMA Channel 1"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LCH1W::_01)
    }
    #[doc = "DMA Channel 2"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LCH1W::_10)
    }
    #[doc = "DMA Channel 3"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(LCH1W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LINKCC`"]
pub enum LINKCCW {
    #[doc = "No channel-to-channel linking"]
    _00,
    #[doc = "Perform a link to channel LCH1 after each cycle-steal transfer followed by a link to LCH2 after the BCR decrements to zero"]
    _01,
    #[doc = "Perform a link to channel LCH1 after each cycle-steal transfer"]
    _10,
    #[doc = "Perform a link to channel LCH1 after the BCR decrements to zero"]
    _11,
}
impl LINKCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LINKCCW::_00 => 0,
            LINKCCW::_01 => 1,
            LINKCCW::_10 => 2,
            LINKCCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINKCCW<'a> {
    w: &'a mut W,
}
impl<'a> _LINKCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINKCCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No channel-to-channel linking"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LINKCCW::_00)
    }
    #[doc = "Perform a link to channel LCH1 after each cycle-steal transfer followed by a link to LCH2 after the BCR decrements to zero"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LINKCCW::_01)
    }
    #[doc = "Perform a link to channel LCH1 after each cycle-steal transfer"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LINKCCW::_10)
    }
    #[doc = "Perform a link to channel LCH1 after the BCR decrements to zero"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(LINKCCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `D_REQ`"]
pub enum D_REQW {
    #[doc = "ERQ bit is not affected."]
    _0,
    #[doc = "ERQ bit is cleared when the BCR is exhausted."]
    _1,
}
impl D_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D_REQW::_0 => false,
            D_REQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _D_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ERQ bit is not affected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(D_REQW::_0)
    }
    #[doc = "ERQ bit is cleared when the BCR is exhausted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(D_REQW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMOD`"]
pub enum DMODW {
    #[doc = "Buffer disabled"]
    _0000,
    #[doc = "Circular buffer size is 16 bytes"]
    _0001,
    #[doc = "Circular buffer size is 32 bytes"]
    _0010,
    #[doc = "Circular buffer size is 64 bytes"]
    _0011,
    #[doc = "Circular buffer size is 128 bytes"]
    _0100,
    #[doc = "Circular buffer size is 256 bytes"]
    _0101,
    #[doc = "Circular buffer size is 512 bytes"]
    _0110,
    #[doc = "Circular buffer size is 1 KB"]
    _0111,
    #[doc = "Circular buffer size is 2 KB"]
    _1000,
    #[doc = "Circular buffer size is 4 KB"]
    _1001,
    #[doc = "Circular buffer size is 8 KB"]
    _1010,
    #[doc = "Circular buffer size is 16 KB"]
    _1011,
    #[doc = "Circular buffer size is 32 KB"]
    _1100,
    #[doc = "Circular buffer size is 64 KB"]
    _1101,
    #[doc = "Circular buffer size is 128 KB"]
    _1110,
    #[doc = "Circular buffer size is 256 KB"]
    _1111,
}
impl DMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMODW::_0000 => 0,
            DMODW::_0001 => 1,
            DMODW::_0010 => 2,
            DMODW::_0011 => 3,
            DMODW::_0100 => 4,
            DMODW::_0101 => 5,
            DMODW::_0110 => 6,
            DMODW::_0111 => 7,
            DMODW::_1000 => 8,
            DMODW::_1001 => 9,
            DMODW::_1010 => 10,
            DMODW::_1011 => 11,
            DMODW::_1100 => 12,
            DMODW::_1101 => 13,
            DMODW::_1110 => 14,
            DMODW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMODW<'a> {
    w: &'a mut W,
}
impl<'a> _DMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Buffer disabled"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DMODW::_0000)
    }
    #[doc = "Circular buffer size is 16 bytes"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DMODW::_0001)
    }
    #[doc = "Circular buffer size is 32 bytes"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DMODW::_0010)
    }
    #[doc = "Circular buffer size is 64 bytes"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DMODW::_0011)
    }
    #[doc = "Circular buffer size is 128 bytes"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DMODW::_0100)
    }
    #[doc = "Circular buffer size is 256 bytes"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DMODW::_0101)
    }
    #[doc = "Circular buffer size is 512 bytes"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DMODW::_0110)
    }
    #[doc = "Circular buffer size is 1 KB"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DMODW::_0111)
    }
    #[doc = "Circular buffer size is 2 KB"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(DMODW::_1000)
    }
    #[doc = "Circular buffer size is 4 KB"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(DMODW::_1001)
    }
    #[doc = "Circular buffer size is 8 KB"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(DMODW::_1010)
    }
    #[doc = "Circular buffer size is 16 KB"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(DMODW::_1011)
    }
    #[doc = "Circular buffer size is 32 KB"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(DMODW::_1100)
    }
    #[doc = "Circular buffer size is 64 KB"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(DMODW::_1101)
    }
    #[doc = "Circular buffer size is 128 KB"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DMODW::_1110)
    }
    #[doc = "Circular buffer size is 256 KB"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(DMODW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMOD`"]
pub enum SMODW {
    #[doc = "Buffer disabled"]
    _0000,
    #[doc = "Circular buffer size is 16 bytes"]
    _0001,
    #[doc = "Circular buffer size is 32 bytes"]
    _0010,
    #[doc = "Circular buffer size is 64 bytes"]
    _0011,
    #[doc = "Circular buffer size is 128 bytes"]
    _0100,
    #[doc = "Circular buffer size is 256 bytes"]
    _0101,
    #[doc = "Circular buffer size is 512 bytes"]
    _0110,
    #[doc = "Circular buffer size is 1 KB"]
    _0111,
    #[doc = "Circular buffer size is 2 KB"]
    _1000,
    #[doc = "Circular buffer size is 4 KB"]
    _1001,
    #[doc = "Circular buffer size is 8 KB"]
    _1010,
    #[doc = "Circular buffer size is 16 KB"]
    _1011,
    #[doc = "Circular buffer size is 32 KB"]
    _1100,
    #[doc = "Circular buffer size is 64 KB"]
    _1101,
    #[doc = "Circular buffer size is 128 KB"]
    _1110,
    #[doc = "Circular buffer size is 256 KB"]
    _1111,
}
impl SMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMODW::_0000 => 0,
            SMODW::_0001 => 1,
            SMODW::_0010 => 2,
            SMODW::_0011 => 3,
            SMODW::_0100 => 4,
            SMODW::_0101 => 5,
            SMODW::_0110 => 6,
            SMODW::_0111 => 7,
            SMODW::_1000 => 8,
            SMODW::_1001 => 9,
            SMODW::_1010 => 10,
            SMODW::_1011 => 11,
            SMODW::_1100 => 12,
            SMODW::_1101 => 13,
            SMODW::_1110 => 14,
            SMODW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMODW<'a> {
    w: &'a mut W,
}
impl<'a> _SMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Buffer disabled"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(SMODW::_0000)
    }
    #[doc = "Circular buffer size is 16 bytes"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(SMODW::_0001)
    }
    #[doc = "Circular buffer size is 32 bytes"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(SMODW::_0010)
    }
    #[doc = "Circular buffer size is 64 bytes"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(SMODW::_0011)
    }
    #[doc = "Circular buffer size is 128 bytes"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(SMODW::_0100)
    }
    #[doc = "Circular buffer size is 256 bytes"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(SMODW::_0101)
    }
    #[doc = "Circular buffer size is 512 bytes"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(SMODW::_0110)
    }
    #[doc = "Circular buffer size is 1 KB"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(SMODW::_0111)
    }
    #[doc = "Circular buffer size is 2 KB"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(SMODW::_1000)
    }
    #[doc = "Circular buffer size is 4 KB"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(SMODW::_1001)
    }
    #[doc = "Circular buffer size is 8 KB"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(SMODW::_1010)
    }
    #[doc = "Circular buffer size is 16 KB"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(SMODW::_1011)
    }
    #[doc = "Circular buffer size is 32 KB"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(SMODW::_1100)
    }
    #[doc = "Circular buffer size is 64 KB"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(SMODW::_1101)
    }
    #[doc = "Circular buffer size is 128 KB"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(SMODW::_1110)
    }
    #[doc = "Circular buffer size is 256 KB"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(SMODW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `START`"]
pub enum STARTW {
    #[doc = "DMA inactive"]
    _0,
    #[doc = "The DMA begins the transfer in accordance to the values in the TCDn. START is cleared automatically after one module clock and always reads as logic 0."]
    _1,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTW::_0 => false,
            STARTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA inactive"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARTW::_0)
    }
    #[doc = "The DMA begins the transfer in accordance to the values in the TCDn. START is cleared automatically after one module clock and always reads as logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STARTW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DSIZE`"]
pub enum DSIZEW {
    #[doc = "32-bit"]
    _00,
    #[doc = "8-bit"]
    _01,
    #[doc = "16-bit"]
    _10,
    #[doc = "Reserved (generates a configuration error (DSRn[CE]) if incorrectly specified at time of channel activation)"]
    _11,
}
impl DSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSIZEW::_00 => 0,
            DSIZEW::_01 => 1,
            DSIZEW::_10 => 2,
            DSIZEW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "32-bit"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DSIZEW::_00)
    }
    #[doc = "8-bit"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DSIZEW::_01)
    }
    #[doc = "16-bit"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DSIZEW::_10)
    }
    #[doc = "Reserved (generates a configuration error (DSRn[CE]) if incorrectly specified at time of channel activation)"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DSIZEW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DINC`"]
pub enum DINCW {
    #[doc = "No change to the DAR after a successful transfer."]
    _0,
    #[doc = "The DAR increments by 1, 2, 4 depending upon the size of the transfer."]
    _1,
}
impl DINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DINCW::_0 => false,
            DINCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DINCW<'a> {
    w: &'a mut W,
}
impl<'a> _DINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change to the DAR after a successful transfer."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINCW::_0)
    }
    #[doc = "The DAR increments by 1, 2, 4 depending upon the size of the transfer."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINCW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SSIZE`"]
pub enum SSIZEW {
    #[doc = "32-bit"]
    _00,
    #[doc = "8-bit"]
    _01,
    #[doc = "16-bit"]
    _10,
    #[doc = "Reserved (generates a configuration error (DSRn[CE]) if incorrectly specified at time of channel activation)"]
    _11,
}
impl SSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSIZEW::_00 => 0,
            SSIZEW::_01 => 1,
            SSIZEW::_10 => 2,
            SSIZEW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "32-bit"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(SSIZEW::_00)
    }
    #[doc = "8-bit"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(SSIZEW::_01)
    }
    #[doc = "16-bit"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SSIZEW::_10)
    }
    #[doc = "Reserved (generates a configuration error (DSRn[CE]) if incorrectly specified at time of channel activation)"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(SSIZEW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SINC`"]
pub enum SINCW {
    #[doc = "No change to SAR after a successful transfer."]
    _0,
    #[doc = "The SAR increments by 1, 2, 4 as determined by the transfer size."]
    _1,
}
impl SINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SINCW::_0 => false,
            SINCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SINCW<'a> {
    w: &'a mut W,
}
impl<'a> _SINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change to SAR after a successful transfer."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SINCW::_0)
    }
    #[doc = "The SAR increments by 1, 2, 4 as determined by the transfer size."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SINCW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EADREQ`"]
pub enum EADREQW {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl EADREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EADREQW::_0 => false,
            EADREQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EADREQW<'a> {
    w: &'a mut W,
}
impl<'a> _EADREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EADREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EADREQW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EADREQW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AA`"]
pub enum AAW {
    #[doc = "Auto-align disabled"]
    _0,
    #[doc = "If SSIZE indicates a transfer no smaller than DSIZE, source accesses are auto-aligned; otherwise, destination accesses are auto-aligned. Source alignment takes precedence over destination alignment. If auto-alignment is enabled, the appropriate address register increments, regardless of DINC or SINC."]
    _1,
}
impl AAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AAW::_0 => false,
            AAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AAW<'a> {
    w: &'a mut W,
}
impl<'a> _AAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto-align disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AAW::_0)
    }
    #[doc = "If SSIZE indicates a transfer no smaller than DSIZE, source accesses are auto-aligned; otherwise, destination accesses are auto-aligned. Source alignment takes precedence over destination alignment. If auto-alignment is enabled, the appropriate address register increments, regardless of DINC or SINC."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AAW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CS`"]
pub enum CSW {
    #[doc = "DMA continuously makes read/write transfers until the BCR decrements to 0."]
    _0,
    #[doc = "Forces a single read/write transfer per request."]
    _1,
}
impl CSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSW::_0 => false,
            CSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSW<'a> {
    w: &'a mut W,
}
impl<'a> _CSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA continuously makes read/write transfers until the BCR decrements to 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSW::_0)
    }
    #[doc = "Forces a single read/write transfer per request."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ`"]
pub enum ERQW {
    #[doc = "Peripheral request is ignored."]
    _0,
    #[doc = "Enables peripheral request to initiate transfer. A software-initiated request (setting the START bit) is always enabled."]
    _1,
}
impl ERQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQW::_0 => false,
            ERQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQW<'a> {
    w: &'a mut W,
}
impl<'a> _ERQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral request is ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQW::_0)
    }
    #[doc = "Enables peripheral request to initiate transfer. A software-initiated request (setting the START bit) is always enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EINT`"]
pub enum EINTW {
    #[doc = "No interrupt is generated."]
    _0,
    #[doc = "Interrupt signal is enabled."]
    _1,
}
impl EINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EINTW::_0 => false,
            EINTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EINTW<'a> {
    w: &'a mut W,
}
impl<'a> _EINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt is generated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EINTW::_0)
    }
    #[doc = "Interrupt signal is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EINTW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Link channel 2"]
    #[inline]
    pub fn lch2(&self) -> LCH2R {
        LCH2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Link channel 1"]
    #[inline]
    pub fn lch1(&self) -> LCH1R {
        LCH1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Link channel control"]
    #[inline]
    pub fn linkcc(&self) -> LINKCCR {
        LINKCCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Disable request"]
    #[inline]
    pub fn d_req(&self) -> D_REQR {
        D_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Destination address modulo"]
    #[inline]
    pub fn dmod(&self) -> DMODR {
        DMODR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Source address modulo"]
    #[inline]
    pub fn smod(&self) -> SMODR {
        SMODR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:18 - Destination size"]
    #[inline]
    pub fn dsize(&self) -> DSIZER {
        DSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Destination increment"]
    #[inline]
    pub fn dinc(&self) -> DINCR {
        DINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 20:21 - Source size"]
    #[inline]
    pub fn ssize(&self) -> SSIZER {
        SSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Source increment"]
    #[inline]
    pub fn sinc(&self) -> SINCR {
        SINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enable asynchronous DMA requests"]
    #[inline]
    pub fn eadreq(&self) -> EADREQR {
        EADREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Auto-align"]
    #[inline]
    pub fn aa(&self) -> AAR {
        AAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Cycle steal"]
    #[inline]
    pub fn cs(&self) -> CSR {
        CSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Enable peripheral request"]
    #[inline]
    pub fn erq(&self) -> ERQR {
        ERQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enable interrupt on completion of transfer"]
    #[inline]
    pub fn eint(&self) -> EINTR {
        EINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Link channel 2"]
    #[inline]
    pub fn lch2(&mut self) -> _LCH2W {
        _LCH2W { w: self }
    }
    #[doc = "Bits 2:3 - Link channel 1"]
    #[inline]
    pub fn lch1(&mut self) -> _LCH1W {
        _LCH1W { w: self }
    }
    #[doc = "Bits 4:5 - Link channel control"]
    #[inline]
    pub fn linkcc(&mut self) -> _LINKCCW {
        _LINKCCW { w: self }
    }
    #[doc = "Bit 7 - Disable request"]
    #[inline]
    pub fn d_req(&mut self) -> _D_REQW {
        _D_REQW { w: self }
    }
    #[doc = "Bits 8:11 - Destination address modulo"]
    #[inline]
    pub fn dmod(&mut self) -> _DMODW {
        _DMODW { w: self }
    }
    #[doc = "Bits 12:15 - Source address modulo"]
    #[inline]
    pub fn smod(&mut self) -> _SMODW {
        _SMODW { w: self }
    }
    #[doc = "Bit 16 - Start transfer"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bits 17:18 - Destination size"]
    #[inline]
    pub fn dsize(&mut self) -> _DSIZEW {
        _DSIZEW { w: self }
    }
    #[doc = "Bit 19 - Destination increment"]
    #[inline]
    pub fn dinc(&mut self) -> _DINCW {
        _DINCW { w: self }
    }
    #[doc = "Bits 20:21 - Source size"]
    #[inline]
    pub fn ssize(&mut self) -> _SSIZEW {
        _SSIZEW { w: self }
    }
    #[doc = "Bit 22 - Source increment"]
    #[inline]
    pub fn sinc(&mut self) -> _SINCW {
        _SINCW { w: self }
    }
    #[doc = "Bit 23 - Enable asynchronous DMA requests"]
    #[inline]
    pub fn eadreq(&mut self) -> _EADREQW {
        _EADREQW { w: self }
    }
    #[doc = "Bit 28 - Auto-align"]
    #[inline]
    pub fn aa(&mut self) -> _AAW {
        _AAW { w: self }
    }
    #[doc = "Bit 29 - Cycle steal"]
    #[inline]
    pub fn cs(&mut self) -> _CSW {
        _CSW { w: self }
    }
    #[doc = "Bit 30 - Enable peripheral request"]
    #[inline]
    pub fn erq(&mut self) -> _ERQW {
        _ERQW { w: self }
    }
    #[doc = "Bit 31 - Enable interrupt on completion of transfer"]
    #[inline]
    pub fn eint(&mut self) -> _EINTW {
        _EINTW { w: self }
    }
}
