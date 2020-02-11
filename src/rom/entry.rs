#[doc = "Reader of register ENTRY%s"]
pub type R = crate::R<u32, super::ENTRY>;
#[doc = "Reader of field `ENTRY`"]
pub type ENTRY_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ENTRY"]
    #[inline(always)]
    pub fn entry(&self) -> ENTRY_R {
        ENTRY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
