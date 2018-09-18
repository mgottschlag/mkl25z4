#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLACR {
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
#[doc = "Possible values of the field `ARB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBR {
    #[doc = "Fixed-priority arbitration for the crossbar masters"]
    _0,
    #[doc = "Round-robin arbitration for the crossbar masters"]
    _1,
}
impl ARBR {
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
            ARBR::_0 => false,
            ARBR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARBR {
        match value {
            false => ARBR::_0,
            true => ARBR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ARBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ARBR::_1
    }
}
#[doc = "Possible values of the field `DFCDA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFCDAR {
    #[doc = "Enable flash controller data caching"]
    _0,
    #[doc = "Disable flash controller data caching."]
    _1,
}
impl DFCDAR {
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
            DFCDAR::_0 => false,
            DFCDAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFCDAR {
        match value {
            false => DFCDAR::_0,
            true => DFCDAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFCDAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFCDAR::_1
    }
}
#[doc = "Possible values of the field `DFCIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFCICR {
    #[doc = "Enable flash controller instruction caching."]
    _0,
    #[doc = "Disable flash controller instruction caching."]
    _1,
}
impl DFCICR {
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
            DFCICR::_0 => false,
            DFCICR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFCICR {
        match value {
            false => DFCICR::_0,
            true => DFCICR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFCICR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFCICR::_1
    }
}
#[doc = "Possible values of the field `DFCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFCCR {
    #[doc = "Enable flash controller cache."]
    _0,
    #[doc = "Disable flash controller cache."]
    _1,
}
impl DFCCR {
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
            DFCCR::_0 => false,
            DFCCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFCCR {
        match value {
            false => DFCCR::_0,
            true => DFCCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFCCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFCCR::_1
    }
}
#[doc = "Possible values of the field `EFDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFDSR {
    #[doc = "Disable flash data speculation."]
    _0,
    #[doc = "Enable flash data speculation."]
    _1,
}
impl EFDSR {
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
            EFDSR::_0 => false,
            EFDSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EFDSR {
        match value {
            false => EFDSR::_0,
            true => EFDSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EFDSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EFDSR::_1
    }
}
#[doc = "Possible values of the field `DFCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFCSR {
    #[doc = "Enable flash controller speculation."]
    _0,
    #[doc = "Disable flash controller speculation."]
    _1,
}
impl DFCSR {
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
            DFCSR::_0 => false,
            DFCSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFCSR {
        match value {
            false => DFCSR::_0,
            true => DFCSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFCSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFCSR::_1
    }
}
#[doc = "Possible values of the field `ESFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESFCR {
    #[doc = "Disable stalling flash controller when flash is busy."]
    _0,
    #[doc = "Enable stalling flash controller when flash is busy."]
    _1,
}
impl ESFCR {
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
            ESFCR::_0 => false,
            ESFCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESFCR {
        match value {
            false => ESFCR::_0,
            true => ESFCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ESFCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ESFCR::_1
    }
}
#[doc = "Values that can be written to the field `ARB`"]
pub enum ARBW {
    #[doc = "Fixed-priority arbitration for the crossbar masters"]
    _0,
    #[doc = "Round-robin arbitration for the crossbar masters"]
    _1,
}
impl ARBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARBW::_0 => false,
            ARBW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fixed-priority arbitration for the crossbar masters"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARBW::_0)
    }
    #[doc = "Round-robin arbitration for the crossbar masters"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARBW::_1)
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
#[doc = r" Proxy"]
pub struct _CFCCW<'a> {
    w: &'a mut W,
}
impl<'a> _CFCCW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DFCDA`"]
pub enum DFCDAW {
    #[doc = "Enable flash controller data caching"]
    _0,
    #[doc = "Disable flash controller data caching."]
    _1,
}
impl DFCDAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFCDAW::_0 => false,
            DFCDAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFCDAW<'a> {
    w: &'a mut W,
}
impl<'a> _DFCDAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFCDAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable flash controller data caching"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFCDAW::_0)
    }
    #[doc = "Disable flash controller data caching."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFCDAW::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DFCIC`"]
pub enum DFCICW {
    #[doc = "Enable flash controller instruction caching."]
    _0,
    #[doc = "Disable flash controller instruction caching."]
    _1,
}
impl DFCICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFCICW::_0 => false,
            DFCICW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFCICW<'a> {
    w: &'a mut W,
}
impl<'a> _DFCICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFCICW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable flash controller instruction caching."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFCICW::_0)
    }
    #[doc = "Disable flash controller instruction caching."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFCICW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DFCC`"]
