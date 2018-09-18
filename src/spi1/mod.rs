#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI control register 1"]
    pub c1: C1,
    #[doc = "0x01 - SPI control register 2"]
    pub c2: C2,
    #[doc = "0x02 - SPI baud rate register"]
    pub br: BR,
    #[doc = "0x03 - SPI status register"]
    pub s: S,
    _reserved0: [u8; 1usize],
    #[doc = "0x05 - SPI data register"]
    pub d: D,
    _reserved1: [u8; 1usize],
    #[doc = "0x07 - SPI match register"]
    pub m: M,
}
#[doc = "SPI control register 1"]
pub struct C1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPI control register 1"]
pub mod c1;
#[doc = "SPI control register 2"]
pub struct C2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPI control register 2"]
pub mod c2;
#[doc = "SPI baud rate register"]
pub struct BR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPI baud rate register"]
pub mod br;
#[doc = "SPI status register"]
pub struct S {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPI status register"]
pub mod s;
#[doc = "SPI data register"]
pub struct D {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPI data register"]
pub mod d;
#[doc = "SPI match register"]
pub struct M {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPI match register"]
pub mod m;
