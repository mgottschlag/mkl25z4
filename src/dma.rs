#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - Source Address Register"]
    pub sar0: SAR0,
    #[doc = "0x104 - Destination Address Register"]
    pub dar0: DAR0,
    _reserved_2_dsr0: [u8; 4usize],
    #[doc = "0x10c - DMA Control Register"]
    pub dcr0: DCR0,
    #[doc = "0x110 - Source Address Register"]
    pub sar1: SAR1,
    #[doc = "0x114 - Destination Address Register"]
    pub dar1: DAR1,
    _reserved_6_dsr1: [u8; 4usize],
    #[doc = "0x11c - DMA Control Register"]
    pub dcr1: DCR1,
    #[doc = "0x120 - Source Address Register"]
    pub sar2: SAR2,
    #[doc = "0x124 - Destination Address Register"]
    pub dar2: DAR2,
    _reserved_10_dsr2: [u8; 4usize],
    #[doc = "0x12c - DMA Control Register"]
    pub dcr2: DCR2,
    #[doc = "0x130 - Source Address Register"]
    pub sar3: SAR3,
    #[doc = "0x134 - Destination Address Register"]
    pub dar3: DAR3,
    _reserved_14_dsr3: [u8; 4usize],
    #[doc = "0x13c - DMA Control Register"]
    pub dcr3: DCR3,
}
impl RegisterBlock {
    #[doc = "0x108 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub fn dsr_bcr0(&self) -> &DSR_BCR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const DSR_BCR0) }
    }
    #[doc = "0x108 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub fn dsr_bcr0_mut(&self) -> &mut DSR_BCR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(264usize) as *mut DSR_BCR0) }
    }
    #[doc = "0x10b - DMA_DSR0 register."]
    #[inline(always)]
    pub fn dsr0(&self) -> &DSR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(267usize) as *const DSR0) }
    }
    #[doc = "0x10b - DMA_DSR0 register."]
    #[inline(always)]
    pub fn dsr0_mut(&self) -> &mut DSR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(267usize) as *mut DSR0) }
    }
    #[doc = "0x118 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub fn dsr_bcr1(&self) -> &DSR_BCR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(280usize) as *const DSR_BCR1) }
    }
    #[doc = "0x118 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub fn dsr_bcr1_mut(&self) -> &mut DSR_BCR1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(280usize) as *mut DSR_BCR1) }
    }
    #[doc = "0x11b - DMA_DSR1 register."]
    #[inline(always)]
    pub fn dsr1(&self) -> &DSR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(283usize) as *const DSR1) }
    }
    #[doc = "0x11b - DMA_DSR1 register."]
    #[inline(always)]
    pub fn dsr1_mut(&self) -> &mut DSR1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(283usize) as *mut DSR1) }
    }
    #[doc = "0x128 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub fn dsr_bcr2(&self) -> &DSR_BCR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(296usize) as *const DSR_BCR2) }
    }
    #[doc = "0x128 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub fn dsr_bcr2_mut(&self) -> &mut DSR_BCR2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(296usize) as *mut DSR_BCR2) }
    }
    #[doc = "0x12b - DMA_DSR2 register."]
    #[inline(always)]
    pub fn dsr2(&self) -> &DSR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(299usize) as *const DSR2) }
    }
    #[doc = "0x12b - DMA_DSR2 register."]
    #[inline(always)]
    pub fn dsr2_mut(&self) -> &mut DSR2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(299usize) as *mut DSR2) }
    }
    #[doc = "0x138 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub fn dsr_bcr3(&self) -> &DSR_BCR3 {
        unsafe { &*(((self as *const Self) as *const u8).add(312usize) as *const DSR_BCR3) }
    }
    #[doc = "0x138 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub fn dsr_bcr3_mut(&self) -> &mut DSR_BCR3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(312usize) as *mut DSR_BCR3) }
    }
    #[doc = "0x13b - DMA_DSR3 register."]
    #[inline(always)]
    pub fn dsr3(&self) -> &DSR3 {
        unsafe { &*(((self as *const Self) as *const u8).add(315usize) as *const DSR3) }
    }
    #[doc = "0x13b - DMA_DSR3 register."]
    #[inline(always)]
    pub fn dsr3_mut(&self) -> &mut DSR3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(315usize) as *mut DSR3) }
    }
}
#[doc = "Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar0](sar0) module"]
pub type SAR0 = crate::Reg<u32, _SAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR0;
#[doc = "`read()` method returns [sar0::R](sar0::R) reader structure"]
impl crate::Readable for SAR0 {}
#[doc = "`write(|w| ..)` method takes [sar0::W](sar0::W) writer structure"]
impl crate::Writable for SAR0 {}
#[doc = "Source Address Register"]
pub mod sar0;
#[doc = "Destination Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dar0](dar0) module"]
pub type DAR0 = crate::Reg<u32, _DAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAR0;
#[doc = "`read()` method returns [dar0::R](dar0::R) reader structure"]
impl crate::Readable for DAR0 {}
#[doc = "`write(|w| ..)` method takes [dar0::W](dar0::W) writer structure"]
impl crate::Writable for DAR0 {}
#[doc = "Destination Address Register"]
pub mod dar0;
#[doc = "DMA Status Register / Byte Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr_bcr0](dsr_bcr0) module"]
pub type DSR_BCR0 = crate::Reg<u32, _DSR_BCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSR_BCR0;
#[doc = "`read()` method returns [dsr_bcr0::R](dsr_bcr0::R) reader structure"]
impl crate::Readable for DSR_BCR0 {}
#[doc = "`write(|w| ..)` method takes [dsr_bcr0::W](dsr_bcr0::W) writer structure"]
impl crate::Writable for DSR_BCR0 {}
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr0;
#[doc = "DMA_DSR0 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr0](dsr0) module"]
pub type DSR0 = crate::Reg<u8, _DSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSR0;
#[doc = "`read()` method returns [dsr0::R](dsr0::R) reader structure"]
impl crate::Readable for DSR0 {}
#[doc = "`write(|w| ..)` method takes [dsr0::W](dsr0::W) writer structure"]
impl crate::Writable for DSR0 {}
#[doc = "DMA_DSR0 register."]
pub mod dsr0;
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr0](dcr0) module"]
pub type DCR0 = crate::Reg<u32, _DCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR0;
#[doc = "`read()` method returns [dcr0::R](dcr0::R) reader structure"]
impl crate::Readable for DCR0 {}
#[doc = "`write(|w| ..)` method takes [dcr0::W](dcr0::W) writer structure"]
impl crate::Writable for DCR0 {}
#[doc = "DMA Control Register"]
pub mod dcr0;
#[doc = "Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar1](sar1) module"]
pub type SAR1 = crate::Reg<u32, _SAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR1;
#[doc = "`read()` method returns [sar1::R](sar1::R) reader structure"]
impl crate::Readable for SAR1 {}
#[doc = "`write(|w| ..)` method takes [sar1::W](sar1::W) writer structure"]
impl crate::Writable for SAR1 {}
#[doc = "Source Address Register"]
pub mod sar1;
#[doc = "Destination Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dar1](dar1) module"]
pub type DAR1 = crate::Reg<u32, _DAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAR1;
#[doc = "`read()` method returns [dar1::R](dar1::R) reader structure"]
impl crate::Readable for DAR1 {}
#[doc = "`write(|w| ..)` method takes [dar1::W](dar1::W) writer structure"]
impl crate::Writable for DAR1 {}
#[doc = "Destination Address Register"]
pub mod dar1;
#[doc = "DMA Status Register / Byte Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr_bcr1](dsr_bcr1) module"]
pub type DSR_BCR1 = crate::Reg<u32, _DSR_BCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSR_BCR1;
#[doc = "`read()` method returns [dsr_bcr1::R](dsr_bcr1::R) reader structure"]
impl crate::Readable for DSR_BCR1 {}
#[doc = "`write(|w| ..)` method takes [dsr_bcr1::W](dsr_bcr1::W) writer structure"]
impl crate::Writable for DSR_BCR1 {}
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr1;
#[doc = "DMA_DSR1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr1](dsr1) module"]
pub type DSR1 = crate::Reg<u8, _DSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSR1;
#[doc = "`read()` method returns [dsr1::R](dsr1::R) reader structure"]
impl crate::Readable for DSR1 {}
#[doc = "`write(|w| ..)` method takes [dsr1::W](dsr1::W) writer structure"]
impl crate::Writable for DSR1 {}
#[doc = "DMA_DSR1 register."]
pub mod dsr1;
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr1](dcr1) module"]
pub type DCR1 = crate::Reg<u32, _DCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR1;
#[doc = "`read()` method returns [dcr1::R](dcr1::R) reader structure"]
impl crate::Readable for DCR1 {}
#[doc = "`write(|w| ..)` method takes [dcr1::W](dcr1::W) writer structure"]
impl crate::Writable for DCR1 {}
#[doc = "DMA Control Register"]
pub mod dcr1;
#[doc = "Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar2](sar2) module"]
pub type SAR2 = crate::Reg<u32, _SAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR2;
#[doc = "`read()` method returns [sar2::R](sar2::R) reader structure"]
impl crate::Readable for SAR2 {}
#[doc = "`write(|w| ..)` method takes [sar2::W](sar2::W) writer structure"]
impl crate::Writable for SAR2 {}
#[doc = "Source Address Register"]
pub mod sar2;
#[doc = "Destination Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dar2](dar2) module"]
pub type DAR2 = crate::Reg<u32, _DAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAR2;
#[doc = "`read()` method returns [dar2::R](dar2::R) reader structure"]
impl crate::Readable for DAR2 {}
#[doc = "`write(|w| ..)` method takes [dar2::W](dar2::W) writer structure"]
impl crate::Writable for DAR2 {}
#[doc = "Destination Address Register"]
pub mod dar2;
#[doc = "DMA Status Register / Byte Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr_bcr2](dsr_bcr2) module"]
pub type DSR_BCR2 = crate::Reg<u32, _DSR_BCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSR_BCR2;
#[doc = "`read()` method returns [dsr_bcr2::R](dsr_bcr2::R) reader structure"]
impl crate::Readable for DSR_BCR2 {}
#[doc = "`write(|w| ..)` method takes [dsr_bcr2::W](dsr_bcr2::W) writer structure"]
impl crate::Writable for DSR_BCR2 {}
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr2;
#[doc = "DMA_DSR2 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr2](dsr2) module"]
pub type DSR2 = crate::Reg<u8, _DSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSR2;
#[doc = "`read()` method returns [dsr2::R](dsr2::R) reader structure"]
impl crate::Readable for DSR2 {}
#[doc = "`write(|w| ..)` method takes [dsr2::W](dsr2::W) writer structure"]
impl crate::Writable for DSR2 {}
#[doc = "DMA_DSR2 register."]
pub mod dsr2;
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr2](dcr2) module"]
pub type DCR2 = crate::Reg<u32, _DCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR2;
#[doc = "`read()` method returns [dcr2::R](dcr2::R) reader structure"]
impl crate::Readable for DCR2 {}
#[doc = "`write(|w| ..)` method takes [dcr2::W](dcr2::W) writer structure"]
impl crate::Writable for DCR2 {}
#[doc = "DMA Control Register"]
pub mod dcr2;
#[doc = "Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar3](sar3) module"]
pub type SAR3 = crate::Reg<u32, _SAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR3;
#[doc = "`read()` method returns [sar3::R](sar3::R) reader structure"]
impl crate::Readable for SAR3 {}
#[doc = "`write(|w| ..)` method takes [sar3::W](sar3::W) writer structure"]
impl crate::Writable for SAR3 {}
#[doc = "Source Address Register"]
pub mod sar3;
#[doc = "Destination Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dar3](dar3) module"]
pub type DAR3 = crate::Reg<u32, _DAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAR3;
#[doc = "`read()` method returns [dar3::R](dar3::R) reader structure"]
impl crate::Readable for DAR3 {}
#[doc = "`write(|w| ..)` method takes [dar3::W](dar3::W) writer structure"]
impl crate::Writable for DAR3 {}
#[doc = "Destination Address Register"]
pub mod dar3;
#[doc = "DMA Status Register / Byte Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr_bcr3](dsr_bcr3) module"]
pub type DSR_BCR3 = crate::Reg<u32, _DSR_BCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSR_BCR3;
#[doc = "`read()` method returns [dsr_bcr3::R](dsr_bcr3::R) reader structure"]
impl crate::Readable for DSR_BCR3 {}
#[doc = "`write(|w| ..)` method takes [dsr_bcr3::W](dsr_bcr3::W) writer structure"]
impl crate::Writable for DSR_BCR3 {}
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr3;
#[doc = "DMA_DSR3 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr3](dsr3) module"]
pub type DSR3 = crate::Reg<u8, _DSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSR3;
#[doc = "`read()` method returns [dsr3::R](dsr3::R) reader structure"]
impl crate::Readable for DSR3 {}
#[doc = "`write(|w| ..)` method takes [dsr3::W](dsr3::W) writer structure"]
impl crate::Writable for DSR3 {}
#[doc = "DMA_DSR3 register."]
pub mod dsr3;
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr3](dcr3) module"]
pub type DCR3 = crate::Reg<u32, _DCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR3;
#[doc = "`read()` method returns [dcr3::R](dcr3::R) reader structure"]
impl crate::Readable for DCR3 {}
#[doc = "`write(|w| ..)` method takes [dcr3::W](dcr3::W) writer structure"]
impl crate::Writable for DCR3 {}
#[doc = "DMA Control Register"]
pub mod dcr3;
