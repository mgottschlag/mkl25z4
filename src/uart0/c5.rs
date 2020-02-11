#[doc = "Reader of register C5"]
pub type R = crate::R<u8, super::C5>;
#[doc = "Writer for register C5"]
pub type W = crate::W<u8, super::C5>;
#[doc = "Register C5 `reset()`'s with value 0"]
impl crate::ResetValue for super::C5 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Resynchronization Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESYNCDIS_A {
    #[doc = "0: Resynchronization during received data word is supported"]
    _0 = 0,
    #[doc = "1: Resynchronization during received data word is disabled"]
    _1 = 1,
}
impl From<RESYNCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RESYNCDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESYNCDIS`"]
pub type RESYNCDIS_R = crate::R<bool, RESYNCDIS_A>;
impl RESYNCDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESYNCDIS_A {
        match self.bits {
            false => RESYNCDIS_A::_0,
            true => RESYNCDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESYNCDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESYNCDIS_A::_1
    }
}
#[doc = "Write proxy for field `RESYNCDIS`"]
pub struct RESYNCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESYNCDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESYNCDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resynchronization during received data word is supported"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESYNCDIS_A::_0)
    }
    #[doc = "Resynchronization during received data word is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESYNCDIS_A::_1)
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
#[doc = "Both Edge Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOTHEDGE_A {
    #[doc = "0: Receiver samples input data using the rising edge of the baud rate clock."]
    _0 = 0,
    #[doc = "1: Receiver samples input data using the rising and falling edge of the baud rate clock."]
    _1 = 1,
}
impl From<BOTHEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: BOTHEDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOTHEDGE`"]
pub type BOTHEDGE_R = crate::R<bool, BOTHEDGE_A>;
impl BOTHEDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOTHEDGE_A {
        match self.bits {
            false => BOTHEDGE_A::_0,
            true => BOTHEDGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOTHEDGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOTHEDGE_A::_1
    }
}
#[doc = "Write proxy for field `BOTHEDGE`"]
pub struct BOTHEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOTHEDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOTHEDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOTHEDGE_A::_0)
    }
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOTHEDGE_A::_1)
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
#[doc = "Receiver Full DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMAE_A {
    #[doc = "0: DMA request disabled."]
    _0 = 0,
    #[doc = "1: DMA request enabled."]
    _1 = 1,
}
impl From<RDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: RDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDMAE`"]
pub type RDMAE_R = crate::R<bool, RDMAE_A>;
impl RDMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMAE_A {
        match self.bits {
            false => RDMAE_A::_0,
            true => RDMAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDMAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDMAE_A::_1
    }
}
#[doc = "Write proxy for field `RDMAE`"]
pub struct RDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDMAE_A::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDMAE_A::_1)
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
#[doc = "Transmitter DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMAE_A {
    #[doc = "0: DMA request disabled."]
    _0 = 0,
    #[doc = "1: DMA request enabled."]
    _1 = 1,
}
impl From<TDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: TDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDMAE`"]
pub type TDMAE_R = crate::R<bool, TDMAE_A>;
impl TDMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDMAE_A {
        match self.bits {
            false => TDMAE_A::_0,
            true => TDMAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDMAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDMAE_A::_1
    }
}
#[doc = "Write proxy for field `TDMAE`"]
pub struct TDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDMAE_A::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDMAE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Resynchronization Disable"]
    #[inline(always)]
    pub fn resyncdis(&self) -> RESYNCDIS_R {
        RESYNCDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Both Edge Sampling"]
    #[inline(always)]
    pub fn bothedge(&self) -> BOTHEDGE_R {
        BOTHEDGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receiver Full DMA Enable"]
    #[inline(always)]
    pub fn rdmae(&self) -> RDMAE_R {
        RDMAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmitter DMA Enable"]
    #[inline(always)]
    pub fn tdmae(&self) -> TDMAE_R {
        TDMAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Resynchronization Disable"]
    #[inline(always)]
    pub fn resyncdis(&mut self) -> RESYNCDIS_W {
        RESYNCDIS_W { w: self }
    }
    #[doc = "Bit 1 - Both Edge Sampling"]
    #[inline(always)]
    pub fn bothedge(&mut self) -> BOTHEDGE_W {
        BOTHEDGE_W { w: self }
    }
    #[doc = "Bit 5 - Receiver Full DMA Enable"]
    #[inline(always)]
    pub fn rdmae(&mut self) -> RDMAE_W {
        RDMAE_W { w: self }
    }
    #[doc = "Bit 7 - Transmitter DMA Enable"]
    #[inline(always)]
    pub fn tdmae(&mut self) -> TDMAE_W {
        TDMAE_W { w: self }
    }
}
