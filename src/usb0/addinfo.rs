#[doc = "Reader of register ADDINFO"]
pub type R = crate::R<u8, super::ADDINFO>;
#[doc = "Reader of field `IEHOST`"]
pub type IEHOST_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQNUM`"]
pub type IRQNUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - When this bit is set, the USB peripheral is operating in host mode."]
    #[inline(always)]
    pub fn iehost(&self) -> IEHOST_R {
        IEHOST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - Assigned Interrupt Request Number"]
    #[inline(always)]
    pub fn irqnum(&self) -> IRQNUM_R {
        IRQNUM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
