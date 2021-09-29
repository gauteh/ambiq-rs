#[doc = "Register `OUTCFG0` reader"]
pub struct R(crate::R<OUTCFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTCFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTCFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTCFG0` writer"]
pub struct W(crate::W<OUTCFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OUTCFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTCFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad output 9 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG9_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is B0OUT. value."]
    B0OUT = 5,
    #[doc = "4: Output is A4OUT. value."]
    A4OUT = 4,
    #[doc = "3: Output is A2OUT. value."]
    A2OUT = 3,
    #[doc = "2: Output is A2OUT2 value."]
    A2OUT2 = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG9_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG9` reader - Pad output 9 configuration"]
pub struct CFG9_R(crate::FieldReader<u8, CFG9_A>);
impl CFG9_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG9_A {
        match self.bits {
            7 => CFG9_A::A7OUT2,
            6 => CFG9_A::A6OUT2,
            5 => CFG9_A::B0OUT,
            4 => CFG9_A::A4OUT,
            3 => CFG9_A::A2OUT,
            2 => CFG9_A::A2OUT2,
            1 => CFG9_A::ONE,
            0 => CFG9_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG9_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG9_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline(always)]
    pub fn is_b0out(&self) -> bool {
        **self == CFG9_A::B0OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        **self == CFG9_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline(always)]
    pub fn is_a2out(&self) -> bool {
        **self == CFG9_A::A2OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline(always)]
    pub fn is_a2out2(&self) -> bool {
        **self == CFG9_A::A2OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG9_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG9_A::ZERO
    }
}
impl core::ops::Deref for CFG9_R {
    type Target = crate::FieldReader<u8, CFG9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG9` writer - Pad output 9 configuration"]
pub struct CFG9_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG9_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG9_A::A6OUT2)
    }
    #[doc = "Output is B0OUT. value."]
    #[inline(always)]
    pub fn b0out(self) -> &'a mut W {
        self.variant(CFG9_A::B0OUT)
    }
    #[doc = "Output is A4OUT. value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(CFG9_A::A4OUT)
    }
    #[doc = "Output is A2OUT. value."]
    #[inline(always)]
    pub fn a2out(self) -> &'a mut W {
        self.variant(CFG9_A::A2OUT)
    }
    #[doc = "Output is A2OUT2 value."]
    #[inline(always)]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(CFG9_A::A2OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG9_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG9_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Pad output 8 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG8_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is B6OUT. value."]
    B6OUT = 5,
    #[doc = "4: Output is A4OUT2. value."]
    A4OUT2 = 4,
    #[doc = "3: Output is A3OUT. value."]
    A3OUT2 = 3,
    #[doc = "2: Output is A2OUT value."]
    A2OUT = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG8_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG8` reader - Pad output 8 configuration"]
pub struct CFG8_R(crate::FieldReader<u8, CFG8_A>);
impl CFG8_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG8_A {
        match self.bits {
            7 => CFG8_A::A7OUT2,
            6 => CFG8_A::A6OUT2,
            5 => CFG8_A::B6OUT,
            4 => CFG8_A::A4OUT2,
            3 => CFG8_A::A3OUT2,
            2 => CFG8_A::A2OUT,
            1 => CFG8_A::ONE,
            0 => CFG8_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG8_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG8_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline(always)]
    pub fn is_b6out(&self) -> bool {
        **self == CFG8_A::B6OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT2`"]
    #[inline(always)]
    pub fn is_a4out2(&self) -> bool {
        **self == CFG8_A::A4OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        **self == CFG8_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline(always)]
    pub fn is_a2out(&self) -> bool {
        **self == CFG8_A::A2OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG8_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG8_A::ZERO
    }
}
impl core::ops::Deref for CFG8_R {
    type Target = crate::FieldReader<u8, CFG8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG8` writer - Pad output 8 configuration"]
pub struct CFG8_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG8_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG8_A::A6OUT2)
    }
    #[doc = "Output is B6OUT. value."]
    #[inline(always)]
    pub fn b6out(self) -> &'a mut W {
        self.variant(CFG8_A::B6OUT)
    }
    #[doc = "Output is A4OUT2. value."]
    #[inline(always)]
    pub fn a4out2(self) -> &'a mut W {
        self.variant(CFG8_A::A4OUT2)
    }
    #[doc = "Output is A3OUT. value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(CFG8_A::A3OUT2)
    }
    #[doc = "Output is A2OUT value."]
    #[inline(always)]
    pub fn a2out(self) -> &'a mut W {
        self.variant(CFG8_A::A2OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG8_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG8_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Pad output 7 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG7_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is A7OUT. value."]
    A7OUT = 5,
    #[doc = "4: Output is B5OUT. value."]
    B5OUT = 4,
    #[doc = "3: Output is B1OUT. value."]
    B1OUT = 3,
    #[doc = "2: Output is B1OUT2 value."]
    B1OUT2 = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG7_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG7` reader - Pad output 7 configuration"]
pub struct CFG7_R(crate::FieldReader<u8, CFG7_A>);
impl CFG7_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG7_A {
        match self.bits {
            7 => CFG7_A::A7OUT2,
            6 => CFG7_A::A6OUT2,
            5 => CFG7_A::A7OUT,
            4 => CFG7_A::B5OUT,
            3 => CFG7_A::B1OUT,
            2 => CFG7_A::B1OUT2,
            1 => CFG7_A::ONE,
            0 => CFG7_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG7_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG7_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline(always)]
    pub fn is_a7out(&self) -> bool {
        **self == CFG7_A::A7OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline(always)]
    pub fn is_b5out(&self) -> bool {
        **self == CFG7_A::B5OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline(always)]
    pub fn is_b1out(&self) -> bool {
        **self == CFG7_A::B1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline(always)]
    pub fn is_b1out2(&self) -> bool {
        **self == CFG7_A::B1OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG7_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG7_A::ZERO
    }
}
impl core::ops::Deref for CFG7_R {
    type Target = crate::FieldReader<u8, CFG7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG7` writer - Pad output 7 configuration"]
pub struct CFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG7_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG7_A::A6OUT2)
    }
    #[doc = "Output is A7OUT. value."]
    #[inline(always)]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG7_A::A7OUT)
    }
    #[doc = "Output is B5OUT. value."]
    #[inline(always)]
    pub fn b5out(self) -> &'a mut W {
        self.variant(CFG7_A::B5OUT)
    }
    #[doc = "Output is B1OUT. value."]
    #[inline(always)]
    pub fn b1out(self) -> &'a mut W {
        self.variant(CFG7_A::B1OUT)
    }
    #[doc = "Output is B1OUT2 value."]
    #[inline(always)]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(CFG7_A::B1OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG7_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG7_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Pad output 6 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG6_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is B7OUT. value."]
    B7OUT = 5,
    #[doc = "4: Output is B5OUT2. value."]
    B5OUT2 = 4,
    #[doc = "3: Output is A1OUT. value."]
    A1OUT = 3,
    #[doc = "2: Output is B1OUT value."]
    B1OUT = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG6_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG6` reader - Pad output 6 configuration"]
pub struct CFG6_R(crate::FieldReader<u8, CFG6_A>);
impl CFG6_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG6_A {
        match self.bits {
            7 => CFG6_A::A7OUT2,
            6 => CFG6_A::A6OUT2,
            5 => CFG6_A::B7OUT,
            4 => CFG6_A::B5OUT2,
            3 => CFG6_A::A1OUT,
            2 => CFG6_A::B1OUT,
            1 => CFG6_A::ONE,
            0 => CFG6_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG6_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG6_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline(always)]
    pub fn is_b7out(&self) -> bool {
        **self == CFG6_A::B7OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline(always)]
    pub fn is_b5out2(&self) -> bool {
        **self == CFG6_A::B5OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        **self == CFG6_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline(always)]
    pub fn is_b1out(&self) -> bool {
        **self == CFG6_A::B1OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG6_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG6_A::ZERO
    }
}
impl core::ops::Deref for CFG6_R {
    type Target = crate::FieldReader<u8, CFG6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG6` writer - Pad output 6 configuration"]
