#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT2 {
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
#[doc = "Possible values of the field `RTCCLKOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCLKOUTSELR {
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0,
    #[doc = "OSCERCLK clock is output on the RTC_CLKOUT pin."]
    _1,
}
impl RTCCLKOUTSELR {
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
            RTCCLKOUTSELR::_0 => false,
            RTCCLKOUTSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCCLKOUTSELR {
        match value {
            false => RTCCLKOUTSELR::_0,
            true => RTCCLKOUTSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RTCCLKOUTSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RTCCLKOUTSELR::_1
    }
}
#[doc = "Possible values of the field `CLKOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSELR {
    #[doc = "Bus clock"]
    _010,
    #[doc = "LPO clock (1 kHz)"]
    _011,
    #[doc = "MCGIRCLK"]
    _100,
    #[doc = "OSCERCLK"]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKOUTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTSELR::_010 => 2,
            CLKOUTSELR::_011 => 3,
            CLKOUTSELR::_100 => 4,
            CLKOUTSELR::_110 => 6,
            CLKOUTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTSELR {
        match value {
            2 => CLKOUTSELR::_010,
            3 => CLKOUTSELR::_011,
            4 => CLKOUTSELR::_100,
            6 => CLKOUTSELR::_110,
            i => CLKOUTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTSELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTSELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTSELR::_100
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTSELR::_110
    }
}
#[doc = "Possible values of the field `PLLFLLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLFLLSELR {
    #[doc = "MCGFLLCLK clock"]
    _0,
    #[doc = "MCGPLLCLK clock with fixed divide by two"]
    _1,
}
impl PLLFLLSELR {
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
            PLLFLLSELR::_0 => false,
            PLLFLLSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLFLLSELR {
        match value {
            false => PLLFLLSELR::_0,
            true => PLLFLLSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLLFLLSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLLFLLSELR::_1
    }
}
#[doc = "Possible values of the field `USBSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSRCR {
    #[doc = "External bypass clock (USB_CLKIN)."]
    _0,
    #[doc = "MCGPLLCLK/2 or MCGFLLCLK clock"]
    _1,
}
impl USBSRCR {
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
            USBSRCR::_0 => false,
            USBSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBSRCR {
        match value {
            false => USBSRCR::_0,
            true => USBSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBSRCR::_1
    }
}
#[doc = "Possible values of the field `TPMSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPMSRCR {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK clock or MCGPLLCLK/2"]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl TPMSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TPMSRCR::_00 => 0,
            TPMSRCR::_01 => 1,
            TPMSRCR::_10 => 2,
            TPMSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TPMSRCR {
        match value {
            0 => TPMSRCR::_00,
            1 => TPMSRCR::_01,
            2 => TPMSRCR::_10,
            3 => TPMSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TPMSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TPMSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TPMSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TPMSRCR::_11
    }
}
#[doc = "Possible values of the field `UART0SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0SRCR {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK clock or MCGPLLCLK/2 clock"]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl UART0SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UART0SRCR::_00 => 0,
            UART0SRCR::_01 => 1,
            UART0SRCR::_10 => 2,
            UART0SRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UART0SRCR {
        match value {
            0 => UART0SRCR::_00,
            1 => UART0SRCR::_01,
            2 => UART0SRCR::_10,
            3 => UART0SRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == UART0SRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == UART0SRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == UART0SRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == UART0SRCR::_11
    }
}
#[doc = "Values that can be written to the field `RTCCLKOUTSEL`"]
pub enum RTCCLKOUTSELW {
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0,
    #[doc = "OSCERCLK clock is output on the RTC_CLKOUT pin."]
    _1,
}
impl RTCCLKOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCCLKOUTSELW::_0 => false,
            RTCCLKOUTSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCCLKOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCCLKOUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCCLKOUTSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCCLKOUTSELW::_0)
    }
    #[doc = "OSCERCLK clock is output on the RTC_CLKOUT pin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCLKOUTSELW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKOUTSEL`"]
