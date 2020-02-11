#[doc = "Reader of register FOPT"]
pub type R = crate::R<u8, super::FOPT>;
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBOOT0_A {
    #[doc = "0: Core and system clock divider (OUTDIV1) is 0x7 (divide by 8) when LPBOOT1=0 or 0x1 (divide by 2) when LPBOOT1=1."]
    _00 = 0,
    #[doc = "1: Core and system clock divider (OUTDIV1) is 0x3 (divide by 4) when LPBOOT1=0 or 0x0 (divide by 1) when LPBOOT1=1."]
    _01 = 1,
}
impl From<LPBOOT0_A> for bool {
    #[inline(always)]
    fn from(variant: LPBOOT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPBOOT0`"]
pub type LPBOOT0_R = crate::R<bool, LPBOOT0_A>;
impl LPBOOT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPBOOT0_A {
        match self.bits {
            false => LPBOOT0_A::_00,
            true => LPBOOT0_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPBOOT0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPBOOT0_A::_01
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMI_DIS_A {
    #[doc = "0: NMI interrupts are always blocked"]
    _00 = 0,
    #[doc = "1: NMI_b pin/interrupts reset default to enabled"]
    _01 = 1,
}
impl From<NMI_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: NMI_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NMI_DIS`"]
pub type NMI_DIS_R = crate::R<bool, NMI_DIS_A>;
impl NMI_DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMI_DIS_A {
        match self.bits {
            false => NMI_DIS_A::_00,
            true => NMI_DIS_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NMI_DIS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NMI_DIS_A::_01
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_PIN_CFG_A {
    #[doc = "0: RESET pin is disabled following a POR and cannot be enabled as reset function"]
    _00 = 0,
    #[doc = "1: RESET_b pin is dedicated"]
    _01 = 1,
}
impl From<RESET_PIN_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_PIN_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESET_PIN_CFG`"]
pub type RESET_PIN_CFG_R = crate::R<bool, RESET_PIN_CFG_A>;
impl RESET_PIN_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_PIN_CFG_A {
        match self.bits {
            false => RESET_PIN_CFG_A::_00,
            true => RESET_PIN_CFG_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RESET_PIN_CFG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RESET_PIN_CFG_A::_01
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBOOT1_A {
    #[doc = "0: Core and system clock divider (OUTDIV1) is 0x7 (divide by 8) when LPBOOT0=0 or 0x3 (divide by 4) when LPBOOT0=1."]
    _00 = 0,
    #[doc = "1: Core and system clock divider (OUTDIV1) is 0x1 (divide by 2) when LPBOOT0=0 or 0x0 (divide by 1) when LPBOOT0=1."]
    _01 = 1,
}
impl From<LPBOOT1_A> for bool {
    #[inline(always)]
    fn from(variant: LPBOOT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPBOOT1`"]
pub type LPBOOT1_R = crate::R<bool, LPBOOT1_A>;
impl LPBOOT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPBOOT1_A {
        match self.bits {
            false => LPBOOT1_A::_00,
            true => LPBOOT1_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPBOOT1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPBOOT1_A::_01
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_INIT_A {
    #[doc = "0: Slower initialization"]
    _00 = 0,
    #[doc = "1: Fast Initialization"]
    _01 = 1,
}
impl From<FAST_INIT_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAST_INIT`"]
pub type FAST_INIT_R = crate::R<bool, FAST_INIT_A>;
impl FAST_INIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_INIT_A {
        match self.bits {
            false => FAST_INIT_A::_00,
            true => FAST_INIT_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FAST_INIT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FAST_INIT_A::_01
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn lpboot0(&self) -> LPBOOT0_R {
        LPBOOT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn nmi_dis(&self) -> NMI_DIS_R {
        NMI_DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn reset_pin_cfg(&self) -> RESET_PIN_CFG_R {
        RESET_PIN_CFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn lpboot1(&self) -> LPBOOT1_R {
        LPBOOT1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn fast_init(&self) -> FAST_INIT_R {
        FAST_INIT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