pub struct CFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG6_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG6_A::A6OUT2)
    }
    #[doc = "Output is B7OUT. value."]
    #[inline(always)]
    pub fn b7out(self) -> &'a mut W {
        self.variant(CFG6_A::B7OUT)
    }
    #[doc = "Output is B5OUT2. value."]
    #[inline(always)]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(CFG6_A::B5OUT2)
    }
    #[doc = "Output is A1OUT. value."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG6_A::A1OUT)
    }
    #[doc = "Output is B1OUT value."]
    #[inline(always)]
    pub fn b1out(self) -> &'a mut W {
        self.variant(CFG6_A::B1OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG6_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG6_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad output 5 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG5_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is A7OUT. value."]
    A7OUT = 5,
    #[doc = "4: Output is A5OUT. value."]
    B6OUT = 4,
    #[doc = "3: Output is A1OUT. value."]
    A1OUT = 3,
    #[doc = "2: Output is A1OUT2 value."]
    A1OUT2 = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG5_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG5` reader - Pad output 5 configuration"]
pub struct CFG5_R(crate::FieldReader<u8, CFG5_A>);
impl CFG5_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG5_A {
        match self.bits {
            7 => CFG5_A::A7OUT2,
            6 => CFG5_A::A6OUT2,
            5 => CFG5_A::A7OUT,
            4 => CFG5_A::B6OUT,
            3 => CFG5_A::A1OUT,
            2 => CFG5_A::A1OUT2,
            1 => CFG5_A::ONE,
            0 => CFG5_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG5_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG5_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline(always)]
    pub fn is_a7out(&self) -> bool {
        **self == CFG5_A::A7OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline(always)]
    pub fn is_b6out(&self) -> bool {
        **self == CFG5_A::B6OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        **self == CFG5_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline(always)]
    pub fn is_a1out2(&self) -> bool {
        **self == CFG5_A::A1OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG5_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG5_A::ZERO
    }
}
impl core::ops::Deref for CFG5_R {
    type Target = crate::FieldReader<u8, CFG5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG5` writer - Pad output 5 configuration"]
pub struct CFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG5_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG5_A::A6OUT2)
    }
    #[doc = "Output is A7OUT. value."]
    #[inline(always)]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG5_A::A7OUT)
    }
    #[doc = "Output is A5OUT. value."]
    #[inline(always)]
    pub fn b6out(self) -> &'a mut W {
        self.variant(CFG5_A::B6OUT)
    }
    #[doc = "Output is A1OUT. value."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG5_A::A1OUT)
    }
    #[doc = "Output is A1OUT2 value."]
    #[inline(always)]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(CFG5_A::A1OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG5_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG5_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Pad output 4 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG4_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is B5OUT. value."]
    B5OUT = 5,
    #[doc = "4: Output is A5OUT2. value."]
    A5OUT2 = 4,
    #[doc = "3: Output is A2OUT2. value."]
    A2OUT2 = 3,
    #[doc = "2: Output is A1OUT value."]
    A1OUT = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG4_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG4` reader - Pad output 4 configuration"]
pub struct CFG4_R(crate::FieldReader<u8, CFG4_A>);
impl CFG4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG4_A {
        match self.bits {
            7 => CFG4_A::A7OUT2,
            6 => CFG4_A::A6OUT2,
            5 => CFG4_A::B5OUT,
            4 => CFG4_A::A5OUT2,
            3 => CFG4_A::A2OUT2,
            2 => CFG4_A::A1OUT,
            1 => CFG4_A::ONE,
            0 => CFG4_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG4_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG4_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline(always)]
    pub fn is_b5out(&self) -> bool {
        **self == CFG4_A::B5OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline(always)]
    pub fn is_a5out2(&self) -> bool {
        **self == CFG4_A::A5OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline(always)]
    pub fn is_a2out2(&self) -> bool {
        **self == CFG4_A::A2OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        **self == CFG4_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG4_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG4_A::ZERO
    }
}
impl core::ops::Deref for CFG4_R {
    type Target = crate::FieldReader<u8, CFG4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG4` writer - Pad output 4 configuration"]
