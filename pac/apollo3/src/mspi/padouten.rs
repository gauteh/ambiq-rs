#[doc = "Register `PADOUTEN` reader"]
pub struct R(crate::R<PADOUTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADOUTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADOUTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADOUTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADOUTEN` writer"]
pub struct W(crate::W<PADOUTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADOUTEN_SPEC>;
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
impl From<crate::W<PADOUTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADOUTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data, \\[7:4\\]
are Quad1 data, and \\[8\\]
is clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum OUTEN_A {
    #[doc = "271: Quad0 (4 data + 1 clock) value."]
    QUAD0 = 271,
    #[doc = "496: Quad1 (4 data + 1 clock) value."]
    QUAD1 = 496,
    #[doc = "511: Octal (8 data + 1 clock) value."]
    OCTAL = 511,
    #[doc = "259: Serial (2 data + 1 clock) value."]
    SERIAL0 = 259,
}
impl From<OUTEN_A> for u16 {
    #[inline(always)]
    fn from(variant: OUTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OUTEN` reader - Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data, \\[7:4\\]
are Quad1 data, and \\[8\\]
is clock."]
pub struct OUTEN_R(crate::FieldReader<u16, OUTEN_A>);
impl OUTEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        OUTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUTEN_A> {
        match self.bits {
            271 => Some(OUTEN_A::QUAD0),
            496 => Some(OUTEN_A::QUAD1),
            511 => Some(OUTEN_A::OCTAL),
            259 => Some(OUTEN_A::SERIAL0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `QUAD0`"]
    #[inline(always)]
    pub fn is_quad0(&self) -> bool {
        **self == OUTEN_A::QUAD0
    }
    #[doc = "Checks if the value of the field is `QUAD1`"]
    #[inline(always)]
    pub fn is_quad1(&self) -> bool {
        **self == OUTEN_A::QUAD1
    }
    #[doc = "Checks if the value of the field is `OCTAL`"]
    #[inline(always)]
    pub fn is_octal(&self) -> bool {
        **self == OUTEN_A::OCTAL
    }
    #[doc = "Checks if the value of the field is `SERIAL0`"]
    #[inline(always)]
    pub fn is_serial0(&self) -> bool {
        **self == OUTEN_A::SERIAL0
    }
}
impl core::ops::Deref for OUTEN_R {
    type Target = crate::FieldReader<u16, OUTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEN` writer - Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data, \\[7:4\\]
are Quad1 data, and \\[8\\]
is clock."]
pub struct OUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Quad0 (4 data + 1 clock) value."]
    #[inline(always)]
    pub fn quad0(self) -> &'a mut W {
        self.variant(OUTEN_A::QUAD0)
    }
    #[doc = "Quad1 (4 data + 1 clock) value."]
    #[inline(always)]
    pub fn quad1(self) -> &'a mut W {
        self.variant(OUTEN_A::QUAD1)
    }
    #[doc = "Octal (8 data + 1 clock) value."]
    #[inline(always)]
    pub fn octal(self) -> &'a mut W {
        self.variant(OUTEN_A::OCTAL)
    }
    #[doc = "Serial (2 data + 1 clock) value."]
    #[inline(always)]
    pub fn serial0(self) -> &'a mut W {
        self.variant(OUTEN_A::SERIAL0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data, \\[7:4\\]
are Quad1 data, and \\[8\\]
is clock."]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data, \\[7:4\\]
are Quad1 data, and \\[8\\]
is clock."]
    #[inline(always)]
    pub fn outen(&mut self) -> OUTEN_W {
        OUTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI Output Enable Pad Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padouten](index.html) module"]
pub struct PADOUTEN_SPEC;
impl crate::RegisterSpec for PADOUTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padouten::R](R) reader structure"]
impl crate::Readable for PADOUTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padouten::W](W) writer structure"]
impl crate::Writable for PADOUTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADOUTEN to value 0"]
impl crate::Resettable for PADOUTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
