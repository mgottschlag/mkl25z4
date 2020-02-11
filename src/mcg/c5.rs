#[doc = "Reader of register C5"]
pub type R = crate::R<u8, super::C5>;
#[doc = "Writer for register C5"]
pub type W = crate::W<u8, super::C5>;
#[doc = "Register C5 `reset()`'s with value 0"]
impl crate::ResetValue for super::C5 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRDIV0`"]
pub type PRDIV0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRDIV0`"]
pub struct PRDIV0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDIV0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "PLL Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTEN0_A {
    #[doc = "0: MCGPLLCLK is disabled in any of the Stop modes."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK is enabled if system is in Normal Stop mode."]
    _1 = 1,
}
impl From<PLLSTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLSTEN0`"]
pub type PLLSTEN0_R = crate::R<bool, PLLSTEN0_A>;
impl PLLSTEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTEN0_A {
        match self.bits {
            false => PLLSTEN0_A::_0,
            true => PLLSTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLSTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLSTEN0_A::_1
    }
}
#[doc = "Write proxy for field `PLLSTEN0`"]
pub struct PLLSTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSTEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCGPLLCLK is disabled in any of the Stop modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLSTEN0_A::_0)
    }
    #[doc = "MCGPLLCLK is enabled if system is in Normal Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLSTEN0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "PLL Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLCLKEN0_A {
    #[doc = "0: MCGPLLCLK is inactive."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK is active."]
    _1 = 1,
}
impl From<PLLCLKEN0_A> for bool {
    #[inline(always)]
    fn from(variant: PLLCLKEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLCLKEN0`"]
pub type PLLCLKEN0_R = crate::R<bool, PLLCLKEN0_A>;
impl PLLCLKEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLCLKEN0_A {
        match self.bits {
            false => PLLCLKEN0_A::_0,
            true => PLLCLKEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLCLKEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLCLKEN0_A::_1
    }
}
#[doc = "Write proxy for field `PLLCLKEN0`"]
pub struct PLLCLKEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLCLKEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLCLKEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCGPLLCLK is inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLCLKEN0_A::_0)
    }
    #[doc = "MCGPLLCLK is active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLCLKEN0_A::_1)
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
impl R {
    #[doc = "Bits 0:4 - PLL External Reference Divider"]
    #[inline(always)]
    pub fn prdiv0(&self) -> PRDIV0_R {
        PRDIV0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline(always)]
    pub fn pllsten0(&self) -> PLLSTEN0_R {
        PLLSTEN0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline(always)]
    pub fn pllclken0(&self) -> PLLCLKEN0_R {
        PLLCLKEN0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL External Reference Divider"]
    #[inline(always)]
    pub fn prdiv0(&mut self) -> PRDIV0_W {
        PRDIV0_W { w: self }
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline(always)]
    pub fn pllsten0(&mut self) -> PLLSTEN0_W {
        PLLSTEN0_W { w: self }
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline(always)]
    pub fn pllclken0(&mut self) -> PLLCLKEN0_W {
        PLLCLKEN0_W { w: self }
    }
}
