#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONF {
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
#[doc = "Possible values of the field `DOZEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEENR {
    #[doc = "Internal LPTPM counter continues in Doze mode."]
    _0,
    #[doc = "Internal LPTPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    _1,
}
impl DOZEENR {
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
            DOZEENR::_0 => false,
            DOZEENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZEENR {
        match value {
            false => DOZEENR::_0,
            true => DOZEENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DOZEENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DOZEENR::_1
    }
}
#[doc = "Possible values of the field `DBGMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGMODER {
    #[doc = "LPTPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    _00,
    #[doc = "LPTPM counter continues in debug mode."]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DBGMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DBGMODER::_00 => 0,
            DBGMODER::_11 => 3,
            DBGMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DBGMODER {
        match value {
            0 => DBGMODER::_00,
            3 => DBGMODER::_11,
            i => DBGMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DBGMODER::_00
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DBGMODER::_11
    }
}
#[doc = "Possible values of the field `GTBEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEENR {
    #[doc = "All channels use the internally generated LPTPM counter as their timebase"]
    _0,
    #[doc = "All channels use an externally generated global timebase as their timebase"]
    _1,
}
impl GTBEENR {
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
            GTBEENR::_0 => false,
            GTBEENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GTBEENR {
        match value {
            false => GTBEENR::_0,
            true => GTBEENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GTBEENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GTBEENR::_1
    }
}
#[doc = "Possible values of the field `CSOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSOTR {
    #[doc = "LPTPM counter starts to increment immediately, once it is enabled."]
    _0,
    #[doc = "LPTPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    _1,
}
impl CSOTR {
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
            CSOTR::_0 => false,
            CSOTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSOTR {
        match value {
            false => CSOTR::_0,
            true => CSOTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CSOTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CSOTR::_1
    }
}
#[doc = "Possible values of the field `CSOO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSOOR {
    #[doc = "LPTPM counter continues incrementing or decrementing after overflow"]
    _0,
    #[doc = "LPTPM counter stops incrementing or decrementing after overflow."]
    _1,
}
impl CSOOR {
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
            CSOOR::_0 => false,
            CSOOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSOOR {
        match value {
            false => CSOOR::_0,
            true => CSOOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CSOOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CSOOR::_1
    }
}
#[doc = "Possible values of the field `CROT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CROTR {
    #[doc = "Counter is not reloaded due to a rising edge on the selected input trigger"]
    _0,
    #[doc = "Counter is reloaded when a rising edge is detected on the selected input trigger"]
    _1,
}
impl CROTR {
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
            CROTR::_0 => false,
            CROTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CROTR {
        match value {
            false => CROTR::_0,
            true => CROTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CROTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CROTR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TRGSELR {
    bits: u8,
}
impl TRGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `DOZEEN`"]
pub enum DOZEENW {
    #[doc = "Internal LPTPM counter continues in Doze mode."]
    _0,
    #[doc = "Internal LPTPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    _1,
}
impl DOZEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZEENW::_0 => false,
            DOZEENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZEENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal LPTPM counter continues in Doze mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZEENW::_0)
    }
    #[doc = "Internal LPTPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZEENW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBGMODE`"]
pub enum DBGMODEW {
    #[doc = "LPTPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    _00,
    #[doc = "LPTPM counter continues in debug mode."]
    _11,
}
impl DBGMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DBGMODEW::_00 => 0,
            DBGMODEW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LPTPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DBGMODEW::_00)
    }
    #[doc = "LPTPM counter continues in debug mode."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DBGMODEW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GTBEEN`"]
pub enum GTBEENW {
    #[doc = "All channels use the internally generated LPTPM counter as their timebase"]
    _0,
    #[doc = "All channels use an externally generated global timebase as their timebase"]
    _1,
}
impl GTBEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GTBEENW::_0 => false,
            GTBEENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GTBEENW<'a> {
    w: &'a mut W,
}
impl<'a> _GTBEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GTBEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All channels use the internally generated LPTPM counter as their timebase"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBEENW::_0)
    }
    #[doc = "All channels use an externally generated global timebase as their timebase"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBEENW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSOT`"]
pub enum CSOTW {
    #[doc = "LPTPM counter starts to increment immediately, once it is enabled."]
    _0,
    #[doc = "LPTPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    _1,
}
impl CSOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSOTW::_0 => false,
            CSOTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSOTW<'a> {
    w: &'a mut W,
}
impl<'a> _CSOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSOTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPTPM counter starts to increment immediately, once it is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSOTW::_0)
    }
    #[doc = "LPTPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSOTW::_1)
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
#[doc = "Values that can be written to the field `CSOO`"]
pub enum CSOOW {
    #[doc = "LPTPM counter continues incrementing or decrementing after overflow"]
    _0,
    #[doc = "LPTPM counter stops incrementing or decrementing after overflow."]
    _1,
}
impl CSOOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSOOW::_0 => false,
            CSOOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSOOW<'a> {
    w: &'a mut W,
}
impl<'a> _CSOOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSOOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPTPM counter continues incrementing or decrementing after overflow"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSOOW::_0)
    }
    #[doc = "LPTPM counter stops incrementing or decrementing after overflow."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSOOW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CROT`"]
pub enum CROTW {
    #[doc = "Counter is not reloaded due to a rising edge on the selected input trigger"]
    _0,
    #[doc = "Counter is reloaded when a rising edge is detected on the selected input trigger"]
    _1,
}
impl CROTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CROTW::_0 => false,
            CROTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CROTW<'a> {
    w: &'a mut W,
}
impl<'a> _CROTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CROTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter is not reloaded due to a rising edge on the selected input trigger"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CROTW::_0)
    }
    #[doc = "Counter is reloaded when a rising edge is detected on the selected input trigger"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CROTW::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 5 - Doze Enable"]
    #[inline]
    pub fn dozeen(&self) -> DOZEENR {
        DOZEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline]
    pub fn dbgmode(&self) -> DBGMODER {
        DBGMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 9 - Global time base enable"]
    #[inline]
    pub fn gtbeen(&self) -> GTBEENR {
        GTBEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Counter Start on Trigger"]
    #[inline]
    pub fn csot(&self) -> CSOTR {
        CSOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Counter Stop On Overflow"]
    #[inline]
    pub fn csoo(&self) -> CSOOR {
        CSOOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Counter Reload On Trigger"]
    #[inline]
    pub fn crot(&self) -> CROTR {
        CROTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline]
    pub fn trgsel(&self) -> TRGSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRGSELR { bits }
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
    #[doc = "Bit 5 - Doze Enable"]
    #[inline]
    pub fn dozeen(&mut self) -> _DOZEENW {
        _DOZEENW { w: self }
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline]
    pub fn dbgmode(&mut self) -> _DBGMODEW {
        _DBGMODEW { w: self }
    }
    #[doc = "Bit 9 - Global time base enable"]
    #[inline]
    pub fn gtbeen(&mut self) -> _GTBEENW {
        _GTBEENW { w: self }
    }
    #[doc = "Bit 16 - Counter Start on Trigger"]
    #[inline]
    pub fn csot(&mut self) -> _CSOTW {
        _CSOTW { w: self }
    }
    #[doc = "Bit 17 - Counter Stop On Overflow"]
    #[inline]
    pub fn csoo(&mut self) -> _CSOOW {
        _CSOOW { w: self }
    }
    #[doc = "Bit 18 - Counter Reload On Trigger"]
    #[inline]
    pub fn crot(&mut self) -> _CROTW {
        _CROTW { w: self }
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline]
    pub fn trgsel(&mut self) -> _TRGSELW {
        _TRGSELW { w: self }
    }
}
