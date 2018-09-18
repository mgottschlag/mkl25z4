# ! [ doc = "Peripheral access API for MKL25Z4 microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api" ] # ! [ deny ( missing_docs ) ] # ! [ deny ( warnings ) ] # ! [ allow ( non_camel_case_types ) ] # ! [ no_std ]extern crate cortex_m ;
#[cfg(feature = "rt")]
extern crate cortex_m_rt ;
extern crate bare_metal ;
extern crate vcell ;
use core::ops::Deref;
use core::marker::PhantomData;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA0();
    fn DMA1();
    fn DMA2();
    fn DMA3();
    fn FTFA();
    fn LVD_LVW();
    fn LLWU();
    fn I2C0();
    fn I2C1();
    fn SPI0();
    fn SPI1();
    fn UART0();
    fn UART1();
    fn UART2();
    fn ADC0();
    fn CMP0();
    fn TPM0();
    fn TPM1();
    fn TPM2();
    fn RTC();
    fn RTC_SECONDS();
    fn PIT();
    fn USB0();
    fn DAC0();
    fn TSI0();
    fn MCG();
    fn LPTMR0();
    fn PORTA();
    fn PORTD();
}
#[doc(hidden)]pub union Vector { _handler : unsafe extern "C" fn ( ) , _reserved : u32 }#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: DMA0 },
    Vector { _handler: DMA1 },
    Vector { _handler: DMA2 },
    Vector { _handler: DMA3 },
    Vector { _reserved: 0 },
    Vector { _handler: FTFA },
    Vector { _handler: LVD_LVW },
    Vector { _handler: LLWU },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: SPI0 },
    Vector { _handler: SPI1 },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: UART2 },
    Vector { _handler: ADC0 },
    Vector { _handler: CMP0 },
    Vector { _handler: TPM0 },
    Vector { _handler: TPM1 },
    Vector { _handler: TPM2 },
    Vector { _handler: RTC },
    Vector { _handler: RTC_SECONDS },
    Vector { _handler: PIT },
    Vector { _reserved: 0 },
    Vector { _handler: USB0 },
    Vector { _handler: DAC0 },
    Vector { _handler: TSI0 },
    Vector { _handler: MCG },
    Vector { _handler: LPTMR0 },
    Vector { _reserved: 0 },
    Vector { _handler: PORTA },
    Vector { _handler: PORTD },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ Name : ident , $ handler : path , state : $ State : ty = $ initial_state : expr ) => { # [ allow ( unsafe_code ) ] # [ deny ( private_no_mangle_fns ) ] # [ no_mangle ] pub unsafe extern "C" fn $ Name ( ) { static mut STATE : $ State = $ initial_state ; let _ = $ crate :: Interrupt :: $ Name ; let f : fn ( & mut $ State ) = $ handler ; f ( & mut STATE ) } } ; ( $ Name : ident , $ handler : path ) => { # [ allow ( unsafe_code ) ] # [ deny ( private_no_mangle_fns ) ] # [ no_mangle ] pub unsafe extern "C" fn $ Name ( ) { let _ = $ crate :: Interrupt :: $ Name ; let f : fn ( ) = $ handler ; f ( ) } } ; }
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - DMA0"]
    DMA0,
    #[doc = "1 - DMA1"]
    DMA1,
    #[doc = "2 - DMA2"]
    DMA2,
    #[doc = "3 - DMA3"]
    DMA3,
    #[doc = "5 - FTFA"]
    FTFA,
    #[doc = "6 - LVD_LVW"]
    LVD_LVW,
    #[doc = "7 - LLWU"]
    LLWU,
    #[doc = "8 - I2C0"]
    I2C0,
    #[doc = "9 - I2C1"]
    I2C1,
    #[doc = "10 - SPI0"]
    SPI0,
    #[doc = "11 - SPI1"]
    SPI1,
    #[doc = "12 - UART0"]
    UART0,
    #[doc = "13 - UART1"]
    UART1,
    #[doc = "14 - UART2"]
    UART2,
    #[doc = "15 - ADC0"]
    ADC0,
    #[doc = "16 - CMP0"]
    CMP0,
    #[doc = "17 - TPM0"]
    TPM0,
    #[doc = "18 - TPM1"]
    TPM1,
    #[doc = "19 - TPM2"]
    TPM2,
    #[doc = "20 - RTC"]
    RTC,
    #[doc = "21 - RTC_Seconds"]
    RTC_SECONDS,
    #[doc = "22 - PIT"]
    PIT,
    #[doc = "24 - USB0"]
    USB0,
    #[doc = "25 - DAC0"]
    DAC0,
    #[doc = "26 - TSI0"]
    TSI0,
    #[doc = "27 - MCG"]
    MCG,
    #[doc = "28 - LPTMR0"]
    LPTMR0,
    #[doc = "30 - PORTA"]
    PORTA,
    #[doc = "31 - PORTD"]
    PORTD,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::DMA0 => 0,
            Interrupt::DMA1 => 1,
            Interrupt::DMA2 => 2,
            Interrupt::DMA3 => 3,
            Interrupt::FTFA => 5,
            Interrupt::LVD_LVW => 6,
            Interrupt::LLWU => 7,
            Interrupt::I2C0 => 8,
            Interrupt::I2C1 => 9,
            Interrupt::SPI0 => 10,
            Interrupt::SPI1 => 11,
            Interrupt::UART0 => 12,
            Interrupt::UART1 => 13,
            Interrupt::UART2 => 14,
            Interrupt::ADC0 => 15,
            Interrupt::CMP0 => 16,
            Interrupt::TPM0 => 17,
            Interrupt::TPM1 => 18,
            Interrupt::TPM2 => 19,
            Interrupt::RTC => 20,
            Interrupt::RTC_SECONDS => 21,
            Interrupt::PIT => 22,
            Interrupt::USB0 => 24,
            Interrupt::DAC0 => 25,
            Interrupt::TSI0 => 26,
            Interrupt::MCG => 27,
            Interrupt::LPTMR0 => 28,
            Interrupt::PORTA => 30,
            Interrupt::PORTD => 31,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Flash configuration field"]
pub struct FTFA_FLASHCONFIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFA_FLASHCONFIG {}
impl FTFA_FLASHCONFIG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftfa_flash_config::RegisterBlock {
        1024 as *const _
    }
}
impl Deref for FTFA_FLASHCONFIG {
    type Target = ftfa_flash_config::RegisterBlock;
    fn deref(&self) -> &ftfa_flash_config::RegisterBlock {
        unsafe { &*FTFA_FLASHCONFIG::ptr() }
    }
}
#[doc = "Flash configuration field"]
pub mod ftfa_flash_config;
#[doc = "DMA Controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &dma::RegisterBlock {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA Controller"]
pub mod dma;
#[doc = "Flash Memory Interface"]
pub struct FTFA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFA {}
impl FTFA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftfa::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for FTFA {
    type Target = ftfa::RegisterBlock;
    fn deref(&self) -> &ftfa::RegisterBlock {
        unsafe { &*FTFA::ptr() }
    }
}
#[doc = "Flash Memory Interface"]
pub mod ftfa;
#[doc = "DMA channel multiplexor"]
pub struct DMAMUX0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX0 {}
impl DMAMUX0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dmamux0::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for DMAMUX0 {
    type Target = dmamux0::RegisterBlock;
    fn deref(&self) -> &dmamux0::RegisterBlock {
        unsafe { &*DMAMUX0::ptr() }
    }
}
#[doc = "DMA channel multiplexor"]
pub mod dmamux0;
#[doc = "Periodic Interrupt Timer"]
pub struct PIT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIT {}
impl PIT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pit::RegisterBlock {
        1073967104 as *const _
    }
}
impl Deref for PIT {
    type Target = pit::RegisterBlock;
    fn deref(&self) -> &pit::RegisterBlock {
        unsafe { &*PIT::ptr() }
    }
}
#[doc = "Periodic Interrupt Timer"]
pub mod pit;
#[doc = "Timer/PWM Module"]
pub struct TPM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM0 {}
impl TPM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tpm0::RegisterBlock {
        1073971200 as *const _
    }
}
impl Deref for TPM0 {
    type Target = tpm0::RegisterBlock;
    fn deref(&self) -> &tpm0::RegisterBlock {
        unsafe { &*TPM0::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm0;
#[doc = "Timer/PWM Module"]
pub struct TPM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM1 {}
impl TPM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tpm1::RegisterBlock {
        1073975296 as *const _
    }
}
impl Deref for TPM1 {
    type Target = tpm1::RegisterBlock;
    fn deref(&self) -> &tpm1::RegisterBlock {
        unsafe { &*TPM1::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm1;
#[doc = "Timer/PWM Module"]
pub struct TPM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM2 {}
impl TPM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tpm2::RegisterBlock {
        1073979392 as *const _
    }
}
impl Deref for TPM2 {
    type Target = tpm2::RegisterBlock;
    fn deref(&self) -> &tpm2::RegisterBlock {
        unsafe { &*TPM2::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm2;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc0::RegisterBlock {
        1073983488 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &adc0::RegisterBlock {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc0;
#[doc = "Secure Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073991680 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Secure Real Time Clock"]
pub mod rtc;
#[doc = "12-Bit Digital-to-Analog Converter"]
pub struct DAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC0 {}
impl DAC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac0::RegisterBlock {
        1073999872 as *const _
    }
}
impl Deref for DAC0 {
    type Target = dac0::RegisterBlock;
    fn deref(&self) -> &dac0::RegisterBlock {
        unsafe { &*DAC0::ptr() }
    }
}
#[doc = "12-Bit Digital-to-Analog Converter"]
pub mod dac0;
#[doc = "Low Power Timer"]
pub struct LPTMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTMR0 {}
impl LPTMR0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lptmr0::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for LPTMR0 {
    type Target = lptmr0::RegisterBlock;
    fn deref(&self) -> &lptmr0::RegisterBlock {
        unsafe { &*LPTMR0::ptr() }
    }
}
#[doc = "Low Power Timer"]
pub mod lptmr0;
#[doc = "Touch sense input"]
pub struct TSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSI0 {}
impl TSI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tsi0::RegisterBlock {
        1074024448 as *const _
    }
}
impl Deref for TSI0 {
    type Target = tsi0::RegisterBlock;
    fn deref(&self) -> &tsi0::RegisterBlock {
        unsafe { &*TSI0::ptr() }
    }
}
#[doc = "Touch sense input"]
pub mod tsi0;
#[doc = "System Integration Module"]
pub struct SIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIM {}
impl SIM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sim::RegisterBlock {
        1074032640 as *const _
    }
}
impl Deref for SIM {
    type Target = sim::RegisterBlock;
    fn deref(&self) -> &sim::RegisterBlock {
        unsafe { &*SIM::ptr() }
    }
}
#[doc = "System Integration Module"]
pub mod sim;
#[doc = "Pin Control and Interrupts"]
pub struct PORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTA {}
impl PORTA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const porta::RegisterBlock {
        1074040832 as *const _
    }
}
impl Deref for PORTA {
    type Target = porta::RegisterBlock;
    fn deref(&self) -> &porta::RegisterBlock {
        unsafe { &*PORTA::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porta;
#[doc = "Pin Control and Interrupts"]
pub struct PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTB {}
impl PORTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portb::RegisterBlock {
        1074044928 as *const _
    }
}
impl Deref for PORTB {
    type Target = portb::RegisterBlock;
    fn deref(&self) -> &portb::RegisterBlock {
        unsafe { &*PORTB::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portb;
#[doc = "Pin Control and Interrupts"]
pub struct PORTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTC {}
impl PORTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portc::RegisterBlock {
        1074049024 as *const _
    }
}
impl Deref for PORTC {
    type Target = portc::RegisterBlock;
    fn deref(&self) -> &portc::RegisterBlock {
        unsafe { &*PORTC::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portc;
#[doc = "Pin Control and Interrupts"]
pub struct PORTD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTD {}
impl PORTD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portd::RegisterBlock {
        1074053120 as *const _
    }
}
impl Deref for PORTD {
    type Target = portd::RegisterBlock;
    fn deref(&self) -> &portd::RegisterBlock {
        unsafe { &*PORTD::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portd;
#[doc = "Pin Control and Interrupts"]
pub struct PORTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTE {}
impl PORTE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const porte::RegisterBlock {
        1074057216 as *const _
    }
}
impl Deref for PORTE {
    type Target = porte::RegisterBlock;
    fn deref(&self) -> &porte::RegisterBlock {
        unsafe { &*PORTE::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porte;
#[doc = "Multipurpose Clock Generator module"]
pub struct MCG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCG {}
impl MCG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcg::RegisterBlock {
        1074151424 as *const _
    }
}
impl Deref for MCG {
    type Target = mcg::RegisterBlock;
    fn deref(&self) -> &mcg::RegisterBlock {
        unsafe { &*MCG::ptr() }
    }
}
#[doc = "Multipurpose Clock Generator module"]
pub mod mcg;
#[doc = "Oscillator"]
pub struct OSC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSC0 {}
impl OSC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const osc0::RegisterBlock {
        1074155520 as *const _
    }
}
impl Deref for OSC0 {
    type Target = osc0::RegisterBlock;
    fn deref(&self) -> &osc0::RegisterBlock {
        unsafe { &*OSC0::ptr() }
    }
}
#[doc = "Oscillator"]
pub mod osc0;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074159616 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c0;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1074163712 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c1;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1074176000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod uart0;
#[doc = "Universal Asynchronous Receiver/Transmitter (UART)"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        1074180096 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter (UART)"]
pub mod uart1;
#[doc = "Universal Asynchronous Receiver/Transmitter (UART)"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart2::RegisterBlock {
        1074184192 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart2::RegisterBlock;
    fn deref(&self) -> &uart2::RegisterBlock {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter (UART)"]
pub mod uart2;
#[doc = "Universal Serial Bus, OTG Capable Controller"]
pub struct USB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0::RegisterBlock {
        1074208768 as *const _
    }
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    fn deref(&self) -> &usb0::RegisterBlock {
        unsafe { &*USB0::ptr() }
    }
}
#[doc = "Universal Serial Bus, OTG Capable Controller"]
pub mod usb0;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP0 {}
impl CMP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmp0::RegisterBlock {
        1074212864 as *const _
    }
}
impl Deref for CMP0 {
    type Target = cmp0::RegisterBlock;
    fn deref(&self) -> &cmp0::RegisterBlock {
        unsafe { &*CMP0::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp0;
#[doc = "Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1074225152 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi0;
#[doc = "Serial Peripheral Interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1074229248 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi1;
#[doc = "Low leakage wakeup unit"]
pub struct LLWU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LLWU {}
impl LLWU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const llwu::RegisterBlock {
        1074249728 as *const _
    }
}
impl Deref for LLWU {
    type Target = llwu::RegisterBlock;
    fn deref(&self) -> &llwu::RegisterBlock {
        unsafe { &*LLWU::ptr() }
    }
}
#[doc = "Low leakage wakeup unit"]
pub mod llwu;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmc::RegisterBlock {
        1074253824 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &pmc::RegisterBlock {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "System Mode Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const smc::RegisterBlock {
        1074257920 as *const _
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    fn deref(&self) -> &smc::RegisterBlock {
        unsafe { &*SMC::ptr() }
    }
}
#[doc = "System Mode Controller"]
pub mod smc;
#[doc = "Reset Control Module"]
pub struct RCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCM {}
impl RCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcm::RegisterBlock {
        1074262016 as *const _
    }
}
impl Deref for RCM {
    type Target = rcm::RegisterBlock;
    fn deref(&self) -> &rcm::RegisterBlock {
        unsafe { &*RCM::ptr() }
    }
}
#[doc = "Reset Control Module"]
pub mod rcm;
#[doc = "General Purpose Input/Output"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1074786304 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioa;
#[doc = "General Purpose Input/Output"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1074786368 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiob;
#[doc = "General Purpose Input/Output"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1074786432 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioc;
#[doc = "General Purpose Input/Output"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiod::RegisterBlock {
        1074786496 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpiod::RegisterBlock;
    fn deref(&self) -> &gpiod::RegisterBlock {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiod;
#[doc = "General Purpose Input/Output"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioe::RegisterBlock {
        1074786560 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioe::RegisterBlock;
    fn deref(&self) -> &gpioe::RegisterBlock {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioe;
#[doc = "Micro Trace Buffer"]
pub struct MTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB {}
impl MTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mtb::RegisterBlock {
        4026531840 as *const _
    }
}
impl Deref for MTB {
    type Target = mtb::RegisterBlock;
    fn deref(&self) -> &mtb::RegisterBlock {
        unsafe { &*MTB::ptr() }
    }
}
#[doc = "Micro Trace Buffer"]
pub mod mtb;
#[doc = "MTB data watchpoint and trace"]
pub struct MTBDWT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTBDWT {}
impl MTBDWT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mtbdwt::RegisterBlock {
        4026535936 as *const _
    }
}
impl Deref for MTBDWT {
    type Target = mtbdwt::RegisterBlock;
    fn deref(&self) -> &mtbdwt::RegisterBlock {
        unsafe { &*MTBDWT::ptr() }
    }
}
#[doc = "MTB data watchpoint and trace"]
pub mod mtbdwt;
#[doc = "System ROM"]
pub struct ROM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ROM {}
impl ROM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rom::RegisterBlock {
        4026540032 as *const _
    }
}
impl Deref for ROM {
    type Target = rom::RegisterBlock;
    fn deref(&self) -> &rom::RegisterBlock {
        unsafe { &*ROM::ptr() }
    }
}
#[doc = "System ROM"]
pub mod rom;
#[doc = "Core Platform Miscellaneous Control Module"]
pub struct MCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCM {}
impl MCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcm::RegisterBlock {
        4026544128 as *const _
    }
}
impl Deref for MCM {
    type Target = mcm::RegisterBlock;
    fn deref(&self) -> &mcm::RegisterBlock {
        unsafe { &*MCM::ptr() }
    }
}
#[doc = "Core Platform Miscellaneous Control Module"]
pub mod mcm;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOA {}
impl FGPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpioa::RegisterBlock {
        4161794048 as *const _
    }
}
impl Deref for FGPIOA {
    type Target = fgpioa::RegisterBlock;
    fn deref(&self) -> &fgpioa::RegisterBlock {
        unsafe { &*FGPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioa;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOB {}
impl FGPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpiob::RegisterBlock {
        4161794112 as *const _
    }
}
impl Deref for FGPIOB {
    type Target = fgpiob::RegisterBlock;
    fn deref(&self) -> &fgpiob::RegisterBlock {
        unsafe { &*FGPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiob;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOC {}
impl FGPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpioc::RegisterBlock {
        4161794176 as *const _
    }
}
impl Deref for FGPIOC {
    type Target = fgpioc::RegisterBlock;
    fn deref(&self) -> &fgpioc::RegisterBlock {
        unsafe { &*FGPIOC::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioc;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOD {}
impl FGPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpiod::RegisterBlock {
        4161794240 as *const _
    }
}
impl Deref for FGPIOD {
    type Target = fgpiod::RegisterBlock;
    fn deref(&self) -> &fgpiod::RegisterBlock {
        unsafe { &*FGPIOD::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiod;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOE {}
impl FGPIOE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpioe::RegisterBlock {
        4161794304 as *const _
    }
}
impl Deref for FGPIOE {
    type Target = fgpioe::RegisterBlock;
    fn deref(&self) -> &fgpioe::RegisterBlock {
        unsafe { &*FGPIOE::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioe;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FTFA_FLASHCONFIG"]
    pub FTFA_FLASHCONFIG: FTFA_FLASHCONFIG,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "FTFA"]
    pub FTFA: FTFA,
    #[doc = "DMAMUX0"]
    pub DMAMUX0: DMAMUX0,
    #[doc = "PIT"]
    pub PIT: PIT,
    #[doc = "TPM0"]
    pub TPM0: TPM0,
    #[doc = "TPM1"]
    pub TPM1: TPM1,
    #[doc = "TPM2"]
    pub TPM2: TPM2,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "DAC0"]
    pub DAC0: DAC0,
    #[doc = "LPTMR0"]
    pub LPTMR0: LPTMR0,
    #[doc = "TSI0"]
    pub TSI0: TSI0,
    #[doc = "SIM"]
    pub SIM: SIM,
    #[doc = "PORTA"]
    pub PORTA: PORTA,
    #[doc = "PORTB"]
    pub PORTB: PORTB,
    #[doc = "PORTC"]
    pub PORTC: PORTC,
    #[doc = "PORTD"]
    pub PORTD: PORTD,
    #[doc = "PORTE"]
    pub PORTE: PORTE,
    #[doc = "MCG"]
    pub MCG: MCG,
    #[doc = "OSC0"]
    pub OSC0: OSC0,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "USB0"]
    pub USB0: USB0,
    #[doc = "CMP0"]
    pub CMP0: CMP0,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "LLWU"]
    pub LLWU: LLWU,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "SMC"]
    pub SMC: SMC,
    #[doc = "RCM"]
    pub RCM: RCM,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "MTB"]
    pub MTB: MTB,
    #[doc = "MTBDWT"]
    pub MTBDWT: MTBDWT,
    #[doc = "ROM"]
    pub ROM: ROM,
    #[doc = "MCM"]
    pub MCM: MCM,
    #[doc = "FGPIOA"]
    pub FGPIOA: FGPIOA,
    #[doc = "FGPIOB"]
    pub FGPIOB: FGPIOB,
    #[doc = "FGPIOC"]
    pub FGPIOC: FGPIOC,
    #[doc = "FGPIOD"]
    pub FGPIOD: FGPIOD,
    #[doc = "FGPIOE"]
    pub FGPIOE: FGPIOE,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| if unsafe { DEVICE_PERIPHERALS } {
            None
        } else {
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FTFA_FLASHCONFIG: FTFA_FLASHCONFIG { _marker: PhantomData },
            DMA: DMA { _marker: PhantomData },
            FTFA: FTFA { _marker: PhantomData },
            DMAMUX0: DMAMUX0 { _marker: PhantomData },
            PIT: PIT { _marker: PhantomData },
            TPM0: TPM0 { _marker: PhantomData },
            TPM1: TPM1 { _marker: PhantomData },
            TPM2: TPM2 { _marker: PhantomData },
            ADC0: ADC0 { _marker: PhantomData },
            RTC: RTC { _marker: PhantomData },
            DAC0: DAC0 { _marker: PhantomData },
            LPTMR0: LPTMR0 { _marker: PhantomData },
            TSI0: TSI0 { _marker: PhantomData },
            SIM: SIM { _marker: PhantomData },
            PORTA: PORTA { _marker: PhantomData },
            PORTB: PORTB { _marker: PhantomData },
            PORTC: PORTC { _marker: PhantomData },
            PORTD: PORTD { _marker: PhantomData },
            PORTE: PORTE { _marker: PhantomData },
            MCG: MCG { _marker: PhantomData },
            OSC0: OSC0 { _marker: PhantomData },
            I2C0: I2C0 { _marker: PhantomData },
            I2C1: I2C1 { _marker: PhantomData },
            UART0: UART0 { _marker: PhantomData },
            UART1: UART1 { _marker: PhantomData },
            UART2: UART2 { _marker: PhantomData },
            USB0: USB0 { _marker: PhantomData },
            CMP0: CMP0 { _marker: PhantomData },
            SPI0: SPI0 { _marker: PhantomData },
            SPI1: SPI1 { _marker: PhantomData },
            LLWU: LLWU { _marker: PhantomData },
            PMC: PMC { _marker: PhantomData },
            SMC: SMC { _marker: PhantomData },
            RCM: RCM { _marker: PhantomData },
            GPIOA: GPIOA { _marker: PhantomData },
            GPIOB: GPIOB { _marker: PhantomData },
            GPIOC: GPIOC { _marker: PhantomData },
            GPIOD: GPIOD { _marker: PhantomData },
            GPIOE: GPIOE { _marker: PhantomData },
            MTB: MTB { _marker: PhantomData },
            MTBDWT: MTBDWT { _marker: PhantomData },
            ROM: ROM { _marker: PhantomData },
            MCM: MCM { _marker: PhantomData },
            FGPIOA: FGPIOA { _marker: PhantomData },
            FGPIOB: FGPIOB { _marker: PhantomData },
            FGPIOC: FGPIOC { _marker: PhantomData },
            FGPIOD: FGPIOD { _marker: PhantomData },
            FGPIOE: FGPIOE { _marker: PhantomData },
        }
    }
}
