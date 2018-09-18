#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::PMSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct PMSTATR {
    bits: u8,
}
impl PMSTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:6 - When debug is enabled, the PMSTAT will not update to STOP or VLPS When a PSTOP mode is enabled, the PMSTAT will not update to STOP or VLPS"]
    #[inline]
    pub fn pmstat(&self) -> PMSTATR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        PMSTATR { bits }
    }
}
