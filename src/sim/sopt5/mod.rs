#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT5 {
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
#[doc = "Possible values of the field `UART0TXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0TXSRCR {
    #[doc = "UART0_TX pin"]
    _00,
    #[doc = "UART0_TX pin modulated with TPM1 channel 0 output"]
    _01,
    #[doc = "UART0_TX pin modulated with TPM2 channel 0 output"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UART0TXSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UART0TXSRCR::_00 => 0,
            UART0TXSRCR::_01 => 1,
            UART0TXSRCR::_10 => 2,
            UART0TXSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UART0TXSRCR {
        match value {
            0 => UART0TXSRCR::_00,
            1 => UART0TXSRCR::_01,
            2 => UART0TXSRCR::_10,
            i => UART0TXSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == UART0TXSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == UART0TXSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == UART0TXSRCR::_10
    }
}
#[doc = "Possible values of the field `UART0RXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0RXSRCR {
    #[doc = "UART0_RX pin"]
    _0,
    #[doc = "CMP0 output"]
    _1,
}
impl UART0RXSRCR {
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
            UART0RXSRCR::_0 => false,
            UART0RXSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART0RXSRCR {
        match value {
            false => UART0RXSRCR::_0,
            true => UART0RXSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UART0RXSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UART0RXSRCR::_1
    }
}
#[doc = "Possible values of the field `UART1TXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1TXSRCR {
    #[doc = "UART1_TX pin"]
    _00,
    #[doc = "UART1_TX pin modulated with TPM1 channel 0 output"]
    _01,
    #[doc = "UART1_TX pin modulated with TPM2 channel 0 output"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UART1TXSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UART1TXSRCR::_00 => 0,
            UART1TXSRCR::_01 => 1,
            UART1TXSRCR::_10 => 2,
            UART1TXSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UART1TXSRCR {
        match value {
            0 => UART1TXSRCR::_00,
            1 => UART1TXSRCR::_01,
            2 => UART1TXSRCR::_10,
            i => UART1TXSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == UART1TXSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == UART1TXSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == UART1TXSRCR::_10
    }
}
#[doc = "Possible values of the field `UART1RXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1RXSRCR {
    #[doc = "UART1_RX pin"]
    _0,
    #[doc = "CMP0 output"]
    _1,
}
impl UART1RXSRCR {
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
            UART1RXSRCR::_0 => false,
            UART1RXSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART1RXSRCR {
        match value {
            false => UART1RXSRCR::_0,
            true => UART1RXSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UART1RXSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UART1RXSRCR::_1
    }
}
#[doc = "Possible values of the field `UART0ODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0ODER {
    #[doc = "Open drain is disabled on UART0"]
    _0,
    #[doc = "Open drain is enabled on UART0"]
    _1,
}
impl UART0ODER {
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
            UART0ODER::_0 => false,
            UART0ODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART0ODER {
        match value {
            false => UART0ODER::_0,
            true => UART0ODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UART0ODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UART0ODER::_1
    }
}
#[doc = "Possible values of the field `UART1ODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1ODER {
    #[doc = "Open drain is disabled on UART1"]
    _0,
    #[doc = "Open drain is enabled on UART1"]
    _1,
}
impl UART1ODER {
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
            UART1ODER::_0 => false,
            UART1ODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART1ODER {
        match value {
            false => UART1ODER::_0,
            true => UART1ODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UART1ODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UART1ODER::_1
    }
}
#[doc = "Possible values of the field `UART2ODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2ODER {
    #[doc = "Open drain is disabled on UART2"]
    _0,
    #[doc = "Open drain is enabled on UART2"]
    _1,
}
impl UART2ODER {
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
            UART2ODER::_0 => false,
            UART2ODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART2ODER {
        match value {
            false => UART2ODER::_0,
            true => UART2ODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UART2ODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UART2ODER::_1
    }
}
#[doc = "Values that can be written to the field `UART0TXSRC`"]
pub enum UART0TXSRCW {
    #[doc = "UART0_TX pin"]
    _00,
    #[doc = "UART0_TX pin modulated with TPM1 channel 0 output"]
    _01,
    #[doc = "UART0_TX pin modulated with TPM2 channel 0 output"]
    _10,
}
impl UART0TXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UART0TXSRCW::_00 => 0,
            UART0TXSRCW::_01 => 1,
            UART0TXSRCW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART0TXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _UART0TXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART0TXSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "UART0_TX pin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART0TXSRCW::_00)
    }
    #[doc = "UART0_TX pin modulated with TPM1 channel 0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART0TXSRCW::_01)
    }
    #[doc = "UART0_TX pin modulated with TPM2 channel 0 output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART0TXSRCW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART0RXSRC`"]
pub enum UART0RXSRCW {
    #[doc = "UART0_RX pin"]
    _0,
    #[doc = "CMP0 output"]
    _1,
}
impl UART0RXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART0RXSRCW::_0 => false,
            UART0RXSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART0RXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _UART0RXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART0RXSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "UART0_RX pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0RXSRCW::_0)
    }
    #[doc = "CMP0 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0RXSRCW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART1TXSRC`"]
