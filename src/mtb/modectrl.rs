#[doc = "Reader of register MODECTRL"]
pub type R = crate::R<u32, super::MODECTRL>;
#[doc = "Reader of field `MODECTRL`"]
pub type MODECTRL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hardwired to 0x0000_0000"]
    #[inline(always)]
    pub fn modectrl(&self) -> MODECTRL_R {
        MODECTRL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
