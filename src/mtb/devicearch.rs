#[doc = "Reader of register DEVICEARCH"]
pub type R = crate::R<u32, super::DEVICEARCH>;
#[doc = "Reader of field `DEVICEARCH`"]
pub type DEVICEARCH_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hardwired to 0x4770_0A31."]
    #[inline(always)]
    pub fn devicearch(&self) -> DEVICEARCH_R {
        DEVICEARCH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
