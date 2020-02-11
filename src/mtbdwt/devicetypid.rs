#[doc = "Reader of register DEVICETYPID"]
pub type R = crate::R<u32, super::DEVICETYPID>;
#[doc = "Reader of field `DEVICETYPID`"]
pub type DEVICETYPID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hardwired to 0x0000_0004."]
    #[inline(always)]
    pub fn devicetypid(&self) -> DEVICETYPID_R {
        DEVICETYPID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