pub enum DFCCW {
    #[doc = "Enable flash controller cache."]
    _0,
    #[doc = "Disable flash controller cache."]
    _1,
}
impl DFCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFCCW::_0 => false,
            DFCCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFCCW<'a> {
    w: &'a mut W,
}
impl<'a> _DFCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFCCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable flash controller cache."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFCCW::_0)
    }
    #[doc = "Disable flash controller cache."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFCCW::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EFDS`"]
pub enum EFDSW {
    #[doc = "Disable flash data speculation."]
    _0,
    #[doc = "Enable flash data speculation."]
    _1,
}
impl EFDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EFDSW::_0 => false,
            EFDSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EFDSW<'a> {
    w: &'a mut W,
}
impl<'a> _EFDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EFDSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable flash data speculation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EFDSW::_0)
    }
    #[doc = "Enable flash data speculation."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EFDSW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DFCS`"]
pub enum DFCSW {
    #[doc = "Enable flash controller speculation."]
    _0,
    #[doc = "Disable flash controller speculation."]
    _1,
}
impl DFCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFCSW::_0 => false,
            DFCSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFCSW<'a> {
    w: &'a mut W,
}
impl<'a> _DFCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable flash controller speculation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFCSW::_0)
    }
    #[doc = "Disable flash controller speculation."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFCSW::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ESFC`"]
pub enum ESFCW {
    #[doc = "Disable stalling flash controller when flash is busy."]
    _0,
    #[doc = "Enable stalling flash controller when flash is busy."]
    _1,
}
impl ESFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESFCW::_0 => false,
            ESFCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESFCW<'a> {
    w: &'a mut W,
}
impl<'a> _ESFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESFCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable stalling flash controller when flash is busy."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESFCW::_0)
    }
    #[doc = "Enable stalling flash controller when flash is busy."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESFCW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 9 - Arbitration select"]
    #[inline]
    pub fn arb(&self) -> ARBR {
        ARBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Disable Flash Controller Data Caching"]
    #[inline]
    pub fn dfcda(&self) -> DFCDAR {
        DFCDAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Disable Flash Controller Instruction Caching"]
    #[inline]
    pub fn dfcic(&self) -> DFCICR {
        DFCICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Disable Flash Controller Cache"]
    #[inline]
    pub fn dfcc(&self) -> DFCCR {
        DFCCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enable Flash Data Speculation"]
    #[inline]
    pub fn efds(&self) -> EFDSR {
        EFDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Disable Flash Controller Speculation"]
    #[inline]
    pub fn dfcs(&self) -> DFCSR {
        DFCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable Stalling Flash Controller"]
    #[inline]
    pub fn esfc(&self) -> ESFCR {
        ESFCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 9 - Arbitration select"]
    #[inline]
    pub fn arb(&mut self) -> _ARBW {
        _ARBW { w: self }
    }
    #[doc = "Bit 10 - Clear Flash Controller Cache"]
    #[inline]
    pub fn cfcc(&mut self) -> _CFCCW {
        _CFCCW { w: self }
    }
    #[doc = "Bit 11 - Disable Flash Controller Data Caching"]
    #[inline]
    pub fn dfcda(&mut self) -> _DFCDAW {
        _DFCDAW { w: self }
    }
    #[doc = "Bit 12 - Disable Flash Controller Instruction Caching"]
    #[inline]
    pub fn dfcic(&mut self) -> _DFCICW {
        _DFCICW { w: self }
    }
    #[doc = "Bit 13 - Disable Flash Controller Cache"]
    #[inline]
    pub fn dfcc(&mut self) -> _DFCCW {
        _DFCCW { w: self }
    }
    #[doc = "Bit 14 - Enable Flash Data Speculation"]
    #[inline]
    pub fn efds(&mut self) -> _EFDSW {
        _EFDSW { w: self }
    }
    #[doc = "Bit 15 - Disable Flash Controller Speculation"]
    #[inline]
    pub fn dfcs(&mut self) -> _DFCSW {
        _DFCSW { w: self }
    }
    #[doc = "Bit 16 - Enable Stalling Flash Controller"]
    #[inline]
    pub fn esfc(&mut self) -> _ESFCW {
        _ESFCW { w: self }
    }
}