pub enum UART1TXSRCW {
    #[doc = "UART1_TX pin"]
    _00,
    #[doc = "UART1_TX pin modulated with TPM1 channel 0 output"]
    _01,
    #[doc = "UART1_TX pin modulated with TPM2 channel 0 output"]
    _10,
}
impl UART1TXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UART1TXSRCW::_00 => 0,
            UART1TXSRCW::_01 => 1,
            UART1TXSRCW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART1TXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _UART1TXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART1TXSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "UART1_TX pin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART1TXSRCW::_00)
    }
    #[doc = "UART1_TX pin modulated with TPM1 channel 0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART1TXSRCW::_01)
    }
    #[doc = "UART1_TX pin modulated with TPM2 channel 0 output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART1TXSRCW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART1RXSRC`"]
pub enum UART1RXSRCW {
    #[doc = "UART1_RX pin"]
    _0,
    #[doc = "CMP0 output"]
    _1,
}
impl UART1RXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART1RXSRCW::_0 => false,
            UART1RXSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART1RXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _UART1RXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART1RXSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "UART1_RX pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1RXSRCW::_0)
    }
    #[doc = "CMP0 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1RXSRCW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART0ODE`"]
pub enum UART0ODEW {
    #[doc = "Open drain is disabled on UART0"]
    _0,
    #[doc = "Open drain is enabled on UART0"]
    _1,
}
impl UART0ODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART0ODEW::_0 => false,
            UART0ODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART0ODEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART0ODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART0ODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Open drain is disabled on UART0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0ODEW::_0)
    }
    #[doc = "Open drain is enabled on UART0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0ODEW::_1)
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
#[doc = "Values that can be written to the field `UART1ODE`"]
pub enum UART1ODEW {
    #[doc = "Open drain is disabled on UART1"]
    _0,
    #[doc = "Open drain is enabled on UART1"]
    _1,
}
impl UART1ODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART1ODEW::_0 => false,
            UART1ODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART1ODEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART1ODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART1ODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Open drain is disabled on UART1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1ODEW::_0)
    }
    #[doc = "Open drain is enabled on UART1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1ODEW::_1)
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
#[doc = "Values that can be written to the field `UART2ODE`"]
pub enum UART2ODEW {
    #[doc = "Open drain is disabled on UART2"]
    _0,
    #[doc = "Open drain is enabled on UART2"]
    _1,
}
impl UART2ODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART2ODEW::_0 => false,
            UART2ODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART2ODEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART2ODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART2ODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Open drain is disabled on UART2"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART2ODEW::_0)
    }
    #[doc = "Open drain is enabled on UART2"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART2ODEW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - UART0 transmit data source select"]
    #[inline]
    pub fn uart0txsrc(&self) -> UART0TXSRCR {
        UART0TXSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - UART0 receive data source select"]
    #[inline]
    pub fn uart0rxsrc(&self) -> UART0RXSRCR {
        UART0RXSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - UART1 transmit data source select"]
    #[inline]
    pub fn uart1txsrc(&self) -> UART1TXSRCR {
        UART1TXSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - UART1 receive data source select"]
    #[inline]
    pub fn uart1rxsrc(&self) -> UART1RXSRCR {
        UART1RXSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - UART0 Open Drain Enable"]
    #[inline]
    pub fn uart0ode(&self) -> UART0ODER {
        UART0ODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - UART1 Open Drain Enable"]
    #[inline]
    pub fn uart1ode(&self) -> UART1ODER {
        UART1ODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - UART2 Open Drain Enable"]
    #[inline]
    pub fn uart2ode(&self) -> UART2ODER {
        UART2ODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
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
    #[doc = "Bits 0:1 - UART0 transmit data source select"]
    #[inline]
    pub fn uart0txsrc(&mut self) -> _UART0TXSRCW {
        _UART0TXSRCW { w: self }
    }
    #[doc = "Bit 2 - UART0 receive data source select"]
    #[inline]
    pub fn uart0rxsrc(&mut self) -> _UART0RXSRCW {
        _UART0RXSRCW { w: self }
    }
    #[doc = "Bits 4:5 - UART1 transmit data source select"]
    #[inline]
    pub fn uart1txsrc(&mut self) -> _UART1TXSRCW {
        _UART1TXSRCW { w: self }
    }
    #[doc = "Bit 6 - UART1 receive data source select"]
    #[inline]
    pub fn uart1rxsrc(&mut self) -> _UART1RXSRCW {
        _UART1RXSRCW { w: self }
    }
    #[doc = "Bit 16 - UART0 Open Drain Enable"]
    #[inline]
    pub fn uart0ode(&mut self) -> _UART0ODEW {
        _UART0ODEW { w: self }
    }
    #[doc = "Bit 17 - UART1 Open Drain Enable"]
    #[inline]
    pub fn uart1ode(&mut self) -> _UART1ODEW {
        _UART1ODEW { w: self }
    }
    #[doc = "Bit 18 - UART2 Open Drain Enable"]
    #[inline]
    pub fn uart2ode(&mut self) -> _UART2ODEW {
        _UART2ODEW { w: self }
    }
}
