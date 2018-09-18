#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C2 {
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
#[doc = "Possible values of the field `SPC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPC0R {
    #[doc = "SPI uses separate pins for data input and data output (pin mode is normal). In master mode of operation: MISO is master in and MOSI is master out. In slave mode of operation: MISO is slave out and MOSI is slave in."]
    _0,
    #[doc = "SPI configured for single-wire bidirectional operation (pin mode is bidirectional). In master mode of operation: MISO is not used by SPI; MOSI is master in when BIDIROE is 0 or master I/O when BIDIROE is 1. In slave mode of operation: MISO is slave in when BIDIROE is 0 or slave I/O when BIDIROE is 1; MOSI is not used by SPI."]
    _1,
}
impl SPC0R {
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
            SPC0R::_0 => false,
            SPC0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPC0R {
        match value {
            false => SPC0R::_0,
            true => SPC0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPC0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPC0R::_1
    }
}
#[doc = "Possible values of the field `SPISWAI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPISWAIR {
    #[doc = "SPI clocks continue to operate in wait mode"]
    _0,
    #[doc = "SPI clocks stop when the MCU enters wait mode"]
    _1,
}
impl SPISWAIR {
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
            SPISWAIR::_0 => false,
            SPISWAIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPISWAIR {
        match value {
            false => SPISWAIR::_0,
            true => SPISWAIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPISWAIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPISWAIR::_1
    }
}
#[doc = "Possible values of the field `RXDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAER {
    #[doc = "DMA request for receive is disabled and interrupt from SPRF is allowed"]
    _0,
    #[doc = "DMA request for receive is enabled and interrupt from SPRF is disabled"]
    _1,
}
impl RXDMAER {
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
            RXDMAER::_0 => false,
            RXDMAER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDMAER {
        match value {
            false => RXDMAER::_0,
            true => RXDMAER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXDMAER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXDMAER::_1
    }
}
#[doc = "Possible values of the field `BIDIROE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIROER {
    #[doc = "Output driver disabled so SPI data I/O pin acts as an input"]
    _0,
    #[doc = "SPI I/O pin enabled as an output"]
    _1,
}
impl BIDIROER {
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
            BIDIROER::_0 => false,
            BIDIROER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIDIROER {
        match value {
            false => BIDIROER::_0,
            true => BIDIROER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BIDIROER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BIDIROER::_1
    }
}
#[doc = "Possible values of the field `MODFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODFENR {
    #[doc = "Mode fault function disabled, master SS pin reverts to general-purpose I/O not controlled by SPI"]
    _0,
    #[doc = "Mode fault function enabled, master SS pin acts as the mode fault input or the slave select output"]
    _1,
}
impl MODFENR {
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
            MODFENR::_0 => false,
            MODFENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODFENR {
        match value {
            false => MODFENR::_0,
            true => MODFENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MODFENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MODFENR::_1
    }
}
#[doc = "Possible values of the field `TXDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAER {
    #[doc = "DMA request for transmit is disabled and interrupt from SPTEF is allowed"]
    _0,
    #[doc = "DMA request for transmit is enabled and interrupt from SPTEF is disabled"]
    _1,
}
impl TXDMAER {
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
            TXDMAER::_0 => false,
            TXDMAER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDMAER {
        match value {
            false => TXDMAER::_0,
            true => TXDMAER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXDMAER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXDMAER::_1
    }
}
#[doc = "Possible values of the field `SPMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPMIER {
    #[doc = "Interrupts from SPMF inhibited (use polling)"]
    _0,
    #[doc = "When SPMF is 1, requests a hardware interrupt"]
    _1,
}
impl SPMIER {
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
            SPMIER::_0 => false,
            SPMIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPMIER {
        match value {
            false => SPMIER::_0,
            true => SPMIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPMIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPMIER::_1
    }
}
#[doc = "Values that can be written to the field `SPC0`"]
pub enum SPC0W {
    #[doc = "SPI uses separate pins for data input and data output (pin mode is normal). In master mode of operation: MISO is master in and MOSI is master out. In slave mode of operation: MISO is slave out and MOSI is slave in."]
    _0,
    #[doc = "SPI configured for single-wire bidirectional operation (pin mode is bidirectional). In master mode of operation: MISO is not used by SPI; MOSI is master in when BIDIROE is 0 or master I/O when BIDIROE is 1. In slave mode of operation: MISO is slave in when BIDIROE is 0 or slave I/O when BIDIROE is 1; MOSI is not used by SPI."]
    _1,
}
impl SPC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPC0W::_0 => false,
            SPC0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPC0W<'a> {
    w: &'a mut W,
}
impl<'a> _SPC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI uses separate pins for data input and data output (pin mode is normal). In master mode of operation: MISO is master in and MOSI is master out. In slave mode of operation: MISO is slave out and MOSI is slave in."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPC0W::_0)
    }
    #[doc = "SPI configured for single-wire bidirectional operation (pin mode is bidirectional). In master mode of operation: MISO is not used by SPI; MOSI is master in when BIDIROE is 0 or master I/O when BIDIROE is 1. In slave mode of operation: MISO is slave in when BIDIROE is 0 or slave I/O when BIDIROE is 1; MOSI is not used by SPI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPC0W::_1)
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
#[doc = "Values that can be written to the field `SPISWAI`"]
pub enum SPISWAIW {
    #[doc = "SPI clocks continue to operate in wait mode"]
    _0,
    #[doc = "SPI clocks stop when the MCU enters wait mode"]
    _1,
}
impl SPISWAIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPISWAIW::_0 => false,
            SPISWAIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPISWAIW<'a> {
    w: &'a mut W,
}
impl<'a> _SPISWAIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPISWAIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI clocks continue to operate in wait mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPISWAIW::_0)
    }
    #[doc = "SPI clocks stop when the MCU enters wait mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPISWAIW::_1)
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
#[doc = "Values that can be written to the field `RXDMAE`"]
pub enum RXDMAEW {
    #[doc = "DMA request for receive is disabled and interrupt from SPRF is allowed"]
    _0,
    #[doc = "DMA request for receive is enabled and interrupt from SPRF is disabled"]
    _1,
}
impl RXDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDMAEW::_0 => false,
            RXDMAEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request for receive is disabled and interrupt from SPRF is allowed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXDMAEW::_0)
    }
    #[doc = "DMA request for receive is enabled and interrupt from SPRF is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXDMAEW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BIDIROE`"]
