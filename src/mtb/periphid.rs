#[doc = "Reader of register PERIPHID%s"]
pub type R = crate::R<u32, super::PERIPHID>;
#[doc = "Reader of field `PERIPHID`"]
pub type PERIPHID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral ID4 is hardwired to 0x0000_0004; ID0 to 0x0000_0032; ID1 to 0x0000_00B9; ID2 to 0x0000_000B; and all the others to 0x0000_0000"]
    #[inline(always)]
    pub fn periphid(&self) -> PERIPHID_R {
        PERIPHID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
