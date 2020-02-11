#[doc = "Reader of register DEVICECFG"]
pub type R = crate::R<u32, super::DEVICECFG>;
#[doc = "Reader of field `DEVICECFG`"]
pub type DEVICECFG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hardwired to 0x0000_0000."]
    #[inline(always)]
    pub fn devicecfg(&self) -> DEVICECFG_R {
        DEVICECFG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
