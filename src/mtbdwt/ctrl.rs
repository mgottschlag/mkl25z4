#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Reader of field `DWTCFGCTRL`"]
pub type DWTCFGCTRL_R = crate::R<u32, u32>;
#[doc = "Reader of field `NUMCMP`"]
pub type NUMCMP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:27 - DWT configuration controls"]
    #[inline(always)]
    pub fn dwtcfgctrl(&self) -> DWTCFGCTRL_R {
        DWTCFGCTRL_R::new((self.bits & 0x0fff_ffff) as u32)
    }
    #[doc = "Bits 28:31 - Number of comparators"]
    #[inline(always)]
    pub fn numcmp(&self) -> NUMCMP_R {
        NUMCMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
