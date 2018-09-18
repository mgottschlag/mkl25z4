#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT4 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `TPM1CH0SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1CH0SRCR {
    #[doc = "TPM1_CH0 signal"]
    _0,
    #[doc = "CMP0 output"]
    _1,
}
impl TPM1CH0SRCR {
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
            TPM1CH0SRCR::_0 => false,
            TPM1CH0SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPM1CH0SRCR {
        match value {
            false => TPM1CH0SRCR::_0,
            true => TPM1CH0SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPM1CH0SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPM1CH0SRCR::_1
    }
}
#[doc = "Possible values of the field `TPM2CH0SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM2CH0SRCR {
    #[doc = "TPM2_CH0 signal"]
    _0,
    #[doc = "CMP0 output"]
    _1,
}
impl TPM2CH0SRCR {
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
            TPM2CH0SRCR::_0 => false,
            TPM2CH0SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPM2CH0SRCR {
        match value {
            false => TPM2CH0SRCR::_0,
            true => TPM2CH0SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPM2CH0SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPM2CH0SRCR::_1
    }
}
#[doc = "Possible values of the field `TPM0CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM0CLKSELR {
    #[doc = "TPM0 external clock driven by TPM_CLKIN0 pin."]
    _0,
    #[doc = "TPM0 external clock driven by TPM_CLKIN1 pin."]
    _1,
}
impl TPM0CLKSELR {
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
            TPM0CLKSELR::_0 => false,
            TPM0CLKSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPM0CLKSELR {
        match value {
            false => TPM0CLKSELR::_0,
            true => TPM0CLKSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPM0CLKSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPM0CLKSELR::_1
    }
}
#[doc = "Possible values of the field `TPM1CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1CLKSELR {
    #[doc = "TPM1 external clock driven by TPM_CLKIN0 pin."]
    _0,
    #[doc = "TPM1 external clock driven by TPM_CLKIN1 pin."]
    _1,
}
impl TPM1CLKSELR {
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
            TPM1CLKSELR::_0 => false,
            TPM1CLKSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPM1CLKSELR {
        match value {
            false => TPM1CLKSELR::_0,
            true => TPM1CLKSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPM1CLKSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPM1CLKSELR::_1
    }
}
#[doc = "Possible values of the field `TPM2CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM2CLKSELR {
    #[doc = "TPM2 external clock driven by TPM_CLKIN0 pin."]
    _0,
    #[doc = "TPM2 external clock driven by TPM_CLKIN1 pin."]
    _1,
}
impl TPM2CLKSELR {
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
            TPM2CLKSELR::_0 => false,
            TPM2CLKSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPM2CLKSELR {
        match value {
            false => TPM2CLKSELR::_0,
            true => TPM2CLKSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPM2CLKSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPM2CLKSELR::_1
    }
}
#[doc = "Values that can be written to the field `TPM1CH0SRC`"]
pub enum TPM1CH0SRCW {
    #[doc = "TPM1_CH0 signal"]
    _0,
    #[doc = "CMP0 output"]
    _1,
}
impl TPM1CH0SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPM1CH0SRCW::_0 => false,
            TPM1CH0SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPM1CH0SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TPM1CH0SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPM1CH0SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TPM1_CH0 signal"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM1CH0SRCW::_0)
    }
    #[doc = "CMP0 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM1CH0SRCW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPM2CH0SRC`"]
pub enum TPM2CH0SRCW {
    #[doc = "TPM2_CH0 signal"]
    _0,
    #[doc = "CMP0 output"]
    _1,
}
impl TPM2CH0SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPM2CH0SRCW::_0 => false,
            TPM2CH0SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPM2CH0SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TPM2CH0SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPM2CH0SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TPM2_CH0 signal"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM2CH0SRCW::_0)
    }
    #[doc = "CMP0 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM2CH0SRCW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPM0CLKSEL`"]
pub enum TPM0CLKSELW {
    #[doc = "TPM0 external clock driven by TPM_CLKIN0 pin."]
    _0,
    #[doc = "TPM0 external clock driven by TPM_CLKIN1 pin."]
    _1,
}
impl TPM0CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPM0CLKSELW::_0 => false,
            TPM0CLKSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPM0CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TPM0CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPM0CLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TPM0 external clock driven by TPM_CLKIN0 pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM0CLKSELW::_0)
    }
    #[doc = "TPM0 external clock driven by TPM_CLKIN1 pin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM0CLKSELW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPM1CLKSEL`"]
pub enum TPM1CLKSELW {
    #[doc = "TPM1 external clock driven by TPM_CLKIN0 pin."]
    _0,
    #[doc = "TPM1 external clock driven by TPM_CLKIN1 pin."]
    _1,
}
impl TPM1CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPM1CLKSELW::_0 => false,
            TPM1CLKSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPM1CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TPM1CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPM1CLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TPM1 external clock driven by TPM_CLKIN0 pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM1CLKSELW::_0)
    }
    #[doc = "TPM1 external clock driven by TPM_CLKIN1 pin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM1CLKSELW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPM2CLKSEL`"]
pub enum TPM2CLKSELW {
    #[doc = "TPM2 external clock driven by TPM_CLKIN0 pin."]
    _0,
    #[doc = "TPM2 external clock driven by TPM_CLKIN1 pin."]
    _1,
}
impl TPM2CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPM2CLKSELW::_0 => false,
            TPM2CLKSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPM2CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TPM2CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPM2CLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TPM2 external clock driven by TPM_CLKIN0 pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM2CLKSELW::_0)
    }
    #[doc = "TPM2 external clock driven by TPM_CLKIN1 pin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM2CLKSELW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 18 - TPM1 channel 0 input capture source select"]
    #[inline]
    pub fn tpm1ch0src(&self) -> TPM1CH0SRCR {
        TPM1CH0SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - TPM2 channel 0 input capture source select"]
    #[inline]
    pub fn tpm2ch0src(&self) -> TPM2CH0SRCR {
        TPM2CH0SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - TPM0 External Clock Pin Select"]
    #[inline]
    pub fn tpm0clksel(&self) -> TPM0CLKSELR {
        TPM0CLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - TPM1 External Clock Pin Select"]
    #[inline]
    pub fn tpm1clksel(&self) -> TPM1CLKSELR {
        TPM1CLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - TPM2 External Clock Pin Select"]
    #[inline]
    pub fn tpm2clksel(&self) -> TPM2CLKSELR {
        TPM2CLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 18 - TPM1 channel 0 input capture source select"]
    #[inline]
    pub fn tpm1ch0src(&mut self) -> _TPM1CH0SRCW {
        _TPM1CH0SRCW { w: self }
    }
    #[doc = "Bit 20 - TPM2 channel 0 input capture source select"]
    #[inline]
    pub fn tpm2ch0src(&mut self) -> _TPM2CH0SRCW {
        _TPM2CH0SRCW { w: self }
    }
    #[doc = "Bit 24 - TPM0 External Clock Pin Select"]
    #[inline]
    pub fn tpm0clksel(&mut self) -> _TPM0CLKSELW {
        _TPM0CLKSELW { w: self }
    }
    #[doc = "Bit 25 - TPM1 External Clock Pin Select"]
    #[inline]
    pub fn tpm1clksel(&mut self) -> _TPM1CLKSELW {
        _TPM1CLKSELW { w: self }
    }
    #[doc = "Bit 26 - TPM2 External Clock Pin Select"]
    #[inline]
    pub fn tpm2clksel(&mut self) -> _TPM2CLKSELW {
        _TPM2CLKSELW { w: self }
    }
}
