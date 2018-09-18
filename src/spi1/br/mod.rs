#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::BR {
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
#[doc = "Possible values of the field `SPR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPRR {
    #[doc = "Baud rate divisor is 2"]
    _0000,
    #[doc = "Baud rate divisor is 4"]
    _0001,
    #[doc = "Baud rate divisor is 8"]
    _0010,
    #[doc = "Baud rate divisor is 16"]
    _0011,
    #[doc = "Baud rate divisor is 32"]
    _0100,
    #[doc = "Baud rate divisor is 64"]
    _0101,
    #[doc = "Baud rate divisor is 128"]
    _0110,
    #[doc = "Baud rate divisor is 256"]
    _0111,
    #[doc = "Baud rate divisor is 512"]
    _1000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPRR::_0000 => 0,
            SPRR::_0001 => 1,
            SPRR::_0010 => 2,
            SPRR::_0011 => 3,
            SPRR::_0100 => 4,
            SPRR::_0101 => 5,
            SPRR::_0110 => 6,
            SPRR::_0111 => 7,
            SPRR::_1000 => 8,
            SPRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPRR {
        match value {
            0 => SPRR::_0000,
            1 => SPRR::_0001,
            2 => SPRR::_0010,
            3 => SPRR::_0011,
            4 => SPRR::_0100,
            5 => SPRR::_0101,
            6 => SPRR::_0110,
            7 => SPRR::_0111,
            8 => SPRR::_1000,
            i => SPRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == SPRR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == SPRR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == SPRR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == SPRR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == SPRR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == SPRR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == SPRR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == SPRR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == SPRR::_1000
    }
}
#[doc = "Possible values of the field `SPPR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPPRR {
    #[doc = "Baud rate prescaler divisor is 1"]
    _000,
    #[doc = "Baud rate prescaler divisor is 2"]
    _001,
    #[doc = "Baud rate prescaler divisor is 3"]
    _010,
    #[doc = "Baud rate prescaler divisor is 4"]
    _011,
    #[doc = "Baud rate prescaler divisor is 5"]
    _100,
    #[doc = "Baud rate prescaler divisor is 6"]
    _101,
    #[doc = "Baud rate prescaler divisor is 7"]
    _110,
    #[doc = "Baud rate prescaler divisor is 8"]
    _111,
}
impl SPPRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPPRR::_000 => 0,
            SPPRR::_001 => 1,
            SPPRR::_010 => 2,
            SPPRR::_011 => 3,
            SPPRR::_100 => 4,
            SPPRR::_101 => 5,
            SPPRR::_110 => 6,
            SPPRR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPPRR {
        match value {
            0 => SPPRR::_000,
            1 => SPPRR::_001,
            2 => SPPRR::_010,
            3 => SPPRR::_011,
            4 => SPPRR::_100,
            5 => SPPRR::_101,
            6 => SPPRR::_110,
            7 => SPPRR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == SPPRR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == SPPRR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == SPPRR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == SPPRR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == SPPRR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == SPPRR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == SPPRR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == SPPRR::_111
    }
}
#[doc = "Values that can be written to the field `SPR`"]
pub enum SPRW {
    #[doc = "Baud rate divisor is 2"]
    _0000,
    #[doc = "Baud rate divisor is 4"]
    _0001,
    #[doc = "Baud rate divisor is 8"]
    _0010,
    #[doc = "Baud rate divisor is 16"]
    _0011,
    #[doc = "Baud rate divisor is 32"]
    _0100,
    #[doc = "Baud rate divisor is 64"]
    _0101,
    #[doc = "Baud rate divisor is 128"]
    _0110,
    #[doc = "Baud rate divisor is 256"]
    _0111,
    #[doc = "Baud rate divisor is 512"]
    _1000,
}
impl SPRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPRW::_0000 => 0,
            SPRW::_0001 => 1,
            SPRW::_0010 => 2,
            SPRW::_0011 => 3,
            SPRW::_0100 => 4,
            SPRW::_0101 => 5,
            SPRW::_0110 => 6,
            SPRW::_0111 => 7,
            SPRW::_1000 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPRW<'a> {
    w: &'a mut W,
}
impl<'a> _SPRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Baud rate divisor is 2"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(SPRW::_0000)
    }
    #[doc = "Baud rate divisor is 4"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(SPRW::_0001)
    }
    #[doc = "Baud rate divisor is 8"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(SPRW::_0010)
    }
    #[doc = "Baud rate divisor is 16"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(SPRW::_0011)
    }
    #[doc = "Baud rate divisor is 32"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(SPRW::_0100)
    }
    #[doc = "Baud rate divisor is 64"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(SPRW::_0101)
    }
    #[doc = "Baud rate divisor is 128"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(SPRW::_0110)
    }
    #[doc = "Baud rate divisor is 256"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(SPRW::_0111)
    }
    #[doc = "Baud rate divisor is 512"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(SPRW::_1000)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPPR`"]
pub enum SPPRW {
    #[doc = "Baud rate prescaler divisor is 1"]
    _000,
    #[doc = "Baud rate prescaler divisor is 2"]
    _001,
    #[doc = "Baud rate prescaler divisor is 3"]
    _010,
    #[doc = "Baud rate prescaler divisor is 4"]
    _011,
    #[doc = "Baud rate prescaler divisor is 5"]
    _100,
    #[doc = "Baud rate prescaler divisor is 6"]
    _101,
    #[doc = "Baud rate prescaler divisor is 7"]
    _110,
    #[doc = "Baud rate prescaler divisor is 8"]
    _111,
}
impl SPPRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPPRW::_000 => 0,
            SPPRW::_001 => 1,
            SPPRW::_010 => 2,
            SPPRW::_011 => 3,
            SPPRW::_100 => 4,
            SPPRW::_101 => 5,
            SPPRW::_110 => 6,
            SPPRW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPPRW<'a> {
    w: &'a mut W,
}
impl<'a> _SPPRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPPRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Baud rate prescaler divisor is 1"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(SPPRW::_000)
    }
    #[doc = "Baud rate prescaler divisor is 2"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(SPPRW::_001)
    }
    #[doc = "Baud rate prescaler divisor is 3"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(SPPRW::_010)
    }
    #[doc = "Baud rate prescaler divisor is 4"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(SPPRW::_011)
    }
    #[doc = "Baud rate prescaler divisor is 5"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(SPPRW::_100)
    }
    #[doc = "Baud rate prescaler divisor is 6"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(SPPRW::_101)
    }
    #[doc = "Baud rate prescaler divisor is 7"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(SPPRW::_110)
    }
    #[doc = "Baud rate prescaler divisor is 8"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(SPPRW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - SPI baud rate divisor"]
    #[inline]
    pub fn spr(&self) -> SPRR {
        SPRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:6 - SPI baud rate prescale divisor"]
    #[inline]
    pub fn sppr(&self) -> SPPRR {
        SPPRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - SPI baud rate divisor"]
    #[inline]
    pub fn spr(&mut self) -> _SPRW {
        _SPRW { w: self }
    }
    #[doc = "Bits 4:6 - SPI baud rate prescale divisor"]
    #[inline]
    pub fn sppr(&mut self) -> _SPPRW {
        _SPPRW { w: self }
    }
}
