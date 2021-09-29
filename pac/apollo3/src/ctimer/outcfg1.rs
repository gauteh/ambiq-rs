#[doc = "Register `OUTCFG1` reader"]
pub struct R(crate::R<OUTCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTCFG1` writer"]
pub struct W(crate::W<OUTCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTCFG1_SPEC>;
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
impl From<crate::W<OUTCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad output 19 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG19_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is B1OUT2. value."]
    B1OUT2 = 5,
    #[doc = "4: Output is B4OUT. value."]
    B4OUT = 4,
    #[doc = "3: Output is A2OUT. value."]
    A2OUT = 3,
    #[doc = "2: Output is B4OUT2 value."]
    B4OUT2 = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG19_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG19_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG19` reader - Pad output 19 configuration"]
pub struct CFG19_R(crate::FieldReader<u8, CFG19_A>);
impl CFG19_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG19_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG19_A {
        match self.bits {
            7 => CFG19_A::A7OUT2,
            6 => CFG19_A::A6OUT2,
            5 => CFG19_A::B1OUT2,
            4 => CFG19_A::B4OUT,
            3 => CFG19_A::A2OUT,
            2 => CFG19_A::B4OUT2,
            1 => CFG19_A::ONE,
            0 => CFG19_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG19_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG19_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline(always)]
    pub fn is_b1out2(&self) -> bool {
        **self == CFG19_A::B1OUT2
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        **self == CFG19_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline(always)]
    pub fn is_a2out(&self) -> bool {
        **self == CFG19_A::A2OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT2`"]
    #[inline(always)]
    pub fn is_b4out2(&self) -> bool {
        **self == CFG19_A::B4OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG19_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG19_A::ZERO
    }
}
impl core::ops::Deref for CFG19_R {
    type Target = crate::FieldReader<u8, CFG19_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG19` writer - Pad output 19 configuration"]
pub struct CFG19_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG19_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG19_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG19_A::A6OUT2)
    }
    #[doc = "Output is B1OUT2. value."]
    #[inline(always)]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(CFG19_A::B1OUT2)
    }
    #[doc = "Output is B4OUT. value."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(CFG19_A::B4OUT)
    }
    #[doc = "Output is A2OUT. value."]
    #[inline(always)]
    pub fn a2out(self) -> &'a mut W {
        self.variant(CFG19_A::A2OUT)
    }
    #[doc = "Output is B4OUT2 value."]
    #[inline(always)]
    pub fn b4out2(self) -> &'a mut W {
        self.variant(CFG19_A::B4OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG19_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG19_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Pad output 18 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG18_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is A3OUT2. value."]
    A3OUT2 = 5,
    #[doc = "4: Output is A0OUT. value."]
    A0OUT = 4,
    #[doc = "3: Output is B0OUT. value."]
    B0OUT = 3,
    #[doc = "2: Output is B4OUT value."]
    B4OUT = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG18_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG18_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG18` reader - Pad output 18 configuration"]
pub struct CFG18_R(crate::FieldReader<u8, CFG18_A>);
impl CFG18_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG18_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG18_A {
        match self.bits {
            7 => CFG18_A::A7OUT2,
            6 => CFG18_A::A6OUT2,
            5 => CFG18_A::A3OUT2,
            4 => CFG18_A::A0OUT,
            3 => CFG18_A::B0OUT,
            2 => CFG18_A::B4OUT,
            1 => CFG18_A::ONE,
            0 => CFG18_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG18_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG18_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        **self == CFG18_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline(always)]
    pub fn is_a0out(&self) -> bool {
        **self == CFG18_A::A0OUT
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline(always)]
    pub fn is_b0out(&self) -> bool {
        **self == CFG18_A::B0OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        **self == CFG18_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG18_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG18_A::ZERO
    }
}
impl core::ops::Deref for CFG18_R {
    type Target = crate::FieldReader<u8, CFG18_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG18` writer - Pad output 18 configuration"]
pub struct CFG18_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG18_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG18_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG18_A::A6OUT2)
    }
    #[doc = "Output is A3OUT2. value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(CFG18_A::A3OUT2)
    }
    #[doc = "Output is A0OUT. value."]
    #[inline(always)]
    pub fn a0out(self) -> &'a mut W {
        self.variant(CFG18_A::A0OUT)
    }
    #[doc = "Output is B0OUT. value."]
    #[inline(always)]
    pub fn b0out(self) -> &'a mut W {
        self.variant(CFG18_A::B0OUT)
    }
    #[doc = "Output is B4OUT value."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(CFG18_A::B4OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG18_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG18_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Pad output 17 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG17_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is A1OUT2. value."]
    A1OUT2 = 5,
    #[doc = "4: Output is A4OUT. value."]
    A4OUT = 4,
    #[doc = "3: Output is B7OUT. value."]
    B7OUT = 3,
    #[doc = "2: Output is A4OUT2 value."]
    A4OUT2 = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG17_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG17_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG17` reader - Pad output 17 configuration"]
pub struct CFG17_R(crate::FieldReader<u8, CFG17_A>);
impl CFG17_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG17_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG17_A {
        match self.bits {
            7 => CFG17_A::A7OUT2,
            6 => CFG17_A::A6OUT2,
            5 => CFG17_A::A1OUT2,
            4 => CFG17_A::A4OUT,
            3 => CFG17_A::B7OUT,
            2 => CFG17_A::A4OUT2,
            1 => CFG17_A::ONE,
            0 => CFG17_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG17_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG17_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline(always)]
    pub fn is_a1out2(&self) -> bool {
        **self == CFG17_A::A1OUT2
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        **self == CFG17_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline(always)]
    pub fn is_b7out(&self) -> bool {
        **self == CFG17_A::B7OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT2`"]
    #[inline(always)]
    pub fn is_a4out2(&self) -> bool {
        **self == CFG17_A::A4OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG17_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG17_A::ZERO
    }
}
impl core::ops::Deref for CFG17_R {
    type Target = crate::FieldReader<u8, CFG17_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG17` writer - Pad output 17 configuration"]
pub struct CFG17_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG17_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG17_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG17_A::A6OUT2)
    }
    #[doc = "Output is A1OUT2. value."]
    #[inline(always)]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(CFG17_A::A1OUT2)
    }
    #[doc = "Output is A4OUT. value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(CFG17_A::A4OUT)
    }
    #[doc = "Output is B7OUT. value."]
    #[inline(always)]
    pub fn b7out(self) -> &'a mut W {
        self.variant(CFG17_A::B7OUT)
    }
    #[doc = "Output is A4OUT2 value."]
    #[inline(always)]
    pub fn a4out2(self) -> &'a mut W {
        self.variant(CFG17_A::A4OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG17_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG17_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Pad output 16 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG16_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is B3OUT2. value."]
    B3OUT2 = 5,
    #[doc = "4: Output is A0OUT2. value."]
    A0OUT2 = 4,
    #[doc = "3: Output is A0OUT. value."]
    A0OUT = 3,
    #[doc = "2: Output is A4OUT value."]
    A4OUT = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG16_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG16_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG16` reader - Pad output 16 configuration"]
pub struct CFG16_R(crate::FieldReader<u8, CFG16_A>);
impl CFG16_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG16_A {
        match self.bits {
            7 => CFG16_A::A7OUT2,
            6 => CFG16_A::A6OUT2,
            5 => CFG16_A::B3OUT2,
            4 => CFG16_A::A0OUT2,
            3 => CFG16_A::A0OUT,
            2 => CFG16_A::A4OUT,
            1 => CFG16_A::ONE,
            0 => CFG16_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG16_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG16_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        **self == CFG16_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT2`"]
    #[inline(always)]
    pub fn is_a0out2(&self) -> bool {
        **self == CFG16_A::A0OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline(always)]
    pub fn is_a0out(&self) -> bool {
        **self == CFG16_A::A0OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        **self == CFG16_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG16_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG16_A::ZERO
    }
}
impl core::ops::Deref for CFG16_R {
    type Target = crate::FieldReader<u8, CFG16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG16` writer - Pad output 16 configuration"]
pub struct CFG16_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG16_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG16_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG16_A::A6OUT2)
    }
    #[doc = "Output is B3OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(CFG16_A::B3OUT2)
    }
    #[doc = "Output is A0OUT2. value."]
    #[inline(always)]
    pub fn a0out2(self) -> &'a mut W {
        self.variant(CFG16_A::A0OUT2)
    }
    #[doc = "Output is A0OUT. value."]
    #[inline(always)]
    pub fn a0out(self) -> &'a mut W {
        self.variant(CFG16_A::A0OUT)
    }
    #[doc = "Output is A4OUT value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(CFG16_A::A4OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG16_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG16_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad output 15 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG15_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is A4OUT2. value."]
    A4OUT2 = 5,
    #[doc = "4: Output is A7OUT. value."]
    A7OUT = 4,
    #[doc = "3: Output is B3OUT. value."]
    B3OUT = 3,
    #[doc = "2: Output is B3OUT2 value."]
    B3OUT2 = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG15_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG15_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG15` reader - Pad output 15 configuration"]
pub struct CFG15_R(crate::FieldReader<u8, CFG15_A>);
impl CFG15_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG15_A {
        match self.bits {
            7 => CFG15_A::A7OUT2,
            6 => CFG15_A::A6OUT2,
            5 => CFG15_A::A4OUT2,
            4 => CFG15_A::A7OUT,
            3 => CFG15_A::B3OUT,
            2 => CFG15_A::B3OUT2,
            1 => CFG15_A::ONE,
            0 => CFG15_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG15_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG15_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A4OUT2`"]
    #[inline(always)]
    pub fn is_a4out2(&self) -> bool {
        **self == CFG15_A::A4OUT2
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline(always)]
    pub fn is_a7out(&self) -> bool {
        **self == CFG15_A::A7OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        **self == CFG15_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        **self == CFG15_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG15_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG15_A::ZERO
    }
}
impl core::ops::Deref for CFG15_R {
    type Target = crate::FieldReader<u8, CFG15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG15` writer - Pad output 15 configuration"]
pub struct CFG15_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG15_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG15_A::A6OUT2)
    }
    #[doc = "Output is A4OUT2. value."]
    #[inline(always)]
    pub fn a4out2(self) -> &'a mut W {
        self.variant(CFG15_A::A4OUT2)
    }
    #[doc = "Output is A7OUT. value."]
    #[inline(always)]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG15_A::A7OUT)
    }
    #[doc = "Output is B3OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(CFG15_A::B3OUT)
    }
    #[doc = "Output is B3OUT2 value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(CFG15_A::B3OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG15_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG15_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Pad output 14 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG14_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is A7OUT. value."]
    A7OUT = 5,
    #[doc = "4: Output is B7OUT2. value."]
    B7OUT2 = 4,
    #[doc = "3: Output is B1OUT. value."]
    B1OUT = 3,
    #[doc = "2: Output is B3OUT value."]
    B3OUT = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG14_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG14_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG14` reader - Pad output 14 configuration"]
pub struct CFG14_R(crate::FieldReader<u8, CFG14_A>);
impl CFG14_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG14_A {
        match self.bits {
            7 => CFG14_A::A7OUT2,
            6 => CFG14_A::A6OUT2,
            5 => CFG14_A::A7OUT,
            4 => CFG14_A::B7OUT2,
            3 => CFG14_A::B1OUT,
            2 => CFG14_A::B3OUT,
            1 => CFG14_A::ONE,
            0 => CFG14_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG14_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG14_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline(always)]
    pub fn is_a7out(&self) -> bool {
        **self == CFG14_A::A7OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT2`"]
    #[inline(always)]
    pub fn is_b7out2(&self) -> bool {
        **self == CFG14_A::B7OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline(always)]
    pub fn is_b1out(&self) -> bool {
        **self == CFG14_A::B1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        **self == CFG14_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG14_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG14_A::ZERO
    }
}
impl core::ops::Deref for CFG14_R {
    type Target = crate::FieldReader<u8, CFG14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG14` writer - Pad output 14 configuration"]
pub struct CFG14_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG14_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG14_A::A6OUT2)
    }
    #[doc = "Output is A7OUT. value."]
    #[inline(always)]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG14_A::A7OUT)
    }
    #[doc = "Output is B7OUT2. value."]
    #[inline(always)]
    pub fn b7out2(self) -> &'a mut W {
        self.variant(CFG14_A::B7OUT2)
    }
    #[doc = "Output is B1OUT. value."]
    #[inline(always)]
    pub fn b1out(self) -> &'a mut W {
        self.variant(CFG14_A::B1OUT)
    }
    #[doc = "Output is B3OUT value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(CFG14_A::B3OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG14_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG14_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Pad output 13 configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG13_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is B4OUT2. value."]
    B4OUT2 = 5,
    #[doc = "4: Output is A6OUT. value."]
    A6OUT = 4,
    #[doc = "3: Output is A3OUT. value."]
    A3OUT = 3,
    #[doc = "2: Output is A3OUT2 value."]
    A3OUT2 = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG13_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG13_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG13` reader - Pad output 13 configuration"]
pub struct CFG13_R(crate::FieldReader<u8, CFG13_A>);
impl CFG13_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG13_A {
        match self.bits {
            7 => CFG13_A::A7OUT2,
            6 => CFG13_A::A6OUT2,
            5 => CFG13_A::B4OUT2,
            4 => CFG13_A::A6OUT,
            3 => CFG13_A::A3OUT,
            2 => CFG13_A::A3OUT2,
            1 => CFG13_A::ONE,
            0 => CFG13_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG13_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG13_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B4OUT2`"]
    #[inline(always)]
    pub fn is_b4out2(&self) -> bool {
        **self == CFG13_A::B4OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        **self == CFG13_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        **self == CFG13_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        **self == CFG13_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG13_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG13_A::ZERO
    }
}
impl core::ops::Deref for CFG13_R {
    type Target = crate::FieldReader<u8, CFG13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG13` writer - Pad output 13 configuration"]
pub struct CFG13_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG13_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG13_A::A6OUT2)
    }
    #[doc = "Output is B4OUT2. value."]
    #[inline(always)]
    pub fn b4out2(self) -> &'a mut W {
        self.variant(CFG13_A::B4OUT2)
    }
    #[doc = "Output is A6OUT. value."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG13_A::A6OUT)
    }
    #[doc = "Output is A3OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(CFG13_A::A3OUT)
    }
    #[doc = "Output is A3OUT2 value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(CFG13_A::A3OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG13_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG13_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Pad output 12 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG12_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is B6OUT2. value."]
    B6OUT2 = 5,
    #[doc = "4: Output is B0OUT2. value."]
    B0OUT2 = 4,
    #[doc = "3: Output is B1OUT. value."]
    B1OUT = 3,
    #[doc = "2: Output is A3OUT value."]
    A3OUT = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG12_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG12` reader - Pad output 12 configuration"]
pub struct CFG12_R(crate::FieldReader<u8, CFG12_A>);
impl CFG12_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG12_A {
        match self.bits {
            7 => CFG12_A::A7OUT2,
            6 => CFG12_A::A6OUT2,
            5 => CFG12_A::B6OUT2,
            4 => CFG12_A::B0OUT2,
            3 => CFG12_A::B1OUT,
            2 => CFG12_A::A3OUT,
            1 => CFG12_A::ONE,
            0 => CFG12_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG12_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG12_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B6OUT2`"]
    #[inline(always)]
    pub fn is_b6out2(&self) -> bool {
        **self == CFG12_A::B6OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT2`"]
    #[inline(always)]
    pub fn is_b0out2(&self) -> bool {
        **self == CFG12_A::B0OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline(always)]
    pub fn is_b1out(&self) -> bool {
        **self == CFG12_A::B1OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        **self == CFG12_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG12_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG12_A::ZERO
    }
}
impl core::ops::Deref for CFG12_R {
    type Target = crate::FieldReader<u8, CFG12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG12` writer - Pad output 12 configuration"]
pub struct CFG12_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG12_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG12_A::A6OUT2)
    }
    #[doc = "Output is B6OUT2. value."]
    #[inline(always)]
    pub fn b6out2(self) -> &'a mut W {
        self.variant(CFG12_A::B6OUT2)
    }
    #[doc = "Output is B0OUT2. value."]
    #[inline(always)]
    pub fn b0out2(self) -> &'a mut W {
        self.variant(CFG12_A::B0OUT2)
    }
    #[doc = "Output is B1OUT. value."]
    #[inline(always)]
    pub fn b1out(self) -> &'a mut W {
        self.variant(CFG12_A::B1OUT)
    }
    #[doc = "Output is A3OUT value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(CFG12_A::A3OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG12_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG12_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Pad output 11 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG11_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is B5OUT2. value."]
    B5OUT2 = 5,
    #[doc = "4: Output is B4OUT. value."]
    B4OUT = 4,
    #[doc = "3: Output is B2OUT. value."]
    B2OUT = 3,
    #[doc = "2: Output is B2OUT2 value."]
    B2OUT2 = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG11_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG11` reader - Pad output 11 configuration"]
pub struct CFG11_R(crate::FieldReader<u8, CFG11_A>);
impl CFG11_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG11_A {
        match self.bits {
            7 => CFG11_A::A7OUT2,
            6 => CFG11_A::A6OUT2,
            5 => CFG11_A::B5OUT2,
            4 => CFG11_A::B4OUT,
            3 => CFG11_A::B2OUT,
            2 => CFG11_A::B2OUT2,
            1 => CFG11_A::ONE,
            0 => CFG11_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG11_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG11_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline(always)]
    pub fn is_b5out2(&self) -> bool {
        **self == CFG11_A::B5OUT2
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        **self == CFG11_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        **self == CFG11_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline(always)]
    pub fn is_b2out2(&self) -> bool {
        **self == CFG11_A::B2OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG11_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG11_A::ZERO
    }
}
impl core::ops::Deref for CFG11_R {
    type Target = crate::FieldReader<u8, CFG11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG11` writer - Pad output 11 configuration"]
pub struct CFG11_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG11_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG11_A::A6OUT2)
    }
    #[doc = "Output is B5OUT2. value."]
    #[inline(always)]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(CFG11_A::B5OUT2)
    }
    #[doc = "Output is B4OUT. value."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(CFG11_A::B4OUT)
    }
    #[doc = "Output is B2OUT. value."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(CFG11_A::B2OUT)
    }
    #[doc = "Output is B2OUT2 value."]
    #[inline(always)]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(CFG11_A::B2OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG11_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG11_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad output 10 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG10_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is A6OUT. value."]
    A6OUT = 5,
    #[doc = "4: Output is B4OUT2. value."]
    B4OUT2 = 4,
    #[doc = "3: Output is B3OUT2. value."]
    B3OUT2 = 3,
    #[doc = "2: Output is B2OUT value."]
    B2OUT = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG10_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG10` reader - Pad output 10 configuration"]
pub struct CFG10_R(crate::FieldReader<u8, CFG10_A>);
impl CFG10_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG10_A {
        match self.bits {
            7 => CFG10_A::A7OUT2,
            6 => CFG10_A::A6OUT2,
            5 => CFG10_A::A6OUT,
            4 => CFG10_A::B4OUT2,
            3 => CFG10_A::B3OUT2,
            2 => CFG10_A::B2OUT,
            1 => CFG10_A::ONE,
            0 => CFG10_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG10_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG10_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        **self == CFG10_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT2`"]
    #[inline(always)]
    pub fn is_b4out2(&self) -> bool {
        **self == CFG10_A::B4OUT2
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        **self == CFG10_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        **self == CFG10_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG10_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG10_A::ZERO
    }
}
impl core::ops::Deref for CFG10_R {
    type Target = crate::FieldReader<u8, CFG10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG10` writer - Pad output 10 configuration"]
pub struct CFG10_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG10_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG10_A::A6OUT2)
    }
    #[doc = "Output is A6OUT. value."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG10_A::A6OUT)
    }
    #[doc = "Output is B4OUT2. value."]
    #[inline(always)]
    pub fn b4out2(self) -> &'a mut W {
        self.variant(CFG10_A::B4OUT2)
    }
    #[doc = "Output is B3OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(CFG10_A::B3OUT2)
    }
    #[doc = "Output is B2OUT value."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(CFG10_A::B2OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG10_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG10_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30 - Pad output 19 configuration"]
    #[inline(always)]
    pub fn cfg19(&self) -> CFG19_R {
        CFG19_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 25:27 - Pad output 18 configuration"]
    #[inline(always)]
    pub fn cfg18(&self) -> CFG18_R {
        CFG18_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 22:24 - Pad output 17 configuration"]
    #[inline(always)]
    pub fn cfg17(&self) -> CFG17_R {
        CFG17_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - Pad output 16 configuration"]
    #[inline(always)]
    pub fn cfg16(&self) -> CFG16_R {
        CFG16_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Pad output 15 configuration"]
    #[inline(always)]
    pub fn cfg15(&self) -> CFG15_R {
        CFG15_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Pad output 14 configuration"]
    #[inline(always)]
    pub fn cfg14(&self) -> CFG14_R {
        CFG14_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Pad output 13 configuration"]
    #[inline(always)]
    pub fn cfg13(&self) -> CFG13_R {
        CFG13_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - Pad output 12 configuration"]
    #[inline(always)]
    pub fn cfg12(&self) -> CFG12_R {
        CFG12_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Pad output 11 configuration"]
    #[inline(always)]
    pub fn cfg11(&self) -> CFG11_R {
        CFG11_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Pad output 10 configuration"]
    #[inline(always)]
    pub fn cfg10(&self) -> CFG10_R {
        CFG10_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30 - Pad output 19 configuration"]
    #[inline(always)]
    pub fn cfg19(&mut self) -> CFG19_W {
        CFG19_W { w: self }
    }
    #[doc = "Bits 25:27 - Pad output 18 configuration"]
    #[inline(always)]
    pub fn cfg18(&mut self) -> CFG18_W {
        CFG18_W { w: self }
    }
    #[doc = "Bits 22:24 - Pad output 17 configuration"]
    #[inline(always)]
    pub fn cfg17(&mut self) -> CFG17_W {
        CFG17_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad output 16 configuration"]
    #[inline(always)]
    pub fn cfg16(&mut self) -> CFG16_W {
        CFG16_W { w: self }
    }
    #[doc = "Bits 16:18 - Pad output 15 configuration"]
    #[inline(always)]
    pub fn cfg15(&mut self) -> CFG15_W {
        CFG15_W { w: self }
    }
    #[doc = "Bits 12:14 - Pad output 14 configuration"]
    #[inline(always)]
    pub fn cfg14(&mut self) -> CFG14_W {
        CFG14_W { w: self }
    }
    #[doc = "Bits 9:11 - Pad output 13 configuration"]
    #[inline(always)]
    pub fn cfg13(&mut self) -> CFG13_W {
        CFG13_W { w: self }
    }
    #[doc = "Bits 6:8 - Pad output 12 configuration"]
    #[inline(always)]
    pub fn cfg12(&mut self) -> CFG12_W {
        CFG12_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad output 11 configuration"]
    #[inline(always)]
    pub fn cfg11(&mut self) -> CFG11_W {
        CFG11_W { w: self }
    }
    #[doc = "Bits 0:2 - Pad output 10 configuration"]
    #[inline(always)]
    pub fn cfg10(&mut self) -> CFG10_W {
        CFG10_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Output Config 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outcfg1](index.html) module"]
pub struct OUTCFG1_SPEC;
impl crate::RegisterSpec for OUTCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outcfg1::R](R) reader structure"]
impl crate::Readable for OUTCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outcfg1::W](W) writer structure"]
impl crate::Writable for OUTCFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTCFG1 to value 0x2492_2292"]
impl crate::Resettable for OUTCFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2492_2292
    }
}
