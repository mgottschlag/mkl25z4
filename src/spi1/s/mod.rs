#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::S {
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
#[doc = "Possible values of the field `MODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODFR {
    #[doc = "No mode fault error"]
    _0,
    #[doc = "Mode fault error detected"]
    _1,
}
impl MODFR {
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
            MODFR::_0 => false,
            MODFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODFR {
        match value {
            false => MODFR::_0,
            true => MODFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MODFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MODFR::_1
    }
}
#[doc = "Possible values of the field `SPTEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPTEFR {
    #[doc = "SPI transmit buffer not empty"]
    _0,
    #[doc = "SPI transmit buffer empty"]
    _1,
}
impl SPTEFR {
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
            SPTEFR::_0 => false,
            SPTEFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPTEFR {
        match value {
            false => SPTEFR::_0,
            true => SPTEFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPTEFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPTEFR::_1
    }
}
#[doc = "Possible values of the field `SPMF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPMFR {
    #[doc = "Value in the receive data buffer does not match the value in the M register"]
    _0,
    #[doc = "Value in the receive data buffer matches the value in the M register"]
    _1,
}
impl SPMFR {
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
            SPMFR::_0 => false,
            SPMFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPMFR {
        match value {
            false => SPMFR::_0,
            true => SPMFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPMFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPMFR::_1
    }
}
#[doc = "Possible values of the field `SPRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPRFR {
    #[doc = "No data available in the receive data buffer"]
    _0,
    #[doc = "Data available in the receive data buffer"]
    _1,
}
impl SPRFR {
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
            SPRFR::_0 => false,
            SPRFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPRFR {
        match value {
            false => SPRFR::_0,
            true => SPRFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPRFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPRFR::_1
    }
}
#[doc = "Values that can be written to the field `SPMF`"]
pub enum SPMFW {
    #[doc = "Value in the receive data buffer does not match the value in the M register"]
    _0,
    #[doc = "Value in the receive data buffer matches the value in the M register"]
    _1,
}
impl SPMFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPMFW::_0 => false,
            SPMFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPMFW<'a> {
    w: &'a mut W,
}
impl<'a> _SPMFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPMFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Value in the receive data buffer does not match the value in the M register"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPMFW::_0)
    }
    #[doc = "Value in the receive data buffer matches the value in the M register"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPMFW::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 4 - Master mode fault flag"]
    #[inline]
    pub fn modf(&self) -> MODFR {
        MODFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - SPI transmit buffer empty flag"]
    #[inline]
    pub fn sptef(&self) -> SPTEFR {
        SPTEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - SPI match flag"]
    #[inline]
    pub fn spmf(&self) -> SPMFR {
        SPMFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - SPI read buffer full flag"]
    #[inline]
    pub fn sprf(&self) -> SPRFR {
        SPRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 6 - SPI match flag"]
    #[inline]
    pub fn spmf(&mut self) -> _SPMFW {
        _SPMFW { w: self }
    }
}
