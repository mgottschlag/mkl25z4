#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C5 {
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
#[doc = "Possible values of the field `RESYNCDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESYNCDISR {
    #[doc = "Resynchronization during received data word is supported"]
    _0,
    #[doc = "Resynchronization during received data word is disabled"]
    _1,
}
impl RESYNCDISR {
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
            RESYNCDISR::_0 => false,
            RESYNCDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESYNCDISR {
        match value {
            false => RESYNCDISR::_0,
            true => RESYNCDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RESYNCDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RESYNCDISR::_1
    }
}
#[doc = "Possible values of the field `BOTHEDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOTHEDGER {
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    _0,
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    _1,
}
impl BOTHEDGER {
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
            BOTHEDGER::_0 => false,
            BOTHEDGER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOTHEDGER {
        match value {
            false => BOTHEDGER::_0,
            true => BOTHEDGER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOTHEDGER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BOTHEDGER::_1
    }
}
#[doc = "Possible values of the field `RDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMAER {
    #[doc = "DMA request disabled."]
    _0,
    #[doc = "DMA request enabled."]
    _1,
}
impl RDMAER {
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
            RDMAER::_0 => false,
            RDMAER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDMAER {
        match value {
            false => RDMAER::_0,
            true => RDMAER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDMAER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDMAER::_1
    }
}
#[doc = "Possible values of the field `TDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMAER {
    #[doc = "DMA request disabled."]
    _0,
    #[doc = "DMA request enabled."]
    _1,
}
impl TDMAER {
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
            TDMAER::_0 => false,
            TDMAER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDMAER {
        match value {
            false => TDMAER::_0,
            true => TDMAER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDMAER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDMAER::_1
    }
}
#[doc = "Values that can be written to the field `RESYNCDIS`"]
pub enum RESYNCDISW {
    #[doc = "Resynchronization during received data word is supported"]
    _0,
    #[doc = "Resynchronization during received data word is disabled"]
    _1,
}
impl RESYNCDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESYNCDISW::_0 => false,
            RESYNCDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESYNCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _RESYNCDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESYNCDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resynchronization during received data word is supported"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESYNCDISW::_0)
    }
    #[doc = "Resynchronization during received data word is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESYNCDISW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOTHEDGE`"]
pub enum BOTHEDGEW {
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    _0,
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    _1,
}
impl BOTHEDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOTHEDGEW::_0 => false,
            BOTHEDGEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOTHEDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOTHEDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOTHEDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOTHEDGEW::_0)
    }
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOTHEDGEW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDMAE`"]
pub enum RDMAEW {
    #[doc = "DMA request disabled."]
    _0,
    #[doc = "DMA request enabled."]
    _1,
}
impl RDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDMAEW::_0 => false,
            RDMAEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _RDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDMAEW::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDMAEW::_1)
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
#[doc = "Values that can be written to the field `TDMAE`"]
pub enum TDMAEW {
    #[doc = "DMA request disabled."]
    _0,
    #[doc = "DMA request enabled."]
    _1,
}
impl TDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDMAEW::_0 => false,
            TDMAEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDMAEW::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDMAEW::_1)
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
    #[doc = "Bit 0 - Resynchronization Disable"]
    #[inline]
    pub fn resyncdis(&self) -> RESYNCDISR {
        RESYNCDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Both Edge Sampling"]
    #[inline]
    pub fn bothedge(&self) -> BOTHEDGER {
        BOTHEDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Receiver Full DMA Enable"]
    #[inline]
    pub fn rdmae(&self) -> RDMAER {
        RDMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Transmitter DMA Enable"]
    #[inline]
    pub fn tdmae(&self) -> TDMAER {
        TDMAER::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Resynchronization Disable"]
    #[inline]
    pub fn resyncdis(&mut self) -> _RESYNCDISW {
        _RESYNCDISW { w: self }
    }
    #[doc = "Bit 1 - Both Edge Sampling"]
    #[inline]
    pub fn bothedge(&mut self) -> _BOTHEDGEW {
        _BOTHEDGEW { w: self }
    }
    #[doc = "Bit 5 - Receiver Full DMA Enable"]
    #[inline]
    pub fn rdmae(&mut self) -> _RDMAEW {
        _RDMAEW { w: self }
    }
    #[doc = "Bit 7 - Transmitter DMA Enable"]
    #[inline]
    pub fn tdmae(&mut self) -> _TDMAEW {
        _TDMAEW { w: self }
    }
}
