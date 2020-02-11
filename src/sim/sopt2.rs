#[doc = "Reader of register SOPT2"]
pub type R = crate::R<u32, super::SOPT2>;
#[doc = "Writer for register SOPT2"]
pub type W = crate::W<u32, super::SOPT2>;
#[doc = "Register SOPT2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RTC clock out select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCLKOUTSEL_A {
    #[doc = "0: RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0 = 0,
    #[doc = "1: OSCERCLK clock is output on the RTC_CLKOUT pin."]
    _1 = 1,
}
impl From<RTCCLKOUTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCLKOUTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCCLKOUTSEL`"]
pub type RTCCLKOUTSEL_R = crate::R<bool, RTCCLKOUTSEL_A>;
impl RTCCLKOUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCLKOUTSEL_A {
        match self.bits {
            false => RTCCLKOUTSEL_A::_0,
            true => RTCCLKOUTSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCCLKOUTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCCLKOUTSEL_A::_1
    }
}
#[doc = "Write proxy for field `RTCCLKOUTSEL`"]
pub struct RTCCLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCLKOUTSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCCLKOUTSEL_A::_0)
    }
    #[doc = "OSCERCLK clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCLKOUTSEL_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "CLKOUT select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTSEL_A {
    #[doc = "2: Bus clock"]
    _010 = 2,
    #[doc = "3: LPO clock (1 kHz)"]
    _011 = 3,
    #[doc = "4: MCGIRCLK"]
    _100 = 4,
    #[doc = "6: OSCERCLK"]
    _110 = 6,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKOUTSEL`"]
pub type CLKOUTSEL_R = crate::R<u8, CLKOUTSEL_A>;
impl CLKOUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKOUTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(CLKOUTSEL_A::_010),
            3 => Val(CLKOUTSEL_A::_011),
            4 => Val(CLKOUTSEL_A::_100),
            6 => Val(CLKOUTSEL_A::_110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTSEL_A::_110
    }
}
#[doc = "Write proxy for field `CLKOUTSEL`"]
pub struct CLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Bus clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_011)
    }
    #[doc = "MCGIRCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_100)
    }
    #[doc = "OSCERCLK"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "PLL/FLL clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLFLLSEL_A {
    #[doc = "0: MCGFLLCLK clock"]
    _0 = 0,
    #[doc = "1: MCGPLLCLK clock with fixed divide by two"]
    _1 = 1,
}
impl From<PLLFLLSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLLFLLSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLFLLSEL`"]
pub type PLLFLLSEL_R = crate::R<bool, PLLFLLSEL_A>;
impl PLLFLLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLFLLSEL_A {
        match self.bits {
            false => PLLFLLSEL_A::_0,
            true => PLLFLLSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLFLLSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLFLLSEL_A::_1
    }
}
#[doc = "Write proxy for field `PLLFLLSEL`"]
pub struct PLLFLLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFLLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLFLLSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_0)
    }
    #[doc = "MCGPLLCLK clock with fixed divide by two"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "USB clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSRC_A {
    #[doc = "0: External bypass clock (USB_CLKIN)."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK/2 or MCGFLLCLK clock"]
    _1 = 1,
}
impl From<USBSRC_A> for bool {
    #[inline(always)]
    fn from(variant: USBSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBSRC`"]
pub type USBSRC_R = crate::R<bool, USBSRC_A>;
impl USBSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSRC_A {
        match self.bits {
            false => USBSRC_A::_0,
            true => USBSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBSRC_A::_1
    }
}
#[doc = "Write proxy for field `USBSRC`"]
pub struct USBSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSRC_A::_0)
    }
    #[doc = "MCGPLLCLK/2 or MCGFLLCLK clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSRC_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "TPM clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TPMSRC_A {
    #[doc = "0: Clock disabled"]
    _00 = 0,
    #[doc = "1: MCGFLLCLK clock or MCGPLLCLK/2"]
    _01 = 1,
    #[doc = "2: OSCERCLK clock"]
    _10 = 2,
    #[doc = "3: MCGIRCLK clock"]
    _11 = 3,
}
impl From<TPMSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TPMSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TPMSRC`"]
pub type TPMSRC_R = crate::R<u8, TPMSRC_A>;
impl TPMSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPMSRC_A {
        match self.bits {
            0 => TPMSRC_A::_00,
            1 => TPMSRC_A::_01,
            2 => TPMSRC_A::_10,
            3 => TPMSRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TPMSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TPMSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TPMSRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TPMSRC_A::_11
    }
}
#[doc = "Write proxy for field `TPMSRC`"]
pub struct TPMSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TPMSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPMSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPMSRC_A::_00)
    }
    #[doc = "MCGFLLCLK clock or MCGPLLCLK/2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPMSRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPMSRC_A::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPMSRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "UART0 clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART0SRC_A {
    #[doc = "0: Clock disabled"]
    _00 = 0,
    #[doc = "1: MCGFLLCLK clock or MCGPLLCLK/2 clock"]
    _01 = 1,
    #[doc = "2: OSCERCLK clock"]
    _10 = 2,
    #[doc = "3: MCGIRCLK clock"]
    _11 = 3,
}
impl From<UART0SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART0SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UART0SRC`"]
pub type UART0SRC_R = crate::R<u8, UART0SRC_A>;
impl UART0SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0SRC_A {
        match self.bits {
            0 => UART0SRC_A::_00,
            1 => UART0SRC_A::_01,
            2 => UART0SRC_A::_10,
            3 => UART0SRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == UART0SRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == UART0SRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == UART0SRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == UART0SRC_A::_11
    }
}
#[doc = "Write proxy for field `UART0SRC`"]
pub struct UART0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART0SRC_A::_00)
    }
    #[doc = "MCGFLLCLK clock or MCGPLLCLK/2 clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART0SRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART0SRC_A::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(UART0SRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    pub fn rtcclkoutsel(&self) -> RTCCLKOUTSEL_R {
        RTCCLKOUTSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 16 - PLL/FLL clock select"]
    #[inline(always)]
    pub fn pllfllsel(&self) -> PLLFLLSEL_R {
        PLLFLLSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    pub fn usbsrc(&self) -> USBSRC_R {
        USBSRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline(always)]
    pub fn tpmsrc(&self) -> TPMSRC_R {
        TPMSRC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - UART0 clock source select"]
    #[inline(always)]
    pub fn uart0src(&self) -> UART0SRC_R {
        UART0SRC_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    pub fn rtcclkoutsel(&mut self) -> RTCCLKOUTSEL_W {
        RTCCLKOUTSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W {
        CLKOUTSEL_W { w: self }
    }
    #[doc = "Bit 16 - PLL/FLL clock select"]
    #[inline(always)]
    pub fn pllfllsel(&mut self) -> PLLFLLSEL_W {
        PLLFLLSEL_W { w: self }
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    pub fn usbsrc(&mut self) -> USBSRC_W {
        USBSRC_W { w: self }
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline(always)]
    pub fn tpmsrc(&mut self) -> TPMSRC_W {
        TPMSRC_W { w: self }
    }
    #[doc = "Bits 26:27 - UART0 clock source select"]
    #[inline(always)]
    pub fn uart0src(&mut self) -> UART0SRC_W {
        UART0SRC_W { w: self }
    }
}
