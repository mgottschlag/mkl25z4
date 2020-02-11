#[doc = "Reader of register COMPID%s"]
pub type R = crate::R<u32, super::COMPID>;
#[doc = "Reader of field `COMPID`"]
pub type COMPID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Component ID"]
    #[inline(always)]
    pub fn compid(&self) -> COMPID_R {
        COMPID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