pub enum BIDIROEW {
    #[doc = "Output driver disabled so SPI data I/O pin acts as an input"]
    _0,
    #[doc = "SPI I/O pin enabled as an output"]
    _1,
}
impl BIDIROEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BIDIROEW::_0 => false,
            BIDIROEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BIDIROEW<'a> {
    w: &'a mut W,
}
impl<'a> _BIDIROEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIDIROEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output driver disabled so SPI data I/O pin acts as an input"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BIDIROEW::_0)
    }
    #[doc = "SPI I/O pin enabled as an output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BIDIROEW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODFEN`"]
pub enum MODFENW {
    #[doc = "Mode fault function disabled, master SS pin reverts to general-purpose I/O not controlled by SPI"]
    _0,
    #[doc = "Mode fault function enabled, master SS pin acts as the mode fault input or the slave select output"]
    _1,
}
impl MODFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODFENW::_0 => false,
            MODFENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODFENW<'a> {
    w: &'a mut W,
}
impl<'a> _MODFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mode fault function disabled, master SS pin reverts to general-purpose I/O not controlled by SPI"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODFENW::_0)
    }
    #[doc = "Mode fault function enabled, master SS pin acts as the mode fault input or the slave select output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODFENW::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXDMAE`"]
pub enum TXDMAEW {
    #[doc = "DMA request for transmit is disabled and interrupt from SPTEF is allowed"]
    _0,
    #[doc = "DMA request for transmit is enabled and interrupt from SPTEF is disabled"]
    _1,
}
impl TXDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDMAEW::_0 => false,
            TXDMAEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request for transmit is disabled and interrupt from SPTEF is allowed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDMAEW::_0)
    }
    #[doc = "DMA request for transmit is enabled and interrupt from SPTEF is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDMAEW::_1)
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
#[doc = "Values that can be written to the field `SPMIE`"]
pub enum SPMIEW {
    #[doc = "Interrupts from SPMF inhibited (use polling)"]
    _0,
    #[doc = "When SPMF is 1, requests a hardware interrupt"]
    _1,
}
impl SPMIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPMIEW::_0 => false,
            SPMIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _SPMIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPMIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupts from SPMF inhibited (use polling)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPMIEW::_0)
    }
    #[doc = "When SPMF is 1, requests a hardware interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPMIEW::_1)
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
    #[doc = "Bit 0 - SPI pin control 0"]
    #[inline]
    pub fn spc0(&self) -> SPC0R {
        SPC0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - SPI stop in wait mode"]
    #[inline]
    pub fn spiswai(&self) -> SPISWAIR {
        SPISWAIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Receive DMA enable"]
    #[inline]
    pub fn rxdmae(&self) -> RXDMAER {
        RXDMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Bidirectional mode output enable"]
    #[inline]
    pub fn bidiroe(&self) -> BIDIROER {
        BIDIROER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Master mode-fault function enable"]
    #[inline]
    pub fn modfen(&self) -> MODFENR {
        MODFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Transmit DMA enable"]
    #[inline]
    pub fn txdmae(&self) -> TXDMAER {
        TXDMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - SPI match interrupt enable"]
    #[inline]
    pub fn spmie(&self) -> SPMIER {
        SPMIER::_from({
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
    #[doc = "Bit 0 - SPI pin control 0"]
    #[inline]
    pub fn spc0(&mut self) -> _SPC0W {
        _SPC0W { w: self }
    }
    #[doc = "Bit 1 - SPI stop in wait mode"]
    #[inline]
    pub fn spiswai(&mut self) -> _SPISWAIW {
        _SPISWAIW { w: self }
    }
    #[doc = "Bit 2 - Receive DMA enable"]
    #[inline]
    pub fn rxdmae(&mut self) -> _RXDMAEW {
        _RXDMAEW { w: self }
    }
    #[doc = "Bit 3 - Bidirectional mode output enable"]
    #[inline]
    pub fn bidiroe(&mut self) -> _BIDIROEW {
        _BIDIROEW { w: self }
    }
    #[doc = "Bit 4 - Master mode-fault function enable"]
    #[inline]
    pub fn modfen(&mut self) -> _MODFENW {
        _MODFENW { w: self }
    }
    #[doc = "Bit 5 - Transmit DMA enable"]
    #[inline]
    pub fn txdmae(&mut self) -> _TXDMAEW {
        _TXDMAEW { w: self }
    }
    #[doc = "Bit 7 - SPI match interrupt enable"]
    #[inline]
    pub fn spmie(&mut self) -> _SPMIEW {
        _SPMIEW { w: self }
    }
}
