#[doc = "Reader of register S"]
pub type R = crate::R<u8, super::S>;
#[doc = "Writer for register S"]
pub type W = crate::W<u8, super::S>;
#[doc = "Register S `reset()`'s with value 0x20"]
impl crate::ResetValue for super::S {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Master mode fault flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODF_A {
    #[doc = "0: No mode fault error"]
    _0 = 0,
    #[doc = "1: Mode fault error detected"]
    _1 = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODF`"]
pub type MODF_R = crate::R<bool, MODF_A>;
impl MODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::_0,
            true => MODF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODF_A::_1
    }
}
#[doc = "SPI transmit buffer empty flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPTEF_A {
    #[doc = "0: SPI transmit buffer not empty"]
    _0 = 0,
    #[doc = "1: SPI transmit buffer empty"]
    _1 = 1,
}
impl From<SPTEF_A> for bool {
    #[inline(always)]
    fn from(variant: SPTEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPTEF`"]
pub type SPTEF_R = crate::R<bool, SPTEF_A>;
impl SPTEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPTEF_A {
        match self.bits {
            false => SPTEF_A::_0,
            true => SPTEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPTEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPTEF_A::_1
    }
}
#[doc = "SPI match flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPMF_A {
    #[doc = "0: Value in the receive data buffer does not match the value in the M register"]
    _0 = 0,
    #[doc = "1: Value in the receive data buffer matches the value in the M register"]
    _1 = 1,
}
impl From<SPMF_A> for bool {
    #[inline(always)]
    fn from(variant: SPMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPMF`"]
pub type SPMF_R = crate::R<bool, SPMF_A>;
impl SPMF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPMF_A {
        match self.bits {
            false => SPMF_A::_0,
            true => SPMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPMF_A::_1
    }
}
#[doc = "Write proxy for field `SPMF`"]
pub struct SPMF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPMF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPMF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Value in the receive data buffer does not match the value in the M register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPMF_A::_0)
    }
    #[doc = "Value in the receive data buffer matches the value in the M register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPMF_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "SPI read buffer full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPRF_A {
    #[doc = "0: No data available in the receive data buffer"]
    _0 = 0,
    #[doc = "1: Data available in the receive data buffer"]
    _1 = 1,
}
impl From<SPRF_A> for bool {
    #[inline(always)]
    fn from(variant: SPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPRF`"]
pub type SPRF_R = crate::R<bool, SPRF_A>;
impl SPRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRF_A {
        match self.bits {
            false => SPRF_A::_0,
            true => SPRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPRF_A::_1
    }
}
impl R {
    #[doc = "Bit 4 - Master mode fault flag"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI transmit buffer empty flag"]
    #[inline(always)]
    pub fn sptef(&self) -> SPTEF_R {
        SPTEF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI match flag"]
    #[inline(always)]
    pub fn spmf(&self) -> SPMF_R {
        SPMF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPI read buffer full flag"]
    #[inline(always)]
    pub fn sprf(&self) -> SPRF_R {
        SPRF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - SPI match flag"]
    #[inline(always)]
    pub fn spmf(&mut self) -> SPMF_W {
        SPMF_W { w: self }
    }
}
