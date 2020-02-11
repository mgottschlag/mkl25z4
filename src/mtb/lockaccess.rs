#[doc = "Reader of register LOCKACCESS"]
pub type R = crate::R<u32, super::LOCKACCESS>;
#[doc = "Reader of field `LOCKACCESS`"]
pub type LOCKACCESS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hardwired to 0x0000_0000"]
    #[inline(always)]
    pub fn lockaccess(&self) -> LOCKACCESS_R {
        LOCKACCESS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
