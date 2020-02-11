#[doc = "Reader of register C1"]
pub type R = crate::R<u8, super::C1>;
#[doc = "Writer for register C1"]
pub type W = crate::W<u8, super::C1>;
#[doc = "Register C1 `reset()`'s with value 0"]
impl crate::ResetValue for super::C1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DAC Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFEN_A {
    #[doc = "0: Buffer read pointer is disabled. The converted data is always the first word of the buffer."]
    _0 = 0,
    #[doc = "1: Buffer read pointer is enabled. The converted data is the word that the read pointer points to. It means converted data can be from any word of the buffer."]
    _1 = 1,
}
impl From<DACBFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DACBFEN`"]
pub type DACBFEN_R = crate::R<bool, DACBFEN_A>;
impl DACBFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFEN_A {
        match self.bits {
            false => DACBFEN_A::_0,
            true => DACBFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBFEN_A::_1
    }
}
#[doc = "Write proxy for field `DACBFEN`"]
pub struct DACBFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Buffer read pointer is disabled. The converted data is always the first word of the buffer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFEN_A::_0)
    }
    #[doc = "Buffer read pointer is enabled. The converted data is the word that the read pointer points to. It means converted data can be from any word of the buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFEN_A::_1)
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
#[doc = "DAC Buffer Work Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFMD_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: One-Time Scan mode"]
    _1 = 1,
}
impl From<DACBFMD_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DACBFMD`"]
pub type DACBFMD_R = crate::R<bool, DACBFMD_A>;
impl DACBFMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFMD_A {
        match self.bits {
            false => DACBFMD_A::_0,
            true => DACBFMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBFMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBFMD_A::_1
    }
}
#[doc = "Write proxy for field `DACBFMD`"]
pub struct DACBFMD_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBFMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFMD_A::_0)
    }
    #[doc = "One-Time Scan mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFMD_A::_1)
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
#[doc = "DMA Enable Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA is disabled."]
    _0 = 0,
    #[doc = "1: DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time."]
    _1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEN_A::_1
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
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
    #[doc = "Bit 0 - DAC Buffer Enable"]
    #[inline(always)]
    pub fn dacbfen(&self) -> DACBFEN_R {
        DACBFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - DAC Buffer Work Mode Select"]
    #[inline(always)]
    pub fn dacbfmd(&self) -> DACBFMD_R {
        DACBFMD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA Enable Select"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Enable"]
    #[inline(always)]
    pub fn dacbfen(&mut self) -> DACBFEN_W {
        DACBFEN_W { w: self }
    }
    #[doc = "Bit 2 - DAC Buffer Work Mode Select"]
    #[inline(always)]
    pub fn dacbfmd(&mut self) -> DACBFMD_W {
        DACBFMD_W { w: self }
    }
    #[doc = "Bit 7 - DMA Enable Select"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
}
