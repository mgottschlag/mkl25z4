#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BASE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct BASEADDRR {
    bits: u32,
}
impl BASEADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - This value is defined with a hardwired signal and the expression: 0x2000_0000 - (RAM_Size/4)"]
    #[inline]
    pub fn baseaddr(&self) -> BASEADDRR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        BASEADDRR { bits }
    }
}
