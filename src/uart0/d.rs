#[doc = "Reader of register D"]
pub type R = crate::R<u8, super::D>;
#[doc = "Writer for register D"]
pub type W = crate::W<u8, super::D>;
#[doc = "Register D `reset()`'s with value 0"]
impl crate::ResetValue for super::D {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `R0T0`"]
pub type R0T0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R0T0`"]
pub struct R0T0_W<'a> {
    w: &'a mut W,
}
impl<'a> R0T0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `R1T1`"]
pub type R1T1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R1T1`"]
pub struct R1T1_W<'a> {
    w: &'a mut W,
}
impl<'a> R1T1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `R2T2`"]
pub type R2T2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R2T2`"]
pub struct R2T2_W<'a> {
    w: &'a mut W,
}
impl<'a> R2T2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `R3T3`"]
pub type R3T3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R3T3`"]
pub struct R3T3_W<'a> {
    w: &'a mut W,
}
impl<'a> R3T3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `R4T4`"]
pub type R4T4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R4T4`"]
pub struct R4T4_W<'a> {
    w: &'a mut W,
}
impl<'a> R4T4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `R5T5`"]
pub type R5T5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R5T5`"]
pub struct R5T5_W<'a> {
    w: &'a mut W,
}
impl<'a> R5T5_W<'a> {
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
#[doc = "Reader of field `R6T6`"]
pub type R6T6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R6T6`"]
pub struct R6T6_W<'a> {
    w: &'a mut W,
}
impl<'a> R6T6_W<'a> {
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
#[doc = "Reader of field `R7T7`"]
pub type R7T7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R7T7`"]
pub struct R7T7_W<'a> {
    w: &'a mut W,
}
impl<'a> R7T7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Read receive data buffer 0 or write transmit data buffer 0."]
    #[inline(always)]
    pub fn r0t0(&self) -> R0T0_R {
        R0T0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read receive data buffer 1 or write transmit data buffer 1."]
    #[inline(always)]
    pub fn r1t1(&self) -> R1T1_R {
        R1T1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read receive data buffer 2 or write transmit data buffer 2."]
    #[inline(always)]
    pub fn r2t2(&self) -> R2T2_R {
        R2T2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read receive data buffer 3 or write transmit data buffer 3."]
    #[inline(always)]
    pub fn r3t3(&self) -> R3T3_R {
        R3T3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read receive data buffer 4 or write transmit data buffer 4."]
    #[inline(always)]
    pub fn r4t4(&self) -> R4T4_R {
        R4T4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read receive data buffer 5 or write transmit data buffer 5."]
    #[inline(always)]
    pub fn r5t5(&self) -> R5T5_R {
        R5T5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read receive data buffer 6 or write transmit data buffer 6."]
    #[inline(always)]
    pub fn r6t6(&self) -> R6T6_R {
        R6T6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Read receive data buffer 7 or write transmit data buffer 7."]
    #[inline(always)]
    pub fn r7t7(&self) -> R7T7_R {
        R7T7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read receive data buffer 0 or write transmit data buffer 0."]
    #[inline(always)]
    pub fn r0t0(&mut self) -> R0T0_W {
        R0T0_W { w: self }
    }
    #[doc = "Bit 1 - Read receive data buffer 1 or write transmit data buffer 1."]
    #[inline(always)]
    pub fn r1t1(&mut self) -> R1T1_W {
        R1T1_W { w: self }
    }
    #[doc = "Bit 2 - Read receive data buffer 2 or write transmit data buffer 2."]
    #[inline(always)]
    pub fn r2t2(&mut self) -> R2T2_W {
        R2T2_W { w: self }
    }
    #[doc = "Bit 3 - Read receive data buffer 3 or write transmit data buffer 3."]
    #[inline(always)]
    pub fn r3t3(&mut self) -> R3T3_W {
        R3T3_W { w: self }
    }
    #[doc = "Bit 4 - Read receive data buffer 4 or write transmit data buffer 4."]
    #[inline(always)]
    pub fn r4t4(&mut self) -> R4T4_W {
        R4T4_W { w: self }
    }
    #[doc = "Bit 5 - Read receive data buffer 5 or write transmit data buffer 5."]
    #[inline(always)]
    pub fn r5t5(&mut self) -> R5T5_W {
        R5T5_W { w: self }
    }
    #[doc = "Bit 6 - Read receive data buffer 6 or write transmit data buffer 6."]
    #[inline(always)]
    pub fn r6t6(&mut self) -> R6T6_W {
        R6T6_W { w: self }
    }
    #[doc = "Bit 7 - Read receive data buffer 7 or write transmit data buffer 7."]
    #[inline(always)]
    pub fn r7t7(&mut self) -> R7T7_W {
        R7T7_W { w: self }
    }
}
