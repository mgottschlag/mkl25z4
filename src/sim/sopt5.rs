#[doc = "Reader of register SOPT5"]
pub type R = crate::R<u32, super::SOPT5>;
#[doc = "Writer for register SOPT5"]
pub type W = crate::W<u32, super::SOPT5>;
#[doc = "Register SOPT5 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "UART0 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART0TXSRC_A {
    #[doc = "0: UART0_TX pin"]
    _00 = 0,
    #[doc = "1: UART0_TX pin modulated with TPM1 channel 0 output"]
    _01 = 1,
    #[doc = "2: UART0_TX pin modulated with TPM2 channel 0 output"]
    _10 = 2,
}
impl From<UART0TXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART0TXSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UART0TXSRC`"]
pub type UART0TXSRC_R = crate::R<u8, UART0TXSRC_A>;
impl UART0TXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UART0TXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UART0TXSRC_A::_00),
            1 => Val(UART0TXSRC_A::_01),
            2 => Val(UART0TXSRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == UART0TXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == UART0TXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == UART0TXSRC_A::_10
    }
}
#[doc = "Write proxy for field `UART0TXSRC`"]
pub struct UART0TXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0TXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0TXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "UART0_TX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART0TXSRC_A::_00)
    }
    #[doc = "UART0_TX pin modulated with TPM1 channel 0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART0TXSRC_A::_01)
    }
    #[doc = "UART0_TX pin modulated with TPM2 channel 0 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART0TXSRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "UART0 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0RXSRC_A {
    #[doc = "0: UART0_RX pin"]
    _0 = 0,
    #[doc = "1: CMP0 output"]
    _1 = 1,
}
impl From<UART0RXSRC_A> for bool {
    #[inline(always)]
    fn from(variant: UART0RXSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART0RXSRC`"]
pub type UART0RXSRC_R = crate::R<bool, UART0RXSRC_A>;
impl UART0RXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0RXSRC_A {
        match self.bits {
            false => UART0RXSRC_A::_0,
            true => UART0RXSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART0RXSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART0RXSRC_A::_1
    }
}
#[doc = "Write proxy for field `UART0RXSRC`"]
pub struct UART0RXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0RXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0RXSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART0_RX pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0RXSRC_A::_0)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0RXSRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "UART1 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART1TXSRC_A {
    #[doc = "0: UART1_TX pin"]
    _00 = 0,
    #[doc = "1: UART1_TX pin modulated with TPM1 channel 0 output"]
    _01 = 1,
    #[doc = "2: UART1_TX pin modulated with TPM2 channel 0 output"]
    _10 = 2,
}
impl From<UART1TXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART1TXSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UART1TXSRC`"]
pub type UART1TXSRC_R = crate::R<u8, UART1TXSRC_A>;
impl UART1TXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UART1TXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UART1TXSRC_A::_00),
            1 => Val(UART1TXSRC_A::_01),
            2 => Val(UART1TXSRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == UART1TXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == UART1TXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == UART1TXSRC_A::_10
    }
}
#[doc = "Write proxy for field `UART1TXSRC`"]
pub struct UART1TXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1TXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1TXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "UART1_TX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART1TXSRC_A::_00)
    }
    #[doc = "UART1_TX pin modulated with TPM1 channel 0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART1TXSRC_A::_01)
    }
    #[doc = "UART1_TX pin modulated with TPM2 channel 0 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART1TXSRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "UART1 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1RXSRC_A {
    #[doc = "0: UART1_RX pin"]
    _0 = 0,
    #[doc = "1: CMP0 output"]
    _1 = 1,
}
impl From<UART1RXSRC_A> for bool {
    #[inline(always)]
    fn from(variant: UART1RXSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART1RXSRC`"]
