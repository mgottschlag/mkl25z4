#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKDIV1 {
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
#[doc = "Possible values of the field `OUTDIV4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDIV4R {
    #[doc = "Divide-by-1."]
    _000,
    #[doc = "Divide-by-2."]
    _001,
    #[doc = "Divide-by-3."]
    _010,
    #[doc = "Divide-by-4."]
    _011,
    #[doc = "Divide-by-5."]
    _100,
    #[doc = "Divide-by-6."]
    _101,
    #[doc = "Divide-by-7."]
    _110,
    #[doc = "Divide-by-8."]
    _111,
}
impl OUTDIV4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTDIV4R::_000 => 0,
            OUTDIV4R::_001 => 1,
            OUTDIV4R::_010 => 2,
            OUTDIV4R::_011 => 3,
            OUTDIV4R::_100 => 4,
            OUTDIV4R::_101 => 5,
            OUTDIV4R::_110 => 6,
            OUTDIV4R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTDIV4R {
        match value {
            0 => OUTDIV4R::_000,
            1 => OUTDIV4R::_001,
            2 => OUTDIV4R::_010,
            3 => OUTDIV4R::_011,
            4 => OUTDIV4R::_100,
            5 => OUTDIV4R::_101,
            6 => OUTDIV4R::_110,
            7 => OUTDIV4R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == OUTDIV4R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == OUTDIV4R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == OUTDIV4R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == OUTDIV4R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == OUTDIV4R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == OUTDIV4R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == OUTDIV4R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == OUTDIV4R::_111
    }
}
#[doc = "Possible values of the field `OUTDIV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDIV1R {
    #[doc = "Divide-by-1."]
    _0000,
    #[doc = "Divide-by-2."]
    _0001,
    #[doc = "Divide-by-3."]
    _0010,
    #[doc = "Divide-by-4."]
    _0011,
    #[doc = "Divide-by-5."]
    _0100,
    #[doc = "Divide-by-6."]
    _0101,
    #[doc = "Divide-by-7."]
    _0110,
    #[doc = "Divide-by-8."]
    _0111,
    #[doc = "Divide-by-9."]
    _1000,
    #[doc = "Divide-by-10."]
    _1001,
    #[doc = "Divide-by-11."]
    _1010,
    #[doc = "Divide-by-12."]
    _1011,
    #[doc = "Divide-by-13."]
    _1100,
    #[doc = "Divide-by-14."]
    _1101,
    #[doc = "Divide-by-15."]
    _1110,
    #[doc = "Divide-by-16."]
    _1111,
}
impl OUTDIV1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTDIV1R::_0000 => 0,
            OUTDIV1R::_0001 => 1,
            OUTDIV1R::_0010 => 2,
            OUTDIV1R::_0011 => 3,
            OUTDIV1R::_0100 => 4,
            OUTDIV1R::_0101 => 5,
            OUTDIV1R::_0110 => 6,
            OUTDIV1R::_0111 => 7,
            OUTDIV1R::_1000 => 8,
            OUTDIV1R::_1001 => 9,
            OUTDIV1R::_1010 => 10,
            OUTDIV1R::_1011 => 11,
            OUTDIV1R::_1100 => 12,
            OUTDIV1R::_1101 => 13,
            OUTDIV1R::_1110 => 14,
            OUTDIV1R::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTDIV1R {
        match value {
            0 => OUTDIV1R::_0000,
            1 => OUTDIV1R::_0001,
            2 => OUTDIV1R::_0010,
            3 => OUTDIV1R::_0011,
            4 => OUTDIV1R::_0100,
            5 => OUTDIV1R::_0101,
            6 => OUTDIV1R::_0110,
            7 => OUTDIV1R::_0111,
            8 => OUTDIV1R::_1000,
            9 => OUTDIV1R::_1001,
            10 => OUTDIV1R::_1010,
            11 => OUTDIV1R::_1011,
            12 => OUTDIV1R::_1100,
            13 => OUTDIV1R::_1101,
            14 => OUTDIV1R::_1110,
            15 => OUTDIV1R::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == OUTDIV1R::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == OUTDIV1R::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == OUTDIV1R::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == OUTDIV1R::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == OUTDIV1R::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == OUTDIV1R::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == OUTDIV1R::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == OUTDIV1R::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == OUTDIV1R::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == OUTDIV1R::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == OUTDIV1R::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == OUTDIV1R::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == OUTDIV1R::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == OUTDIV1R::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == OUTDIV1R::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == OUTDIV1R::_1111
    }
}
#[doc = "Values that can be written to the field `OUTDIV4`"]
pub enum OUTDIV4W {
    #[doc = "Divide-by-1."]
    _000,
    #[doc = "Divide-by-2."]
    _001,
    #[doc = "Divide-by-3."]
    _010,
    #[doc = "Divide-by-4."]
    _011,
    #[doc = "Divide-by-5."]
    _100,
    #[doc = "Divide-by-6."]
    _101,
    #[doc = "Divide-by-7."]
    _110,
    #[doc = "Divide-by-8."]
    _111,
}
impl OUTDIV4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTDIV4W::_000 => 0,
            OUTDIV4W::_001 => 1,
            OUTDIV4W::_010 => 2,
            OUTDIV4W::_011 => 3,
            OUTDIV4W::_100 => 4,
            OUTDIV4W::_101 => 5,
            OUTDIV4W::_110 => 6,
            OUTDIV4W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTDIV4W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTDIV4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTDIV4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide-by-1."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(OUTDIV4W::_000)
    }
    #[doc = "Divide-by-2."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(OUTDIV4W::_001)
    }
    #[doc = "Divide-by-3."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(OUTDIV4W::_010)
    }
    #[doc = "Divide-by-4."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(OUTDIV4W::_011)
    }
    #[doc = "Divide-by-5."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(OUTDIV4W::_100)
    }
    #[doc = "Divide-by-6."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(OUTDIV4W::_101)
    }
    #[doc = "Divide-by-7."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(OUTDIV4W::_110)
    }
    #[doc = "Divide-by-8."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(OUTDIV4W::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OUTDIV1`"]
