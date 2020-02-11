#[doc = "Reader of register SYSACCESS"]
pub type R = crate::R<u32, super::SYSACCESS>;
#[doc = "Reader of field `SYSACCESS`"]
pub type SYSACCESS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hardwired to 0x0000_0001"]
    #[inline(always)]
    pub fn sysaccess(&self) -> SYSACCESS_R {
        SYSACCESS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
