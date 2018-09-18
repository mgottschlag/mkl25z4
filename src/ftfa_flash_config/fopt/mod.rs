#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FOPT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `LPBOOT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBOOT0R {
    #[doc = "Core and system clock divider (OUTDIV1) is 0x7 (divide by 8) when LPBOOT1=0 or 0x1 (divide by 2) when LPBOOT1=1."]
    _00,
    #[doc = "Core and system clock divider (OUTDIV1) is 0x3 (divide by 4) when LPBOOT1=0 or 0x0 (divide by 1) when LPBOOT1=1."]
    _01,
}
impl LPBOOT0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LPBOOT0R::_00 => false,
            LPBOOT0R::_01 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPBOOT0R {
        match value {
            false => LPBOOT0R::_00,
            true => LPBOOT0R::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPBOOT0R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPBOOT0R::_01
    }
}
#[doc = "Possible values of the field `NMI_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMI_DISR {
    #[doc = "NMI interrupts are always blocked"]
    _00,
    #[doc = "NMI_b pin/interrupts reset default to enabled"]
    _01,
}
impl NMI_DISR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            NMI_DISR::_00 => false,
            NMI_DISR::_01 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NMI_DISR {
        match value {
            false => NMI_DISR::_00,
            true => NMI_DISR::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == NMI_DISR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == NMI_DISR::_01
    }
}
#[doc = "Possible values of the field `RESET_PIN_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_PIN_CFGR {
    #[doc = "RESET pin is disabled following a POR and cannot be enabled as reset function"]
    _00,
    #[doc = "RESET_b pin is dedicated"]
    _01,
}
impl RESET_PIN_CFGR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RESET_PIN_CFGR::_00 => false,
            RESET_PIN_CFGR::_01 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESET_PIN_CFGR {
        match value {
            false => RESET_PIN_CFGR::_00,
            true => RESET_PIN_CFGR::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RESET_PIN_CFGR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RESET_PIN_CFGR::_01
    }
}
#[doc = "Possible values of the field `LPBOOT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBOOT1R {
    #[doc = "Core and system clock divider (OUTDIV1) is 0x7 (divide by 8) when LPBOOT0=0 or 0x3 (divide by 4) when LPBOOT0=1."]
    _00,
    #[doc = "Core and system clock divider (OUTDIV1) is 0x1 (divide by 2) when LPBOOT0=0 or 0x0 (divide by 1) when LPBOOT0=1."]
    _01,
}
impl LPBOOT1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LPBOOT1R::_00 => false,
            LPBOOT1R::_01 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPBOOT1R {
        match value {
            false => LPBOOT1R::_00,
            true => LPBOOT1R::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPBOOT1R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPBOOT1R::_01
    }
}
#[doc = "Possible values of the field `FAST_INIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_INITR {
    #[doc = "Slower initialization"]
    _00,
    #[doc = "Fast Initialization"]
    _01,
}
impl FAST_INITR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FAST_INITR::_00 => false,
            FAST_INITR::_01 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAST_INITR {
        match value {
            false => FAST_INITR::_00,
            true => FAST_INITR::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FAST_INITR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FAST_INITR::_01
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn lpboot0(&self) -> LPBOOT0R {
        LPBOOT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - no description available"]
    #[inline]
    pub fn nmi_dis(&self) -> NMI_DISR {
        NMI_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - no description available"]
    #[inline]
    pub fn reset_pin_cfg(&self) -> RESET_PIN_CFGR {
        RESET_PIN_CFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - no description available"]
    #[inline]
    pub fn lpboot1(&self) -> LPBOOT1R {
        LPBOOT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - no description available"]
    #[inline]
    pub fn fast_init(&self) -> FAST_INITR {
        FAST_INITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
