#[doc = "Reader of register C3"]
pub type R = crate::R<u8, super::C3>;
#[doc = "Writer for register C3"]
pub type W = crate::W<u8, super::C3>;
#[doc = "Register C3 `reset()`'s with value 0"]
impl crate::ResetValue for super::C3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIE_A {
    #[doc = "0: PF interrupts disabled; use polling)."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when PF is set."]
    _1 = 1,
}
impl From<PEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEIE`"]
pub type PEIE_R = crate::R<bool, PEIE_A>;
impl PEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEIE_A {
        match self.bits {
            false => PEIE_A::_0,
            true => PEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEIE_A::_1
    }
}
#[doc = "Write proxy for field `PEIE`"]
pub struct PEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PF interrupts disabled; use polling)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when PF is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEIE_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Framing Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    #[doc = "0: FE interrupts disabled; use polling)."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when FE is set."]
    _1 = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FEIE`"]
pub type FEIE_R = crate::R<bool, FEIE_A>;
impl FEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::_0,
            true => FEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEIE_A::_1
    }
}
#[doc = "Write proxy for field `FEIE`"]
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FE interrupts disabled; use polling)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when FE is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Noise Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEIE_A {
    #[doc = "0: NF interrupts disabled; use polling)."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when NF is set."]
    _1 = 1,
}
impl From<NEIE_A> for bool {
    #[inline(always)]
    fn from(variant: NEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NEIE`"]
pub type NEIE_R = crate::R<bool, NEIE_A>;
impl NEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEIE_A {
        match self.bits {
            false => NEIE_A::_0,
            true => NEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NEIE_A::_1
    }
}
#[doc = "Write proxy for field `NEIE`"]
pub struct NEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NF interrupts disabled; use polling)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NEIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when NF is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NEIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORIE_A {
    #[doc = "0: OR interrupts disabled; use polling."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when OR is set."]
    _1 = 1,
}
impl From<ORIE_A> for bool {
    #[inline(always)]
    fn from(variant: ORIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ORIE`"]
pub type ORIE_R = crate::R<bool, ORIE_A>;
impl ORIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORIE_A {
        match self.bits {
            false => ORIE_A::_0,
            true => ORIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORIE_A::_1
    }
}
#[doc = "Write proxy for field `ORIE`"]
pub struct ORIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ORIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ORIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OR interrupts disabled; use polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ORIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when OR is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ORIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Transmit Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXINV_A {
    #[doc = "0: Transmit data not inverted."]
    _0 = 0,
    #[doc = "1: Transmit data inverted."]
    _1 = 1,
}
impl From<TXINV_A> for bool {
    #[inline(always)]
    fn from(variant: TXINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXINV`"]
pub type TXINV_R = crate::R<bool, TXINV_A>;
impl TXINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXINV_A {
        match self.bits {
            false => TXINV_A::_0,
            true => TXINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXINV_A::_1
    }
}
#[doc = "Write proxy for field `TXINV`"]
pub struct TXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit data not inverted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXINV_A::_0)
    }
    #[doc = "Transmit data inverted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXINV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "TxD Pin Direction in Single-Wire Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDIR_A {
    #[doc = "0: TxD pin is an input in single-wire mode."]
    _0 = 0,
    #[doc = "1: TxD pin is an output in single-wire mode."]
    _1 = 1,
}
impl From<TXDIR_A> for bool {
    #[inline(always)]
    fn from(variant: TXDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXDIR`"]
pub type TXDIR_R = crate::R<bool, TXDIR_A>;
impl TXDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDIR_A {
        match self.bits {
            false => TXDIR_A::_0,
            true => TXDIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXDIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXDIR_A::_1
    }
}
#[doc = "Write proxy for field `TXDIR`"]
pub struct TXDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TxD pin is an input in single-wire mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDIR_A::_0)
    }
    #[doc = "TxD pin is an output in single-wire mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDIR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `T8`"]
pub type T8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T8`"]
pub struct T8_W<'a> {
    w: &'a mut W,
}
impl<'a> T8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `R8`"]
pub type R8_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Noise Error Interrupt Enable"]
    #[inline(always)]
    pub fn neie(&self) -> NEIE_R {
        NEIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&self) -> ORIE_R {
        ORIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit Data Inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TxD Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    pub fn txdir(&self) -> TXDIR_R {
        TXDIR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Ninth Data Bit for Transmitter"]
    #[inline(always)]
    pub fn t8(&self) -> T8_R {
        T8_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Ninth Data Bit for Receiver"]
    #[inline(always)]
    pub fn r8(&self) -> R8_R {
        R8_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W {
        PEIE_W { w: self }
    }
    #[doc = "Bit 1 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 2 - Noise Error Interrupt Enable"]
    #[inline(always)]
    pub fn neie(&mut self) -> NEIE_W {
        NEIE_W { w: self }
    }
    #[doc = "Bit 3 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&mut self) -> ORIE_W {
        ORIE_W { w: self }
    }
    #[doc = "Bit 4 - Transmit Data Inversion"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W {
        TXINV_W { w: self }
    }
    #[doc = "Bit 5 - TxD Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    pub fn txdir(&mut self) -> TXDIR_W {
        TXDIR_W { w: self }
    }
    #[doc = "Bit 6 - Ninth Data Bit for Transmitter"]
    #[inline(always)]
    pub fn t8(&mut self) -> T8_W {
        T8_W { w: self }
    }
}
