#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCT1 {
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
#[doc = "Possible values of the field `FUNCTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNCTIONR {
    #[doc = "Disabled."]
    _0000,
    #[doc = "Instruction fetch."]
    _0100,
    #[doc = "Data operand read."]
    _0101,
    #[doc = "Data operand write."]
    _0110,
    #[doc = "Data operand (read + write)."]
    _0111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCTIONR::_0000 => 0,
            FUNCTIONR::_0100 => 4,
            FUNCTIONR::_0101 => 5,
            FUNCTIONR::_0110 => 6,
            FUNCTIONR::_0111 => 7,
            FUNCTIONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCTIONR {
        match value {
            0 => FUNCTIONR::_0000,
            4 => FUNCTIONR::_0100,
            5 => FUNCTIONR::_0101,
            6 => FUNCTIONR::_0110,
            7 => FUNCTIONR::_0111,
            i => FUNCTIONR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == FUNCTIONR::_0000
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == FUNCTIONR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == FUNCTIONR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == FUNCTIONR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == FUNCTIONR::_0111
    }
}
#[doc = "Possible values of the field `MATCHED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCHEDR {
    #[doc = "No match."]
    _0,
    #[doc = "Match occurred."]
    _1,
}
impl MATCHEDR {
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
            MATCHEDR::_0 => false,
            MATCHEDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MATCHEDR {
        match value {
            false => MATCHEDR::_0,
            true => MATCHEDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MATCHEDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MATCHEDR::_1
    }
}
#[doc = "Values that can be written to the field `FUNCTION`"]
pub enum FUNCTIONW {
    #[doc = "Disabled."]
    _0000,
    #[doc = "Instruction fetch."]
    _0100,
    #[doc = "Data operand read."]
    _0101,
    #[doc = "Data operand write."]
    _0110,
    #[doc = "Data operand (read + write)."]
    _0111,
}
impl FUNCTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCTIONW::_0000 => 0,
            FUNCTIONW::_0100 => 4,
            FUNCTIONW::_0101 => 5,
            FUNCTIONW::_0110 => 6,
            FUNCTIONW::_0111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FUNCTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FUNCTIONW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(FUNCTIONW::_0000)
    }
    #[doc = "Instruction fetch."]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(FUNCTIONW::_0100)
    }
    #[doc = "Data operand read."]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(FUNCTIONW::_0101)
    }
    #[doc = "Data operand write."]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(FUNCTIONW::_0110)
    }
    #[doc = "Data operand (read + write)."]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(FUNCTIONW::_0111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - Function"]
    #[inline]
    pub fn function(&self) -> FUNCTIONR {
        FUNCTIONR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Comparator match"]
    #[inline]
    pub fn matched(&self) -> MATCHEDR {
        MATCHEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:3 - Function"]
    #[inline]
    pub fn function(&mut self) -> _FUNCTIONW {
        _FUNCTIONW { w: self }
    }
}
