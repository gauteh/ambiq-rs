#[doc = "Register `OUTCFG3` reader"]
pub struct R(crate::R<OUTCFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTCFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTCFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTCFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTCFG3` writer"]
pub struct W(crate::W<OUTCFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTCFG3_SPEC>;
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
impl From<crate::W<OUTCFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTCFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad output 31 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG31_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is B3OUT2. value."]
    B3OUT2 = 5,
    #[doc = "4: Output is B7OUT. value."]
    B7OUT = 4,
    #[doc = "3: Output is A6OUT. value."]
    A6OUT = 3,
    #[doc = "2: Output is B7OUT2 value."]
    B7OUT2 = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG31_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG31_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG31` reader - Pad output 31 configuration"]
pub struct CFG31_R(crate::FieldReader<u8, CFG31_A>);
impl CFG31_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG31_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG31_A {
        match self.bits {
            7 => CFG31_A::A7OUT2,
            6 => CFG31_A::A6OUT2,
            5 => CFG31_A::B3OUT2,
            4 => CFG31_A::B7OUT,
            3 => CFG31_A::A6OUT,
            2 => CFG31_A::B7OUT2,
            1 => CFG31_A::ONE,
            0 => CFG31_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG31_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG31_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        **self == CFG31_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline(always)]
    pub fn is_b7out(&self) -> bool {
        **self == CFG31_A::B7OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        **self == CFG31_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT2`"]
    #[inline(always)]
    pub fn is_b7out2(&self) -> bool {
        **self == CFG31_A::B7OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG31_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG31_A::ZERO
    }
}
impl core::ops::Deref for CFG31_R {
    type Target = crate::FieldReader<u8, CFG31_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG31` writer - Pad output 31 configuration"]
pub struct CFG31_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG31_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG31_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG31_A::A6OUT2)
    }
    #[doc = "Output is B3OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(CFG31_A::B3OUT2)
    }
    #[doc = "Output is B7OUT. value."]
    #[inline(always)]
    pub fn b7out(self) -> &'a mut W {
        self.variant(CFG31_A::B7OUT)
    }
    #[doc = "Output is A6OUT. value."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG31_A::A6OUT)
    }
    #[doc = "Output is B7OUT2 value."]
    #[inline(always)]
    pub fn b7out2(self) -> &'a mut W {
        self.variant(CFG31_A::B7OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG31_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG31_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad output 30 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG30_A {
    #[doc = "7: Output is A7OUT2. value."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2. value."]
    A6OUT2 = 6,
    #[doc = "5: Output is A0OUT2. value."]
    A0OUT2 = 5,
    #[doc = "4: Output is A4OUT2. value."]
    A4OUT2 = 4,
    #[doc = "3: Output is B3OUT. value."]
    B3OUT = 3,
    #[doc = "2: Output is B7OUT value."]
    B7OUT = 2,
    #[doc = "1: Force output to 1. value."]
    ONE = 1,
    #[doc = "0: Force output to 0 value."]
    ZERO = 0,
}
impl From<CFG30_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG30_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG30` reader - Pad output 30 configuration"]
pub struct CFG30_R(crate::FieldReader<u8, CFG30_A>);
impl CFG30_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG30_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG30_A {
        match self.bits {
            7 => CFG30_A::A7OUT2,
            6 => CFG30_A::A6OUT2,
            5 => CFG30_A::A0OUT2,
            4 => CFG30_A::A4OUT2,
            3 => CFG30_A::B3OUT,
            2 => CFG30_A::B7OUT,
            1 => CFG30_A::ONE,
            0 => CFG30_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        **self == CFG30_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        **self == CFG30_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT2`"]
    #[inline(always)]
    pub fn is_a0out2(&self) -> bool {
        **self == CFG30_A::A0OUT2
    }
    #[doc = "Checks if the value of the field is `A4OUT2`"]
    #[inline(always)]
    pub fn is_a4out2(&self) -> bool {
        **self == CFG30_A::A4OUT2
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        **self == CFG30_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline(always)]
    pub fn is_b7out(&self) -> bool {
        **self == CFG30_A::B7OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == CFG30_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == CFG30_A::ZERO
    }
}
impl core::ops::Deref for CFG30_R {
    type Target = crate::FieldReader<u8, CFG30_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG30` writer - Pad output 30 configuration"]
pub struct CFG30_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG30_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG30_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG30_A::A6OUT2)
    }
    #[doc = "Output is A0OUT2. value."]
    #[inline(always)]
    pub fn a0out2(self) -> &'a mut W {
        self.variant(CFG30_A::A0OUT2)
    }
    #[doc = "Output is A4OUT2. value."]
    #[inline(always)]
    pub fn a4out2(self) -> &'a mut W {
        self.variant(CFG30_A::A4OUT2)
    }
    #[doc = "Output is B3OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(CFG30_A::B3OUT)
    }
    #[doc = "Output is B7OUT value."]
    #[inline(always)]
    pub fn b7out(self) -> &'a mut W {
        self.variant(CFG30_A::B7OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG30_A::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG30_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - Pad output 31 configuration"]
    #[inline(always)]
    pub fn cfg31(&self) -> CFG31_R {
        CFG31_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Pad output 30 configuration"]
    #[inline(always)]
    pub fn cfg30(&self) -> CFG30_R {
        CFG30_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - Pad output 31 configuration"]
    #[inline(always)]
    pub fn cfg31(&mut self) -> CFG31_W {
        CFG31_W { w: self }
    }
    #[doc = "Bits 0:2 - Pad output 30 configuration"]
    #[inline(always)]
    pub fn cfg30(&mut self) -> CFG30_W {
        CFG30_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Output Config 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outcfg3](index.html) module"]
pub struct OUTCFG3_SPEC;
impl crate::RegisterSpec for OUTCFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outcfg3::R](R) reader structure"]
impl crate::Readable for OUTCFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outcfg3::W](W) writer structure"]
impl crate::Writable for OUTCFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTCFG3 to value 0x12"]
impl crate::Resettable for OUTCFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x12
    }
}
