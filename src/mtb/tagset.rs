#[doc = "Reader of register TAGSET"]
pub type R = crate::R<u32, super::TAGSET>;
#[doc = "Reader of field `TAGSET`"]
pub type TAGSET_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hardwired to 0x0000_0000"]
    #[inline(always)]
    pub fn tagset(&self) -> TAGSET_R {
        TAGSET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
