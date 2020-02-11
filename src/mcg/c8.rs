#[doc = "Reader of register C8"]
pub type R = crate::R<u8, super::C8>;
#[doc = "Writer for register C8"]
pub type W = crate::W<u8, super::C8>;
#[doc = "Register C8 `reset()`'s with value 0x80"]
impl crate::ResetValue for super::C8 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "PLL Loss of Lock Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLRE_A {
    #[doc = "0: Interrupt request is generated on a PLL loss of lock indication. The PLL loss of lock interrupt enable bit must also be set to generate the interrupt request."]
    _0 = 0,
    #[doc = "1: Generate a reset request on a PLL loss of lock indication."]
    _1 = 1,
}
impl From<LOLRE_A> for bool {
    #[inline(always)]
    fn from(variant: LOLRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOLRE`"]
pub type LOLRE_R = crate::R<bool, LOLRE_A>;
impl LOLRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOLRE_A {
        match self.bits {
            false => LOLRE_A::_0,
            true => LOLRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOLRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOLRE_A::_1
    }
}
#[doc = "Write proxy for field `LOLRE`"]
pub struct LOLRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOLRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOLRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt request is generated on a PLL loss of lock indication. The PLL loss of lock interrupt enable bit must also be set to generate the interrupt request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLRE_A::_0)
    }
    #[doc = "Generate a reset request on a PLL loss of lock indication."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLRE_A::_1)
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
    #[doc = "Bit 6 - PLL Loss of Lock Reset Enable"]
    #[inline(always)]
    pub fn lolre(&self) -> LOLRE_R {
        LOLRE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - PLL Loss of Lock Reset Enable"]
    #[inline(always)]
    pub fn lolre(&mut self) -> LOLRE_W {
        LOLRE_W { w: self }
    }
}
