#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TBCTRL {
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
#[doc = "Possible values of the field `ACOMP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACOMP0R {
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT0[MATCHED]."]
    _0,
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT0[MATCHED]."]
    _1,
}
impl ACOMP0R {
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
            ACOMP0R::_0 => false,
            ACOMP0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACOMP0R {
        match value {
            false => ACOMP0R::_0,
            true => ACOMP0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ACOMP0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ACOMP0R::_1
    }
}
#[doc = "Possible values of the field `ACOMP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACOMP1R {
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT1[MATCHED]."]
    _0,
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT1[MATCHED]."]
    _1,
}
impl ACOMP1R {
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
            ACOMP1R::_0 => false,
            ACOMP1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACOMP1R {
        match value {
            false => ACOMP1R::_0,
            true => ACOMP1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ACOMP1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ACOMP1R::_1
    }
}
#[doc = r" Value of the field"]
pub struct NUMCOMPR {
    bits: u8,
}
impl NUMCOMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ACOMP0`"]
pub enum ACOMP0W {
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT0[MATCHED]."]
    _0,
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT0[MATCHED]."]
    _1,
}
impl ACOMP0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACOMP0W::_0 => false,
            ACOMP0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACOMP0W<'a> {
    w: &'a mut W,
}
impl<'a> _ACOMP0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACOMP0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT0[MATCHED]."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACOMP0W::_0)
    }
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT0[MATCHED]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACOMP0W::_1)
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
#[doc = "Values that can be written to the field `ACOMP1`"]
pub enum ACOMP1W {
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT1[MATCHED]."]
    _0,
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT1[MATCHED]."]
    _1,
}
impl ACOMP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACOMP1W::_0 => false,
            ACOMP1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACOMP1W<'a> {
    w: &'a mut W,
}
impl<'a> _ACOMP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACOMP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT1[MATCHED]."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACOMP1W::_0)
    }
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT1[MATCHED]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACOMP1W::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Action based on Comparator 0 match"]
    #[inline]
    pub fn acomp0(&self) -> ACOMP0R {
        ACOMP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Action based on Comparator 1 match"]
    #[inline]
    pub fn acomp1(&self) -> ACOMP1R {
        ACOMP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:31 - Number of Comparators"]
    #[inline]
    pub fn numcomp(&self) -> NUMCOMPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUMCOMPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 536870912 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Action based on Comparator 0 match"]
    #[inline]
    pub fn acomp0(&mut self) -> _ACOMP0W {
        _ACOMP0W { w: self }
    }
    #[doc = "Bit 1 - Action based on Comparator 1 match"]
    #[inline]
    pub fn acomp1(&mut self) -> _ACOMP1W {
        _ACOMP1W { w: self }
    }
}
