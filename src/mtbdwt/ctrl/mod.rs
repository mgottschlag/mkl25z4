#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CTRL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DWTCFGCTRLR {
    bits: u32,
}
impl DWTCFGCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NUMCMPR {
    bits: u8,
}
impl NUMCMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:27 - DWT configuration controls"]
    #[inline]
    pub fn dwtcfgctrl(&self) -> DWTCFGCTRLR {
        let bits = {
            const MASK: u32 = 268435455;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DWTCFGCTRLR { bits }
    }
    #[doc = "Bits 28:31 - Number of comparators"]
    #[inline]
    pub fn numcmp(&self) -> NUMCMPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUMCMPR { bits }
    }
}
