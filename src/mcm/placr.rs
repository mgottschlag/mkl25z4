#[doc = "Reader of register PLACR"]
pub type R = crate::R<u32, super::PLACR>;
#[doc = "Writer for register PLACR"]
pub type W = crate::W<u32, super::PLACR>;
#[doc = "Register PLACR `reset()`'s with value 0"]
impl crate::ResetValue for super::PLACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Arbitration select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARB_A {
    #[doc = "0: Fixed-priority arbitration for the crossbar masters"]
    _0 = 0,
    #[doc = "1: Round-robin arbitration for the crossbar masters"]
    _1 = 1,
}
impl From<ARB_A> for bool {
    #[inline(always)]
    fn from(variant: ARB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARB`"]
pub type ARB_R = crate::R<bool, ARB_A>;
impl ARB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARB_A {
        match self.bits {
            false => ARB_A::_0,
            true => ARB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ARB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ARB_A::_1
    }
}
#[doc = "Write proxy for field `ARB`"]
pub struct ARB_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fixed-priority arbitration for the crossbar masters"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARB_A::_0)
    }
    #[doc = "Round-robin arbitration for the crossbar masters"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARB_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `CFCC`"]
pub struct CFCC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Disable Flash Controller Data Caching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFCDA_A {
    #[doc = "0: Enable flash controller data caching"]
    _0 = 0,
    #[doc = "1: Disable flash controller data caching."]
    _1 = 1,
}
impl From<DFCDA_A> for bool {
    #[inline(always)]
    fn from(variant: DFCDA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFCDA`"]
pub type DFCDA_R = crate::R<bool, DFCDA_A>;
impl DFCDA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFCDA_A {
        match self.bits {
            false => DFCDA_A::_0,
            true => DFCDA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFCDA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFCDA_A::_1
    }
}
#[doc = "Write proxy for field `DFCDA`"]
pub struct DFCDA_W<'a> {
    w: &'a mut W,
}
impl<'a> DFCDA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFCDA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable flash controller data caching"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFCDA_A::_0)
    }
    #[doc = "Disable flash controller data caching."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFCDA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Disable Flash Controller Instruction Caching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFCIC_A {
    #[doc = "0: Enable flash controller instruction caching."]
    _0 = 0,
    #[doc = "1: Disable flash controller instruction caching."]
    _1 = 1,
}
impl From<DFCIC_A> for bool {
    #[inline(always)]
    fn from(variant: DFCIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFCIC`"]
pub type DFCIC_R = crate::R<bool, DFCIC_A>;
impl DFCIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFCIC_A {
        match self.bits {
            false => DFCIC_A::_0,
            true => DFCIC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFCIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFCIC_A::_1
    }
}
#[doc = "Write proxy for field `DFCIC`"]
pub struct DFCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DFCIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFCIC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable flash controller instruction caching."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFCIC_A::_0)
    }
    #[doc = "Disable flash controller instruction caching."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFCIC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Disable Flash Controller Cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFCC_A {
    #[doc = "0: Enable flash controller cache."]
    _0 = 0,
    #[doc = "1: Disable flash controller cache."]
    _1 = 1,
}
impl From<DFCC_A> for bool {
    #[inline(always)]
    fn from(variant: DFCC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFCC`"]
pub type DFCC_R = crate::R<bool, DFCC_A>;
impl DFCC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFCC_A {
        match self.bits {
            false => DFCC_A::_0,
            true => DFCC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFCC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFCC_A::_1
    }
}
#[doc = "Write proxy for field `DFCC`"]
pub struct DFCC_W<'a> {
    w: &'a mut W,
}
impl<'a> DFCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFCC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable flash controller cache."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFCC_A::_0)
    }
    #[doc = "Disable flash controller cache."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFCC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Enable Flash Data Speculation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFDS_A {
    #[doc = "0: Disable flash data speculation."]
    _0 = 0,
    #[doc = "1: Enable flash data speculation."]
    _1 = 1,
}
impl From<EFDS_A> for bool {
    #[inline(always)]
    fn from(variant: EFDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EFDS`"]
pub type EFDS_R = crate::R<bool, EFDS_A>;
impl EFDS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFDS_A {
        match self.bits {
            false => EFDS_A::_0,
            true => EFDS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EFDS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EFDS_A::_1
    }
}
#[doc = "Write proxy for field `EFDS`"]
pub struct EFDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EFDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EFDS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable flash data speculation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EFDS_A::_0)
    }
    #[doc = "Enable flash data speculation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EFDS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Disable Flash Controller Speculation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFCS_A {
    #[doc = "0: Enable flash controller speculation."]
    _0 = 0,
    #[doc = "1: Disable flash controller speculation."]
    _1 = 1,
}
impl From<DFCS_A> for bool {
    #[inline(always)]
    fn from(variant: DFCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFCS`"]
pub type DFCS_R = crate::R<bool, DFCS_A>;
impl DFCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFCS_A {
        match self.bits {
            false => DFCS_A::_0,
            true => DFCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFCS_A::_1
    }
}
#[doc = "Write proxy for field `DFCS`"]
pub struct DFCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DFCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable flash controller speculation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFCS_A::_0)
    }
    #[doc = "Disable flash controller speculation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFCS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Enable Stalling Flash Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESFC_A {
    #[doc = "0: Disable stalling flash controller when flash is busy."]
    _0 = 0,
    #[doc = "1: Enable stalling flash controller when flash is busy."]
    _1 = 1,
}
impl From<ESFC_A> for bool {
    #[inline(always)]
    fn from(variant: ESFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ESFC`"]
pub type ESFC_R = crate::R<bool, ESFC_A>;
impl ESFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESFC_A {
        match self.bits {
            false => ESFC_A::_0,
            true => ESFC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESFC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESFC_A::_1
    }
}
#[doc = "Write proxy for field `ESFC`"]
pub struct ESFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ESFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESFC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable stalling flash controller when flash is busy."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESFC_A::_0)
    }
    #[doc = "Enable stalling flash controller when flash is busy."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESFC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - Arbitration select"]
    #[inline(always)]
    pub fn arb(&self) -> ARB_R {
        ARB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Disable Flash Controller Data Caching"]
    #[inline(always)]
    pub fn dfcda(&self) -> DFCDA_R {
        DFCDA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Disable Flash Controller Instruction Caching"]
    #[inline(always)]
    pub fn dfcic(&self) -> DFCIC_R {
        DFCIC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Disable Flash Controller Cache"]
    #[inline(always)]
    pub fn dfcc(&self) -> DFCC_R {
        DFCC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Flash Data Speculation"]
    #[inline(always)]
    pub fn efds(&self) -> EFDS_R {
        EFDS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Disable Flash Controller Speculation"]
    #[inline(always)]
    pub fn dfcs(&self) -> DFCS_R {
        DFCS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable Stalling Flash Controller"]
    #[inline(always)]
    pub fn esfc(&self) -> ESFC_R {
        ESFC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Arbitration select"]
    #[inline(always)]
    pub fn arb(&mut self) -> ARB_W {
        ARB_W { w: self }
    }
    #[doc = "Bit 10 - Clear Flash Controller Cache"]
    #[inline(always)]
    pub fn cfcc(&mut self) -> CFCC_W {
        CFCC_W { w: self }
    }
    #[doc = "Bit 11 - Disable Flash Controller Data Caching"]
    #[inline(always)]
    pub fn dfcda(&mut self) -> DFCDA_W {
        DFCDA_W { w: self }
    }
    #[doc = "Bit 12 - Disable Flash Controller Instruction Caching"]
    #[inline(always)]
    pub fn dfcic(&mut self) -> DFCIC_W {
        DFCIC_W { w: self }
    }
    #[doc = "Bit 13 - Disable Flash Controller Cache"]
    #[inline(always)]
    pub fn dfcc(&mut self) -> DFCC_W {
        DFCC_W { w: self }
    }
    #[doc = "Bit 14 - Enable Flash Data Speculation"]
    #[inline(always)]
    pub fn efds(&mut self) -> EFDS_W {
        EFDS_W { w: self }
    }
    #[doc = "Bit 15 - Disable Flash Controller Speculation"]
    #[inline(always)]
    pub fn dfcs(&mut self) -> DFCS_W {
        DFCS_W { w: self }
    }
    #[doc = "Bit 16 - Enable Stalling Flash Controller"]
    #[inline(always)]
    pub fn esfc(&mut self) -> ESFC_W {
        ESFC_W { w: self }
    }
}
