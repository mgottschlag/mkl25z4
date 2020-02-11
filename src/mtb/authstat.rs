#[doc = "Reader of register AUTHSTAT"]
pub type R = crate::R<u32, super::AUTHSTAT>;
#[doc = "Reader of field `BIT0`"]
pub type BIT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BIT1`"]
pub type BIT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BIT2`"]
pub type BIT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `BIT3`"]
pub type BIT3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Connected to DBGEN."]
    #[inline(always)]
    pub fn bit0(&self) -> BIT0_R {
        BIT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hardwired to 1."]
    #[inline(always)]
    pub fn bit1(&self) -> BIT1_R {
        BIT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Connected to NIDEN or DBGEN signal."]
    #[inline(always)]
    pub fn bit2(&self) -> BIT2_R {
        BIT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Hardwired to 1."]
    #[inline(always)]
    pub fn bit3(&self) -> BIT3_R {
        BIT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