pub struct CFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG4_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG4_A::A6OUT2)
    }
    #[doc = "Output is B5OUT. value."]
    #[inline(always)]
    pub fn b5out(self) -> &'a mut W {
        self.variant(CFG4_A::B5OUT)
    }
    #[doc = "Output is A5OUT2. value."]
    #[inline(always)]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(CFG4_A::A5OUT2)
    }
    #[doc = "Output is A2OUT2. value."]
    #[inline(always)]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(CFG4_A::A2OUT2)
    }
    #[doc = "Output is A1OUT value."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG4_A::A1OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG4_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG4_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Pad output 3 configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG3_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is A6OUT. value."]
    A6OUT = 5,
    #[doc = "4: Output is A1OUT. value."]
    A1OUT = 4,
    #[doc = "3: Output is B0OUT. value."]
    B0OUT = 3,
    #[doc = "2: Output is B0OUT2 value."]
    B0OUT2 = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG3_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG3` reader - Pad output 3 configuration"]
pub struct CFG3_R(crate::FieldReader<u8, CFG3_A>);
impl CFG3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG3_A {
        match self.bits {
            7 => CFG3_A::A7OUT2,
            6 => CFG3_A::A6OUT2,
            5 => CFG3_A::A6OUT,
            4 => CFG3_A::A1OUT,
            3 => CFG3_A::B0OUT,
            2 => CFG3_A::B0OUT2,
            1 => CFG3_A::ONE,
            0 => CFG3_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG3_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG3_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        **self == CFG3_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        **self == CFG3_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline(always)]
    pub fn is_b0out(&self) -> bool {
        **self == CFG3_A::B0OUT
    }
    #[doc = "Checks if the value of the field is `B0OUT2`"]
    #[inline(always)]
    pub fn is_b0out2(&self) -> bool {
        **self == CFG3_A::B0OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG3_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG3_A::ZERO
    }
}
impl core::ops::Deref for CFG3_R {
    type Target = crate::FieldReader<u8, CFG3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG3` writer - Pad output 3 configuration"]
