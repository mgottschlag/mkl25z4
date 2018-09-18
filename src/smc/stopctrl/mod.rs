#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::STOPCTRL {
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
#[doc = "Possible values of the field `VLLSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLLSMR {
    #[doc = "VLLS0"]
    _000,
    #[doc = "VLLS1"]
    _001,
    #[doc = "VLLS3"]
    _011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VLLSMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VLLSMR::_000 => 0,
            VLLSMR::_001 => 1,
            VLLSMR::_011 => 3,
            VLLSMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VLLSMR {
        match value {
            0 => VLLSMR::_000,
            1 => VLLSMR::_001,
            3 => VLLSMR::_011,
            i => VLLSMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == VLLSMR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == VLLSMR::_001
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == VLLSMR::_011
    }
}
#[doc = "Possible values of the field `PORPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORPOR {
    #[doc = "POR detect circuit is enabled in VLLS0"]
    _0,
    #[doc = "POR detect circuit is disabled in VLLS0"]
    _1,
}
impl PORPOR {
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
            PORPOR::_0 => false,
            PORPOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PORPOR {
        match value {
            false => PORPOR::_0,
            true => PORPOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PORPOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PORPOR::_1
    }
}
#[doc = "Possible values of the field `PSTOPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSTOPOR {
    #[doc = "STOP - Normal Stop mode"]
    _00,
    #[doc = "PSTOP1 - Partial Stop with both system and bus clocks disabled"]
    _01,
    #[doc = "PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PSTOPOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSTOPOR::_00 => 0,
            PSTOPOR::_01 => 1,
            PSTOPOR::_10 => 2,
            PSTOPOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSTOPOR {
        match value {
            0 => PSTOPOR::_00,
            1 => PSTOPOR::_01,
            2 => PSTOPOR::_10,
            i => PSTOPOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PSTOPOR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PSTOPOR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PSTOPOR::_10
    }
}
#[doc = "Values that can be written to the field `VLLSM`"]
pub enum VLLSMW {
    #[doc = "VLLS0"]
    _000,
    #[doc = "VLLS1"]
    _001,
    #[doc = "VLLS3"]
    _011,
}
impl VLLSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VLLSMW::_000 => 0,
            VLLSMW::_001 => 1,
            VLLSMW::_011 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VLLSMW<'a> {
    w: &'a mut W,
}
impl<'a> _VLLSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VLLSMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "VLLS0"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(VLLSMW::_000)
    }
    #[doc = "VLLS1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(VLLSMW::_001)
    }
    #[doc = "VLLS3"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(VLLSMW::_011)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PORPO`"]
pub enum PORPOW {
    #[doc = "POR detect circuit is enabled in VLLS0"]
    _0,
    #[doc = "POR detect circuit is disabled in VLLS0"]
    _1,
}
impl PORPOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PORPOW::_0 => false,
            PORPOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PORPOW<'a> {
    w: &'a mut W,
}
impl<'a> _PORPOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORPOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "POR detect circuit is enabled in VLLS0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORPOW::_0)
    }
    #[doc = "POR detect circuit is disabled in VLLS0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORPOW::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSTOPO`"]
pub enum PSTOPOW {
    #[doc = "STOP - Normal Stop mode"]
    _00,
    #[doc = "PSTOP1 - Partial Stop with both system and bus clocks disabled"]
    _01,
    #[doc = "PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
    _10,
}
impl PSTOPOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSTOPOW::_00 => 0,
            PSTOPOW::_01 => 1,
            PSTOPOW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSTOPOW<'a> {
    w: &'a mut W,
}
impl<'a> _PSTOPOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSTOPOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "STOP - Normal Stop mode"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PSTOPOW::_00)
    }
    #[doc = "PSTOP1 - Partial Stop with both system and bus clocks disabled"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PSTOPOW::_01)
    }
    #[doc = "PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PSTOPOW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:2 - VLLS Mode Control."]
    #[inline]
    pub fn vllsm(&self) -> VLLSMR {
        VLLSMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline]
    pub fn porpo(&self) -> PORPOR {
        PORPOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 6:7 - Partial Stop Option"]
    #[inline]
    pub fn pstopo(&self) -> PSTOPOR {
        PSTOPOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - VLLS Mode Control."]
    #[inline]
    pub fn vllsm(&mut self) -> _VLLSMW {
        _VLLSMW { w: self }
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline]
    pub fn porpo(&mut self) -> _PORPOW {
        _PORPOW { w: self }
    }
    #[doc = "Bits 6:7 - Partial Stop Option"]
    #[inline]
    pub fn pstopo(&mut self) -> _PSTOPOW {
        _PSTOPOW { w: self }
    }
}