pub enum OUTDIV1W {
    #[doc = "Divide-by-1."]
    _0000,
    #[doc = "Divide-by-2."]
    _0001,
    #[doc = "Divide-by-3."]
    _0010,
    #[doc = "Divide-by-4."]
    _0011,
    #[doc = "Divide-by-5."]
    _0100,
    #[doc = "Divide-by-6."]
    _0101,
    #[doc = "Divide-by-7."]
    _0110,
    #[doc = "Divide-by-8."]
    _0111,
    #[doc = "Divide-by-9."]
    _1000,
    #[doc = "Divide-by-10."]
    _1001,
    #[doc = "Divide-by-11."]
    _1010,
    #[doc = "Divide-by-12."]
    _1011,
    #[doc = "Divide-by-13."]
    _1100,
    #[doc = "Divide-by-14."]
    _1101,
    #[doc = "Divide-by-15."]
    _1110,
    #[doc = "Divide-by-16."]
    _1111,
}
impl OUTDIV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTDIV1W::_0000 => 0,
            OUTDIV1W::_0001 => 1,
            OUTDIV1W::_0010 => 2,
            OUTDIV1W::_0011 => 3,
            OUTDIV1W::_0100 => 4,
            OUTDIV1W::_0101 => 5,
            OUTDIV1W::_0110 => 6,
            OUTDIV1W::_0111 => 7,
            OUTDIV1W::_1000 => 8,
            OUTDIV1W::_1001 => 9,
            OUTDIV1W::_1010 => 10,
            OUTDIV1W::_1011 => 11,
            OUTDIV1W::_1100 => 12,
            OUTDIV1W::_1101 => 13,
            OUTDIV1W::_1110 => 14,
            OUTDIV1W::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTDIV1W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTDIV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTDIV1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide-by-1."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(OUTDIV1W::_0000)
    }
    #[doc = "Divide-by-2."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(OUTDIV1W::_0001)
    }
    #[doc = "Divide-by-3."]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(OUTDIV1W::_0010)
    }
    #[doc = "Divide-by-4."]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(OUTDIV1W::_0011)
    }
    #[doc = "Divide-by-5."]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(OUTDIV1W::_0100)
    }
    #[doc = "Divide-by-6."]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(OUTDIV1W::_0101)
    }
    #[doc = "Divide-by-7."]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(OUTDIV1W::_0110)
    }
    #[doc = "Divide-by-8."]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(OUTDIV1W::_0111)
    }
    #[doc = "Divide-by-9."]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(OUTDIV1W::_1000)
    }
    #[doc = "Divide-by-10."]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(OUTDIV1W::_1001)
    }
    #[doc = "Divide-by-11."]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(OUTDIV1W::_1010)
    }
    #[doc = "Divide-by-12."]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(OUTDIV1W::_1011)
    }
    #[doc = "Divide-by-13."]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(OUTDIV1W::_1100)
    }
    #[doc = "Divide-by-14."]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(OUTDIV1W::_1101)
    }
    #[doc = "Divide-by-15."]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(OUTDIV1W::_1110)
    }
    #[doc = "Divide-by-16."]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(OUTDIV1W::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 16:18 - Clock 4 output divider value"]
    #[inline]
    pub fn outdiv4(&self) -> OUTDIV4R {
        OUTDIV4R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Clock 1 output divider value"]
    #[inline]
    pub fn outdiv1(&self) -> OUTDIV1R {
        OUTDIV1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65536 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:18 - Clock 4 output divider value"]
    #[inline]
    pub fn outdiv4(&mut self) -> _OUTDIV4W {
        _OUTDIV4W { w: self }
    }
    #[doc = "Bits 28:31 - Clock 1 output divider value"]
    #[inline]
    pub fn outdiv1(&mut self) -> _OUTDIV1W {
        _OUTDIV1W { w: self }
    }
}
