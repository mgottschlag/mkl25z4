#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control"]
    pub sc: SC,
    #[doc = "0x04 - Counter"]
    pub cnt: CNT,
    #[doc = "0x08 - Modulo"]
    pub mod_: MOD,
    #[doc = "0x0c - Channel (n) Status and Control"]
    pub c0sc: CSC,
    #[doc = "0x10 - Channel (n) Value"]
    pub c0v: CV,
    #[doc = "0x14 - Channel (n) Status and Control"]
    pub c1sc: CSC,
    #[doc = "0x18 - Channel (n) Value"]
    pub c1v: CV,
    #[doc = "0x1c - Channel (n) Status and Control"]
    pub c2sc: CSC,
    #[doc = "0x20 - Channel (n) Value"]
    pub c2v: CV,
    #[doc = "0x24 - Channel (n) Status and Control"]
    pub c3sc: CSC,
    #[doc = "0x28 - Channel (n) Value"]
    pub c3v: CV,
    #[doc = "0x2c - Channel (n) Status and Control"]
    pub c4sc: CSC,
    #[doc = "0x30 - Channel (n) Value"]
    pub c4v: CV,
    #[doc = "0x34 - Channel (n) Status and Control"]
    pub c5sc: CSC,
    #[doc = "0x38 - Channel (n) Value"]
    pub c5v: CV,
    _reserved0: [u8; 20usize],
    #[doc = "0x50 - Capture and Compare Status"]
    pub status: STATUS,
    _reserved1: [u8; 48usize],
    #[doc = "0x84 - Configuration"]
    pub conf: CONF,
}
#[doc = "Status and Control"]
pub struct SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status and Control"]
pub mod sc;
#[doc = "Counter"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter"]
pub mod cnt;
#[doc = "Modulo"]
pub struct MOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modulo"]
pub mod mod_;
#[doc = "Channel (n) Status and Control"]
pub struct CSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Status and Control"]
pub mod csc;
#[doc = "Channel (n) Value"]
pub struct CV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Value"]
pub mod cv;
#[doc = "Capture and Compare Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture and Compare Status"]
pub mod status;
#[doc = "Configuration"]
pub struct CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration"]
pub mod conf;
