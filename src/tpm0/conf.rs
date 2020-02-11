#[doc = "Reader of register CONF"]
pub type R = crate::R<u32, super::CONF>;
#[doc = "Writer for register CONF"]
pub type W = crate::W<u32, super::CONF>;
#[doc = "Register CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEEN_A {
    #[doc = "0: Internal LPTPM counter continues in Doze mode."]
    _0 = 0,
    #[doc = "1: Internal LPTPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    _1 = 1,
}
impl From<DOZEEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOZEEN`"]
pub type DOZEEN_R = crate::R<bool, DOZEEN_A>;
impl DOZEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEEN_A {
        match self.bits {
            false => DOZEEN_A::_0,
            true => DOZEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOZEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOZEEN_A::_1
    }
}
#[doc = "Write proxy for field `DOZEEN`"]
pub struct DOZEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal LPTPM counter continues in Doze mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZEEN_A::_0)
    }
    #[doc = "Internal LPTPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZEEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Debug Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBGMODE_A {
    #[doc = "0: LPTPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    _00 = 0,
    #[doc = "3: LPTPM counter continues in debug mode."]
    _11 = 3,
}
impl From<DBGMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DBGMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DBGMODE`"]
pub type DBGMODE_R = crate::R<u8, DBGMODE_A>;
impl DBGMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DBGMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DBGMODE_A::_00),
            3 => Val(DBGMODE_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DBGMODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DBGMODE_A::_11
    }
}
#[doc = "Write proxy for field `DBGMODE`"]
pub struct DBGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPTPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DBGMODE_A::_00)
    }
    #[doc = "LPTPM counter continues in debug mode."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DBGMODE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Global time base enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEEN_A {
    #[doc = "0: All channels use the internally generated LPTPM counter as their timebase"]
    _0 = 0,
    #[doc = "1: All channels use an externally generated global timebase as their timebase"]
    _1 = 1,
}
impl From<GTBEEN_A> for bool {
    #[inline(always)]
    fn from(variant: GTBEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GTBEEN`"]
pub type GTBEEN_R = crate::R<bool, GTBEEN_A>;
impl GTBEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTBEEN_A {
        match self.bits {
            false => GTBEEN_A::_0,
            true => GTBEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GTBEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GTBEEN_A::_1
    }
}
#[doc = "Write proxy for field `GTBEEN`"]
pub struct GTBEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GTBEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GTBEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All channels use the internally generated LPTPM counter as their timebase"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBEEN_A::_0)
    }
    #[doc = "All channels use an externally generated global timebase as their timebase"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBEEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Counter Start on Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSOT_A {
    #[doc = "0: LPTPM counter starts to increment immediately, once it is enabled."]
    _0 = 0,
    #[doc = "1: LPTPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    _1 = 1,
}
impl From<CSOT_A> for bool {
    #[inline(always)]
    fn from(variant: CSOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSOT`"]
pub type CSOT_R = crate::R<bool, CSOT_A>;
impl CSOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSOT_A {
        match self.bits {
            false => CSOT_A::_0,
            true => CSOT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSOT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSOT_A::_1
    }
}
#[doc = "Write proxy for field `CSOT`"]
pub struct CSOT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPTPM counter starts to increment immediately, once it is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSOT_A::_0)
    }
    #[doc = "LPTPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSOT_A::_1)
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
#[doc = "Counter Stop On Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSOO_A {
    #[doc = "0: LPTPM counter continues incrementing or decrementing after overflow"]
    _0 = 0,
    #[doc = "1: LPTPM counter stops incrementing or decrementing after overflow."]
    _1 = 1,
}
impl From<CSOO_A> for bool {
    #[inline(always)]
    fn from(variant: CSOO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSOO`"]
pub type CSOO_R = crate::R<bool, CSOO_A>;
impl CSOO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSOO_A {
        match self.bits {
            false => CSOO_A::_0,
            true => CSOO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSOO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSOO_A::_1
    }
}
#[doc = "Write proxy for field `CSOO`"]
pub struct CSOO_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPTPM counter continues incrementing or decrementing after overflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSOO_A::_0)
    }
    #[doc = "LPTPM counter stops incrementing or decrementing after overflow."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSOO_A::_1)
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
#[doc = "Counter Reload On Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CROT_A {
    #[doc = "0: Counter is not reloaded due to a rising edge on the selected input trigger"]
    _0 = 0,
    #[doc = "1: Counter is reloaded when a rising edge is detected on the selected input trigger"]
    _1 = 1,
}
impl From<CROT_A> for bool {
    #[inline(always)]
    fn from(variant: CROT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CROT`"]
pub type CROT_R = crate::R<bool, CROT_A>;
impl CROT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CROT_A {
        match self.bits {
            false => CROT_A::_0,
            true => CROT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CROT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CROT_A::_1
    }
}
#[doc = "Write proxy for field `CROT`"]
pub struct CROT_W<'a> {
    w: &'a mut W,
}
impl<'a> CROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CROT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter is not reloaded due to a rising edge on the selected input trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CROT_A::_0)
    }
    #[doc = "Counter is reloaded when a rising edge is detected on the selected input trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CROT_A::_1)
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
#[doc = "Reader of field `TRGSEL`"]
pub type TRGSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRGSEL`"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Doze Enable"]
    #[inline(always)]
    pub fn dozeen(&self) -> DOZEEN_R {
        DOZEEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline(always)]
    pub fn dbgmode(&self) -> DBGMODE_R {
        DBGMODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Global time base enable"]
    #[inline(always)]
    pub fn gtbeen(&self) -> GTBEEN_R {
        GTBEEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Counter Start on Trigger"]
    #[inline(always)]
    pub fn csot(&self) -> CSOT_R {
        CSOT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Counter Stop On Overflow"]
    #[inline(always)]
    pub fn csoo(&self) -> CSOO_R {
        CSOO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Counter Reload On Trigger"]
    #[inline(always)]
    pub fn crot(&self) -> CROT_R {
        CROT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Doze Enable"]
    #[inline(always)]
    pub fn dozeen(&mut self) -> DOZEEN_W {
        DOZEEN_W { w: self }
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline(always)]
    pub fn dbgmode(&mut self) -> DBGMODE_W {
        DBGMODE_W { w: self }
    }
    #[doc = "Bit 9 - Global time base enable"]
    #[inline(always)]
    pub fn gtbeen(&mut self) -> GTBEEN_W {
        GTBEEN_W { w: self }
    }
    #[doc = "Bit 16 - Counter Start on Trigger"]
    #[inline(always)]
    pub fn csot(&mut self) -> CSOT_W {
        CSOT_W { w: self }
    }
    #[doc = "Bit 17 - Counter Stop On Overflow"]
    #[inline(always)]
    pub fn csoo(&mut self) -> CSOO_W {
        CSOO_W { w: self }
    }
    #[doc = "Bit 18 - Counter Reload On Trigger"]
    #[inline(always)]
    pub fn crot(&mut self) -> CROT_W {
        CROT_W { w: self }
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W {
        TRGSEL_W { w: self }
    }
}