pub type UART1RXSRC_R = crate::R<bool, UART1RXSRC_A>;
impl UART1RXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1RXSRC_A {
        match self.bits {
            false => UART1RXSRC_A::_0,
            true => UART1RXSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART1RXSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART1RXSRC_A::_1
    }
}
#[doc = "Write proxy for field `UART1RXSRC`"]
pub struct UART1RXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1RXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1RXSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART1_RX pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1RXSRC_A::_0)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1RXSRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "UART0 Open Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0ODE_A {
    #[doc = "0: Open drain is disabled on UART0"]
    _0 = 0,
    #[doc = "1: Open drain is enabled on UART0"]
    _1 = 1,
}
impl From<UART0ODE_A> for bool {
    #[inline(always)]
    fn from(variant: UART0ODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART0ODE`"]
pub type UART0ODE_R = crate::R<bool, UART0ODE_A>;
impl UART0ODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0ODE_A {
        match self.bits {
            false => UART0ODE_A::_0,
            true => UART0ODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART0ODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART0ODE_A::_1
    }
}
#[doc = "Write proxy for field `UART0ODE`"]
pub struct UART0ODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0ODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0ODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Open drain is disabled on UART0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0ODE_A::_0)
    }
    #[doc = "Open drain is enabled on UART0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0ODE_A::_1)
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
#[doc = "UART1 Open Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1ODE_A {
    #[doc = "0: Open drain is disabled on UART1"]
    _0 = 0,
    #[doc = "1: Open drain is enabled on UART1"]
    _1 = 1,
}
impl From<UART1ODE_A> for bool {
    #[inline(always)]
    fn from(variant: UART1ODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART1ODE`"]
pub type UART1ODE_R = crate::R<bool, UART1ODE_A>;
impl UART1ODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1ODE_A {
        match self.bits {
            false => UART1ODE_A::_0,
            true => UART1ODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART1ODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART1ODE_A::_1
    }
}
#[doc = "Write proxy for field `UART1ODE`"]
pub struct UART1ODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1ODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1ODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Open drain is disabled on UART1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1ODE_A::_0)
    }
    #[doc = "Open drain is enabled on UART1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1ODE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "UART2 Open Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2ODE_A {
    #[doc = "0: Open drain is disabled on UART2"]
    _0 = 0,
    #[doc = "1: Open drain is enabled on UART2"]
    _1 = 1,
}
impl From<UART2ODE_A> for bool {
    #[inline(always)]
    fn from(variant: UART2ODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART2ODE`"]
pub type UART2ODE_R = crate::R<bool, UART2ODE_A>;
impl UART2ODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2ODE_A {
        match self.bits {
            false => UART2ODE_A::_0,
            true => UART2ODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART2ODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART2ODE_A::_1
    }
}
#[doc = "Write proxy for field `UART2ODE`"]
pub struct UART2ODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2ODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2ODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Open drain is disabled on UART2"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART2ODE_A::_0)
    }
    #[doc = "Open drain is enabled on UART2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART2ODE_A::_1)
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
impl R {
    #[doc = "Bits 0:1 - UART0 transmit data source select"]
    #[inline(always)]
    pub fn uart0txsrc(&self) -> UART0TXSRC_R {
        UART0TXSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - UART0 receive data source select"]
    #[inline(always)]
    pub fn uart0rxsrc(&self) -> UART0RXSRC_R {
        UART0RXSRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - UART1 transmit data source select"]
    #[inline(always)]
    pub fn uart1txsrc(&self) -> UART1TXSRC_R {
        UART1TXSRC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - UART1 receive data source select"]
    #[inline(always)]
    pub fn uart1rxsrc(&self) -> UART1RXSRC_R {
        UART1RXSRC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 16 - UART0 Open Drain Enable"]
    #[inline(always)]
    pub fn uart0ode(&self) -> UART0ODE_R {
        UART0ODE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - UART1 Open Drain Enable"]
    #[inline(always)]
    pub fn uart1ode(&self) -> UART1ODE_R {
        UART1ODE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - UART2 Open Drain Enable"]
    #[inline(always)]
    pub fn uart2ode(&self) -> UART2ODE_R {
        UART2ODE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART0 transmit data source select"]
    #[inline(always)]
    pub fn uart0txsrc(&mut self) -> UART0TXSRC_W {
        UART0TXSRC_W { w: self }
    }
    #[doc = "Bit 2 - UART0 receive data source select"]
    #[inline(always)]
    pub fn uart0rxsrc(&mut self) -> UART0RXSRC_W {
        UART0RXSRC_W { w: self }
    }
    #[doc = "Bits 4:5 - UART1 transmit data source select"]
    #[inline(always)]
    pub fn uart1txsrc(&mut self) -> UART1TXSRC_W {
        UART1TXSRC_W { w: self }
    }
    #[doc = "Bit 6 - UART1 receive data source select"]
    #[inline(always)]
    pub fn uart1rxsrc(&mut self) -> UART1RXSRC_W {
        UART1RXSRC_W { w: self }
    }
    #[doc = "Bit 16 - UART0 Open Drain Enable"]
    #[inline(always)]
    pub fn uart0ode(&mut self) -> UART0ODE_W {
        UART0ODE_W { w: self }
    }
    #[doc = "Bit 17 - UART1 Open Drain Enable"]
    #[inline(always)]
    pub fn uart1ode(&mut self) -> UART1ODE_W {
        UART1ODE_W { w: self }
    }
    #[doc = "Bit 18 - UART2 Open Drain Enable"]
    #[inline(always)]
    pub fn uart2ode(&mut self) -> UART2ODE_W {
        UART2ODE_W { w: self }
    }
}
