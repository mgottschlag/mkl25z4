#[doc = "Reader of register TABLEMARK"]
pub type R = crate::R<u32, super::TABLEMARK>;
#[doc = "Reader of field `MARK`"]
pub type MARK_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hardwired to 0x0000_0000"]
    #[inline(always)]
    pub fn mark(&self) -> MARK_R {
        MARK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
