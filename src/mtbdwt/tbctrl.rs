#[doc = "Reader of register TBCTRL"]
pub type R = crate::R<u32, super::TBCTRL>;
#[doc = "Writer for register TBCTRL"]
pub type W = crate::W<u32, super::TBCTRL>;
#[doc = "Register TBCTRL `reset()`'s with value 0x2000_0000"]
impl crate::ResetValue for super::TBCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_0000
    }
}
#[doc = "Action based on Comparator 0 match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACOMP0_A {
    #[doc = "0: Trigger TSTOP based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    _0 = 0,
    #[doc = "1: Trigger TSTART based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    _1 = 1,
}
impl From<ACOMP0_A> for bool {
    #[inline(always)]
    fn from(variant: ACOMP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACOMP0`"]
pub type ACOMP0_R = crate::R<bool, ACOMP0_A>;
impl ACOMP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACOMP0_A {
        match self.bits {
            false => ACOMP0_A::_0,
            true => ACOMP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACOMP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACOMP0_A::_1
    }
}
#[doc = "Write proxy for field `ACOMP0`"]
pub struct ACOMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACOMP0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACOMP0_A::_0)
    }
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACOMP0_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Action based on Comparator 1 match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACOMP1_A {
    #[doc = "0: Trigger TSTOP based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    _0 = 0,
    #[doc = "1: Trigger TSTART based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    _1 = 1,
}
impl From<ACOMP1_A> for bool {
    #[inline(always)]
    fn from(variant: ACOMP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACOMP1`"]
pub type ACOMP1_R = crate::R<bool, ACOMP1_A>;
impl ACOMP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACOMP1_A {
        match self.bits {
            false => ACOMP1_A::_0,
            true => ACOMP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACOMP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACOMP1_A::_1
    }
}
#[doc = "Write proxy for field `ACOMP1`"]
pub struct ACOMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACOMP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACOMP1_A::_0)
    }
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACOMP1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `NUMCOMP`"]
pub type NUMCOMP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Action based on Comparator 0 match"]
    #[inline(always)]
    pub fn acomp0(&self) -> ACOMP0_R {
        ACOMP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Action based on Comparator 1 match"]
    #[inline(always)]
    pub fn acomp1(&self) -> ACOMP1_R {
        ACOMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - Number of Comparators"]
    #[inline(always)]
    pub fn numcomp(&self) -> NUMCOMP_R {
        NUMCOMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Action based on Comparator 0 match"]
    #[inline(always)]
    pub fn acomp0(&mut self) -> ACOMP0_W {
        ACOMP0_W { w: self }
    }
    #[doc = "Bit 1 - Action based on Comparator 1 match"]
    #[inline(always)]
    pub fn acomp1(&mut self) -> ACOMP1_W {
        ACOMP1_W { w: self }
    }
}
