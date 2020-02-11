#[doc = "Reader of register FCFG2"]
pub type R = crate::R<u32, super::FCFG2>;
#[doc = "Reader of field `MAXADDR0`"]
pub type MAXADDR0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:30 - Max address block"]
    #[inline(always)]
    pub fn maxaddr0(&self) -> MAXADDR0_R {
        MAXADDR0_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