pub enum CLKOUTSELW {
    #[doc = "Bus clock"]
    _010,
    #[doc = "LPO clock (1 kHz)"]
    _011,
    #[doc = "MCGIRCLK"]
    _100,
    #[doc = "OSCERCLK"]
    _110,
}
impl CLKOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSELW::_010 => 2,
            CLKOUTSELW::_011 => 3,
            CLKOUTSELW::_100 => 4,
            CLKOUTSELW::_110 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Bus clock"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_011)
    }
    #[doc = "MCGIRCLK"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_100)
    }
    #[doc = "OSCERCLK"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLFLLSEL`"]
pub enum PLLFLLSELW {
    #[doc = "MCGFLLCLK clock"]
    _0,
    #[doc = "MCGPLLCLK clock with fixed divide by two"]
    _1,
}
impl PLLFLLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLFLLSELW::_0 => false,
            PLLFLLSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLFLLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLFLLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLFLLSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLFLLSELW::_0)
    }
    #[doc = "MCGPLLCLK clock with fixed divide by two"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLFLLSELW::_1)
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
#[doc = "Values that can be written to the field `USBSRC`"]
pub enum USBSRCW {
    #[doc = "External bypass clock (USB_CLKIN)."]
    _0,
    #[doc = "MCGPLLCLK/2 or MCGFLLCLK clock"]
    _1,
}
impl USBSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBSRCW::_0 => false,
            USBSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _USBSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSRCW::_0)
    }
    #[doc = "MCGPLLCLK/2 or MCGFLLCLK clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSRCW::_1)
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
#[doc = "Values that can be written to the field `TPMSRC`"]
pub enum TPMSRCW {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK clock or MCGPLLCLK/2"]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl TPMSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TPMSRCW::_00 => 0,
            TPMSRCW::_01 => 1,
            TPMSRCW::_10 => 2,
            TPMSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPMSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TPMSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPMSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPMSRCW::_00)
    }
    #[doc = "MCGFLLCLK clock or MCGPLLCLK/2"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPMSRCW::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPMSRCW::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPMSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART0SRC`"]
pub enum UART0SRCW {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK clock or MCGPLLCLK/2 clock"]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl UART0SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UART0SRCW::_00 => 0,
            UART0SRCW::_01 => 1,
            UART0SRCW::_10 => 2,
            UART0SRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART0SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _UART0SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART0SRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART0SRCW::_00)
    }
    #[doc = "MCGFLLCLK clock or MCGPLLCLK/2 clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART0SRCW::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART0SRCW::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(UART0SRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline]
    pub fn rtcclkoutsel(&self) -> RTCCLKOUTSELR {
        RTCCLKOUTSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline]
    pub fn clkoutsel(&self) -> CLKOUTSELR {
        CLKOUTSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - PLL/FLL clock select"]
    #[inline]
    pub fn pllfllsel(&self) -> PLLFLLSELR {
        PLLFLLSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline]
    pub fn usbsrc(&self) -> USBSRCR {
        USBSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline]
    pub fn tpmsrc(&self) -> TPMSRCR {
        TPMSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - UART0 clock source select"]
    #[inline]
    pub fn uart0src(&self) -> UART0SRCR {
        UART0SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline]
    pub fn rtcclkoutsel(&mut self) -> _RTCCLKOUTSELW {
        _RTCCLKOUTSELW { w: self }
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline]
    pub fn clkoutsel(&mut self) -> _CLKOUTSELW {
        _CLKOUTSELW { w: self }
    }
    #[doc = "Bit 16 - PLL/FLL clock select"]
    #[inline]
    pub fn pllfllsel(&mut self) -> _PLLFLLSELW {
        _PLLFLLSELW { w: self }
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline]
    pub fn usbsrc(&mut self) -> _USBSRCW {
        _USBSRCW { w: self }
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline]
    pub fn tpmsrc(&mut self) -> _TPMSRCW {
        _TPMSRCW { w: self }
    }
    #[doc = "Bits 26:27 - UART0 clock source select"]
    #[inline]
    pub fn uart0src(&mut self) -> _UART0SRCW {
        _UART0SRCW { w: self }
    }
}
