#[doc = "Reader of register DSR_BCR0"]
pub type R = crate::R<u32, super::DSR_BCR0>;
#[doc = "Writer for register DSR_BCR0"]
pub type W = crate::W<u32, super::DSR_BCR0>;
#[doc = "Register DSR_BCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSR_BCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BCR`"]
pub type BCR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BCR`"]
pub struct BCR_W<'a> {
    w: &'a mut W,
}
impl<'a> BCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Transactions done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "0: DMA transfer is not yet complete. Writing a 0 has no effect."]
    _0 = 0,
    #[doc = "1: DMA transfer completed. Writing a 1 to this bit clears all DMA status bits and should be used in an interrupt service routine to clear the DMA interrupt and error bits."]
    _1 = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, DONE_A>;
impl DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::_0,
            true => DONE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DONE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DONE_A::_1
    }
}
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA transfer is not yet complete. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DONE_A::_0)
    }
    #[doc = "DMA transfer completed. Writing a 1 to this bit clears all DMA status bits and should be used in an interrupt service routine to clear the DMA interrupt and error bits."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DONE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSY_A {
    #[doc = "0: DMA channel is inactive. Cleared when the DMA has finished the last transaction."]
    _0 = 0,
    #[doc = "1: BSY is set the first time the channel is enabled after a transfer is initiated."]
    _1 = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BSY`"]
pub type BSY_R = crate::R<bool, BSY_A>;
impl BSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::_0,
            true => BSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSY_A::_1
    }
}
#[doc = "Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQ_A {
    #[doc = "0: No request is pending or the channel is currently active. Cleared when the channel is selected."]
    _0 = 0,
    #[doc = "1: The DMA channel has a transfer remaining and the channel is not selected."]
    _1 = 1,
}
impl From<REQ_A> for bool {
    #[inline(always)]
    fn from(variant: REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REQ`"]
pub type REQ_R = crate::R<bool, REQ_A>;
impl REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQ_A {
        match self.bits {
            false => REQ_A::_0,
            true => REQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REQ_A::_1
    }
}
#[doc = "Bus error on destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BED_A {
    #[doc = "0: No bus error occurred."]
    _0 = 0,
    #[doc = "1: The DMA channel terminated with a bus error during the write portion of a transfer."]
    _1 = 1,
}
impl From<BED_A> for bool {
    #[inline(always)]
    fn from(variant: BED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BED`"]
pub type BED_R = crate::R<bool, BED_A>;
impl BED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BED_A {
        match self.bits {
            false => BED_A::_0,
            true => BED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BED_A::_1
    }
}
#[doc = "Bus error on source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BES_A {
    #[doc = "0: No bus error occurred."]
    _0 = 0,
    #[doc = "1: The DMA channel terminated with a bus error during the read portion of a transfer."]
    _1 = 1,
}
impl From<BES_A> for bool {
    #[inline(always)]
    fn from(variant: BES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BES`"]
pub type BES_R = crate::R<bool, BES_A>;
impl BES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BES_A {
        match self.bits {
            false => BES_A::_0,
            true => BES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BES_A::_1
    }
}
#[doc = "Configuration error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CE_A {
    #[doc = "0: No configuration error exists."]
    _0 = 0,
    #[doc = "1: A configuration error has occurred."]
    _1 = 1,
}
impl From<CE_A> for bool {
    #[inline(always)]
    fn from(variant: CE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CE`"]
pub type CE_R = crate::R<bool, CE_A>;
impl CE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CE_A {
        match self.bits {
            false => CE_A::_0,
            true => CE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CE_A::_1
    }
}
impl R {
    #[doc = "Bits 0:23 - This field contains the number of bytes yet to be transferred for a given block"]
    #[inline(always)]
    pub fn bcr(&self) -> BCR_R {
        BCR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Transactions done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Request"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Bus error on destination"]
    #[inline(always)]
    pub fn bed(&self) -> BED_R {
        BED_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Bus error on source"]
    #[inline(always)]
    pub fn bes(&self) -> BES_R {
        BES_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Configuration error"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - This field contains the number of bytes yet to be transferred for a given block"]
    #[inline(always)]
    pub fn bcr(&mut self) -> BCR_W {
        BCR_W { w: self }
    }
    #[doc = "Bit 24 - Transactions done"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
}
