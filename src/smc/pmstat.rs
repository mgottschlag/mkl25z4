#[doc = "Reader of register PMSTAT"]
pub type R = crate::R<u8, super::PMSTAT>;
#[doc = "Reader of field `PMSTAT`"]
pub type PMSTAT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - When debug is enabled, the PMSTAT will not update to STOP or VLPS When a PSTOP mode is enabled, the PMSTAT will not update to STOP or VLPS"]
    #[inline(always)]
    pub fn pmstat(&self) -> PMSTAT_R {
        PMSTAT_R::new((self.bits & 0x7f) as u8)
    }
}
