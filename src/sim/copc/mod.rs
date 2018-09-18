#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::COPC {
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
#[doc = "Possible values of the field `COPW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPWR {
    #[doc = "Normal mode"]
    _0,
    #[doc = "Windowed mode"]
    _1,
}
impl COPWR {
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
            COPWR::_0 => false,
            COPWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COPWR {
        match value {
            false => COPWR::_0,
            true => COPWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COPWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COPWR::_1
    }
}
#[doc = "Possible values of the field `COPCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPCLKSR {
    #[doc = "Internal 1 kHz clock is source to COP"]
    _0,
    #[doc = "Bus clock is source to COP"]
    _1,
}
impl COPCLKSR {
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
            COPCLKSR::_0 => false,
            COPCLKSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COPCLKSR {
        match value {
            false => COPCLKSR::_0,
            true => COPCLKSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COPCLKSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COPCLKSR::_1
    }
}
#[doc = "Possible values of the field `COPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPTR {
    #[doc = "COP disabled"]
    _00,
    #[doc = "COP timeout after 2^5 LPO cycles or 213 bus clock cycles"]
    _01,
    #[doc = "COP timeout after 2^8 LPO cycles or 216 bus clock cycles"]
    _10,
    #[doc = "COP timeout after 2^10 LPO cycles or 218 bus clock cycles"]
    _11,
}
impl COPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COPTR::_00 => 0,
            COPTR::_01 => 1,
            COPTR::_10 => 2,
            COPTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COPTR {
        match value {
            0 => COPTR::_00,
            1 => COPTR::_01,
            2 => COPTR::_10,
            3 => COPTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == COPTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == COPTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == COPTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == COPTR::_11
    }
}
#[doc = "Values that can be written to the field `COPW`"]
pub enum COPWW {
    #[doc = "Normal mode"]
    _0,
    #[doc = "Windowed mode"]
    _1,
}
impl COPWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COPWW::_0 => false,
            COPWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COPWW<'a> {
    w: &'a mut W,
}
impl<'a> _COPWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COPWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COPWW::_0)
    }
    #[doc = "Windowed mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COPWW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COPCLKS`"]
pub enum COPCLKSW {
    #[doc = "Internal 1 kHz clock is source to COP"]
    _0,
    #[doc = "Bus clock is source to COP"]
    _1,
}
impl COPCLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COPCLKSW::_0 => false,
            COPCLKSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COPCLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _COPCLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COPCLKSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal 1 kHz clock is source to COP"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COPCLKSW::_0)
    }
    #[doc = "Bus clock is source to COP"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COPCLKSW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COPT`"]
pub enum COPTW {
    #[doc = "COP disabled"]
    _00,
    #[doc = "COP timeout after 2^5 LPO cycles or 213 bus clock cycles"]
    _01,
    #[doc = "COP timeout after 2^8 LPO cycles or 216 bus clock cycles"]
    _10,
    #[doc = "COP timeout after 2^10 LPO cycles or 218 bus clock cycles"]
    _11,
}
impl COPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COPTW::_00 => 0,
            COPTW::_01 => 1,
            COPTW::_10 => 2,
            COPTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COPTW<'a> {
    w: &'a mut W,
}
impl<'a> _COPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COPTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "COP disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(COPTW::_00)
    }
    #[doc = "COP timeout after 2^5 LPO cycles or 213 bus clock cycles"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(COPTW::_01)
    }
    #[doc = "COP timeout after 2^8 LPO cycles or 216 bus clock cycles"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(COPTW::_10)
    }
    #[doc = "COP timeout after 2^10 LPO cycles or 218 bus clock cycles"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(COPTW::_11)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - COP windowed mode"]
    #[inline]
    pub fn copw(&self) -> COPWR {
        COPWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - COP Clock Select"]
    #[inline]
    pub fn copclks(&self) -> COPCLKSR {
        COPCLKSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - COP Watchdog Timeout"]
    #[inline]
    pub fn copt(&self) -> COPTR {
        COPTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 12 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - COP windowed mode"]
    #[inline]
    pub fn copw(&mut self) -> _COPWW {
        _COPWW { w: self }
    }
    #[doc = "Bit 1 - COP Clock Select"]
    #[inline]
    pub fn copclks(&mut self) -> _COPCLKSW {
        _COPCLKSW { w: self }
    }
    #[doc = "Bits 2:3 - COP Watchdog Timeout"]
    #[inline]
    pub fn copt(&mut self) -> _COPTW {
        _COPTW { w: self }
    }
}
