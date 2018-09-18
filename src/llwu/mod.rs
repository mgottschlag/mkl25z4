#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LLWU Pin Enable 1 register"]
    pub pe1: PE1,
    #[doc = "0x01 - LLWU Pin Enable 2 register"]
    pub pe2: PE2,
    #[doc = "0x02 - LLWU Pin Enable 3 register"]
    pub pe3: PE3,
    #[doc = "0x03 - LLWU Pin Enable 4 register"]
    pub pe4: PE4,
    #[doc = "0x04 - LLWU Module Enable register"]
    pub me: ME,
    #[doc = "0x05 - LLWU Flag 1 register"]
    pub f1: F1,
    #[doc = "0x06 - LLWU Flag 2 register"]
    pub f2: F2,
    #[doc = "0x07 - LLWU Flag 3 register"]
    pub f3: F3,
    #[doc = "0x08 - LLWU Pin Filter 1 register"]
    pub filt1: FILT1,
    #[doc = "0x09 - LLWU Pin Filter 2 register"]
    pub filt2: FILT2,
}
#[doc = "LLWU Pin Enable 1 register"]
pub struct PE1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 1 register"]
pub mod pe1;
#[doc = "LLWU Pin Enable 2 register"]
pub struct PE2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 2 register"]
pub mod pe2;
#[doc = "LLWU Pin Enable 3 register"]
pub struct PE3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 3 register"]
pub mod pe3;
#[doc = "LLWU Pin Enable 4 register"]
pub struct PE4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 4 register"]
pub mod pe4;
#[doc = "LLWU Module Enable register"]
pub struct ME {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Module Enable register"]
pub mod me;
#[doc = "LLWU Flag 1 register"]
pub struct F1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Flag 1 register"]
pub mod f1;
#[doc = "LLWU Flag 2 register"]
pub struct F2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Flag 2 register"]
pub mod f2;
#[doc = "LLWU Flag 3 register"]
pub struct F3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Flag 3 register"]
pub mod f3;
#[doc = "LLWU Pin Filter 1 register"]
pub struct FILT1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Filter 1 register"]
pub mod filt1;
#[doc = "LLWU Pin Filter 2 register"]
pub struct FILT2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Filter 2 register"]
pub mod filt2;
