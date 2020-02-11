#[doc = "Reader of register BASE"]
pub type R = crate::R<u32, super::BASE>;
#[doc = "Reader of field `BASEADDR`"]
pub type BASEADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This value is defined with a hardwired signal and the expression: 0x2000_0000 - (RAM_Size/4)"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
