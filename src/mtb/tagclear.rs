#[doc = "Reader of register TAGCLEAR"]
pub type R = crate::R<u32, super::TAGCLEAR>;
#[doc = "Reader of field `TAGCLEAR`"]
pub type TAGCLEAR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hardwired to 0x0000_0000"]
    #[inline(always)]
    pub fn tagclear(&self) -> TAGCLEAR_R {
        TAGCLEAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