pub struct CFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG3_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG3_A::A6OUT2)
    }
    #[doc = "Output is A6OUT. value."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG3_A::A6OUT)
    }
    #[doc = "Output is A1OUT. value."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG3_A::A1OUT)
    }
    #[doc = "Output is B0OUT. value."]
    #[inline(always)]
    pub fn b0out(self) -> &'a mut W {
        self.variant(CFG3_A::B0OUT)
    }
    #[doc = "Output is B0OUT2 value."]
    #[inline(always)]
    pub fn b0out2(self) -> &'a mut W {
        self.variant(CFG3_A::B0OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG3_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG3_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Pad output 2 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG2_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is A7OUT. value."]
    A7OUT = 5,
    #[doc = "4: Output is B6OUT2. value."]
    B6OUT2 = 4,
    #[doc = "3: Output is B1OUT2. value."]
    B1OUT2 = 3,
    #[doc = "2: Output is B0OUT value."]
    B0OUT = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG2_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG2` reader - Pad output 2 configuration"]
pub struct CFG2_R(crate::FieldReader<u8, CFG2_A>);
impl CFG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG2_A {
        match self.bits {
            7 => CFG2_A::A7OUT2,
            6 => CFG2_A::A6OUT2,
            5 => CFG2_A::A7OUT,
            4 => CFG2_A::B6OUT2,
            3 => CFG2_A::B1OUT2,
            2 => CFG2_A::B0OUT,
            1 => CFG2_A::ONE,
            0 => CFG2_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG2_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG2_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline(always)]
    pub fn is_a7out(&self) -> bool {
        **self == CFG2_A::A7OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT2`"]
    #[inline(always)]
    pub fn is_b6out2(&self) -> bool {
        **self == CFG2_A::B6OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline(always)]
    pub fn is_b1out2(&self) -> bool {
        **self == CFG2_A::B1OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline(always)]
    pub fn is_b0out(&self) -> bool {
        **self == CFG2_A::B0OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG2_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG2_A::ZERO
    }
}
impl core::ops::Deref for CFG2_R {
    type Target = crate::FieldReader<u8, CFG2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG2` writer - Pad output 2 configuration"]
pub struct CFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG2_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG2_A::A6OUT2)
    }
    #[doc = "Output is A7OUT. value."]
    #[inline(always)]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG2_A::A7OUT)
    }
    #[doc = "Output is B6OUT2. value."]
    #[inline(always)]
    pub fn b6out2(self) -> &'a mut W {
        self.variant(CFG2_A::B6OUT2)
    }
    #[doc = "Output is B1OUT2. value."]
    #[inline(always)]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(CFG2_A::B1OUT2)
    }
    #[doc = "Output is B0OUT value."]
    #[inline(always)]
    pub fn b0out(self) -> &'a mut W {
        self.variant(CFG2_A::B0OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG2_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG2_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Pad output 1 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG1_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is B7OUT2. value."]
    B7OUT2 = 5,
    #[doc = "4: Output is A5OUT. value."]
    A5OUT = 4,
    #[doc = "3: Output is A0OUT. value."]
    A0OUT = 3,
    #[doc = "2: Output is A0OUT2 value."]
    A0OUT2 = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG1_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG1` reader - Pad output 1 configuration"]
pub struct CFG1_R(crate::FieldReader<u8, CFG1_A>);
impl CFG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG1_A {
        match self.bits {
            7 => CFG1_A::A7OUT2,
            6 => CFG1_A::A6OUT2,
            5 => CFG1_A::B7OUT2,
            4 => CFG1_A::A5OUT,
            3 => CFG1_A::A0OUT,
            2 => CFG1_A::A0OUT2,
            1 => CFG1_A::ONE,
            0 => CFG1_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG1_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG1_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B7OUT2`"]
    #[inline(always)]
    pub fn is_b7out2(&self) -> bool {
        **self == CFG1_A::B7OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline(always)]
    pub fn is_a5out(&self) -> bool {
        **self == CFG1_A::A5OUT
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline(always)]
    pub fn is_a0out(&self) -> bool {
        **self == CFG1_A::A0OUT
    }
    #[doc = "Checks if the value of the field is `A0OUT2`"]
    #[inline(always)]
    pub fn is_a0out2(&self) -> bool {
        **self == CFG1_A::A0OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG1_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG1_A::ZERO
    }
}
impl core::ops::Deref for CFG1_R {
    type Target = crate::FieldReader<u8, CFG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG1` writer - Pad output 1 configuration"]
pub struct CFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG1_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG1_A::A6OUT2)
    }
    #[doc = "Output is B7OUT2. value."]
    #[inline(always)]
    pub fn b7out2(self) -> &'a mut W {
        self.variant(CFG1_A::B7OUT2)
    }
    #[doc = "Output is A5OUT. value."]
    #[inline(always)]
    pub fn a5out(self) -> &'a mut W {
        self.variant(CFG1_A::A5OUT)
    }
    #[doc = "Output is A0OUT. value."]
    #[inline(always)]
    pub fn a0out(self) -> &'a mut W {
        self.variant(CFG1_A::A0OUT)
    }
    #[doc = "Output is A0OUT2 value."]
    #[inline(always)]
    pub fn a0out2(self) -> &'a mut W {
        self.variant(CFG1_A::A0OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG1_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG1_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad output 0 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG0_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is A6OUT. value."]
    A6OUT = 5,
    #[doc = "4: Output is A5OUT2. value."]
    A5OUT2 = 4,
    #[doc = "3: Output is B2OUT2. value."]
    B2OUT2 = 3,
    #[doc = "2: Output is A0OUT value."]
    A0OUT = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG0_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG0` reader - Pad output 0 configuration"]
pub struct CFG0_R(crate::FieldReader<u8, CFG0_A>);
impl CFG0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG0_A {
        match self.bits {
            7 => CFG0_A::A7OUT2,
            6 => CFG0_A::A6OUT2,
            5 => CFG0_A::A6OUT,
            4 => CFG0_A::A5OUT2,
            3 => CFG0_A::B2OUT2,
            2 => CFG0_A::A0OUT,
            1 => CFG0_A::ONE,
            0 => CFG0_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG0_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG0_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        **self == CFG0_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline(always)]
    pub fn is_a5out2(&self) -> bool {
        **self == CFG0_A::A5OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline(always)]
    pub fn is_b2out2(&self) -> bool {
        **self == CFG0_A::B2OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline(always)]
    pub fn is_a0out(&self) -> bool {
        **self == CFG0_A::A0OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG0_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG0_A::ZERO
    }
}
impl core::ops::Deref for CFG0_R {
    type Target = crate::FieldReader<u8, CFG0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG0` writer - Pad output 0 configuration"]
pub struct CFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG0_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG0_A::A6OUT2)
    }
    #[doc = "Output is A6OUT. value."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG0_A::A6OUT)
    }
    #[doc = "Output is A5OUT2. value."]
    #[inline(always)]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(CFG0_A::A5OUT2)
    }
    #[doc = "Output is B2OUT2. value."]
    #[inline(always)]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(CFG0_A::B2OUT2)
    }
    #[doc = "Output is A0OUT value."]
    #[inline(always)]
    pub fn a0out(self) -> &'a mut W {
        self.variant(CFG0_A::A0OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG0_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG0_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30 - Pad output 9 configuration"]
    #[inline(always)]
    pub fn cfg9(&self) -> CFG9_R {
        CFG9_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 25:27 - Pad output 8 configuration"]
    #[inline(always)]
    pub fn cfg8(&self) -> CFG8_R {
        CFG8_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 22:24 - Pad output 7 configuration"]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - Pad output 6 configuration"]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Pad output 5 configuration"]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Pad output 4 configuration"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Pad output 3 configuration"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - Pad output 2 configuration"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Pad output 1 configuration"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Pad output 0 configuration"]
    #[inline(always)]
    pub fn cfg0(&self) -> CFG0_R {
        CFG0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30 - Pad output 9 configuration"]
    #[inline(always)]
    pub fn cfg9(&mut self) -> CFG9_W {
        CFG9_W { w: self }
    }
    #[doc = "Bits 25:27 - Pad output 8 configuration"]
    #[inline(always)]
    pub fn cfg8(&mut self) -> CFG8_W {
        CFG8_W { w: self }
    }
    #[doc = "Bits 22:24 - Pad output 7 configuration"]
    #[inline(always)]
    pub fn cfg7(&mut self) -> CFG7_W {
        CFG7_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad output 6 configuration"]
    #[inline(always)]
    pub fn cfg6(&mut self) -> CFG6_W {
        CFG6_W { w: self }
    }
    #[doc = "Bits 16:18 - Pad output 5 configuration"]
    #[inline(always)]
    pub fn cfg5(&mut self) -> CFG5_W {
        CFG5_W { w: self }
    }
    #[doc = "Bits 12:14 - Pad output 4 configuration"]
    #[inline(always)]
    pub fn cfg4(&mut self) -> CFG4_W {
        CFG4_W { w: self }
    }
    #[doc = "Bits 9:11 - Pad output 3 configuration"]
    #[inline(always)]
    pub fn cfg3(&mut self) -> CFG3_W {
        CFG3_W { w: self }
    }
    #[doc = "Bits 6:8 - Pad output 2 configuration"]
    #[inline(always)]
    pub fn cfg2(&mut self) -> CFG2_W {
        CFG2_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad output 1 configuration"]
    #[inline(always)]
    pub fn cfg1(&mut self) -> CFG1_W {
        CFG1_W { w: self }
    }
    #[doc = "Bits 0:2 - Pad output 0 configuration"]
    #[inline(always)]
    pub fn cfg0(&mut self) -> CFG0_W {
        CFG0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Output Config 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outcfg0](index.html) module"]
pub struct OUTCFG0_SPEC;
impl crate::RegisterSpec for OUTCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outcfg0::R](R) reader structure"]
impl crate::Readable for OUTCFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outcfg0::W](W) writer structure"]
impl crate::Writable for OUTCFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTCFG0 to value 0x2492_2292"]
impl crate::Resettable for OUTCFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2492_2292
    }
}
