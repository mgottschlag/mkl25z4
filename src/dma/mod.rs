#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - Source Address Register"]
    pub sar0: SAR0,
    #[doc = "0x104 - Destination Address Register"]
    pub dar0: DAR0,
    #[doc = "0x108 - DMA Status Register / Byte Count Register"]
    pub dsr_bcr0: DSR_BCR0,
    #[doc = "0x10c - DMA Control Register"]
    pub dcr0: DCR0,
    #[doc = "0x110 - Source Address Register"]
    pub sar1: SAR1,
    #[doc = "0x114 - Destination Address Register"]
    pub dar1: DAR1,
    #[doc = "0x118 - DMA Status Register / Byte Count Register"]
    pub dsr_bcr1: DSR_BCR1,
    #[doc = "0x11c - DMA Control Register"]
    pub dcr1: DCR1,
    #[doc = "0x120 - Source Address Register"]
    pub sar2: SAR2,
    #[doc = "0x124 - Destination Address Register"]
    pub dar2: DAR2,
    #[doc = "0x128 - DMA Status Register / Byte Count Register"]
    pub dsr_bcr2: DSR_BCR2,
    #[doc = "0x12c - DMA Control Register"]
    pub dcr2: DCR2,
    #[doc = "0x130 - Source Address Register"]
    pub sar3: SAR3,
    #[doc = "0x134 - Destination Address Register"]
    pub dar3: DAR3,
    #[doc = "0x138 - DMA Status Register / Byte Count Register"]
    pub dsr_bcr3: DSR_BCR3,
    #[doc = "0x13c - DMA Control Register"]
    pub dcr3: DCR3,
}
#[doc = "Source Address Register"]
pub struct SAR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Address Register"]
pub mod sar0;
#[doc = "Destination Address Register"]
pub struct DAR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination Address Register"]
pub mod dar0;
#[doc = "DMA Status Register / Byte Count Register"]
pub struct DSR_BCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr0;
#[doc = "DMA_DSR0 register."]
pub struct DSR0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DMA_DSR0 register."]
pub mod dsr0;
#[doc = "DMA Control Register"]
pub struct DCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Control Register"]
pub mod dcr0;
#[doc = "Source Address Register"]
pub struct SAR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Address Register"]
pub mod sar1;
#[doc = "Destination Address Register"]
pub struct DAR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination Address Register"]
pub mod dar1;
#[doc = "DMA Status Register / Byte Count Register"]
pub struct DSR_BCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr1;
#[doc = "DMA_DSR1 register."]
pub struct DSR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DMA_DSR1 register."]
pub mod dsr1;
#[doc = "DMA Control Register"]
pub struct DCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Control Register"]
pub mod dcr1;
#[doc = "Source Address Register"]
pub struct SAR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Address Register"]
pub mod sar2;
#[doc = "Destination Address Register"]
pub struct DAR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination Address Register"]
pub mod dar2;
#[doc = "DMA Status Register / Byte Count Register"]
pub struct DSR_BCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr2;
#[doc = "DMA_DSR2 register."]
pub struct DSR2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DMA_DSR2 register."]
pub mod dsr2;
#[doc = "DMA Control Register"]
pub struct DCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Control Register"]
pub mod dcr2;
#[doc = "Source Address Register"]
pub struct SAR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Address Register"]
pub mod sar3;
#[doc = "Destination Address Register"]
pub struct DAR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination Address Register"]
pub mod dar3;
#[doc = "DMA Status Register / Byte Count Register"]
pub struct DSR_BCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr3;
#[doc = "DMA_DSR3 register."]
pub struct DSR3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DMA_DSR3 register."]
pub mod dsr3;
#[doc = "DMA Control Register"]
pub struct DCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Control Register"]
pub mod dcr3;
