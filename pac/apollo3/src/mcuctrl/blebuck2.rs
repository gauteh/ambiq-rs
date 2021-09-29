#[doc = "Register `BLEBUCK2` reader"]
pub struct R(crate::R<BLEBUCK2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLEBUCK2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLEBUCK2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLEBUCK2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLEBUCK2` writer"]
pub struct W(crate::W<BLEBUCK2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLEBUCK2_SPEC>;
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
impl From<crate::W<BLEBUCK2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLEBUCK2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLEBUCKTOND2ATRIM` reader - blebuck_ton_trim"]
pub struct BLEBUCKTOND2ATRIM_R(crate::FieldReader<u8, u8>);
impl BLEBUCKTOND2ATRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLEBUCKTOND2ATRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEBUCKTOND2ATRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEBUCKTOND2ATRIM` writer - blebuck_ton_trim"]
pub struct BLEBUCKTOND2ATRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEBUCKTOND2ATRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `BLEBUCKTONHITRIM` reader - blebuck_ton_hi_trim"]
pub struct BLEBUCKTONHITRIM_R(crate::FieldReader<u8, u8>);
impl BLEBUCKTONHITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLEBUCKTONHITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEBUCKTONHITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEBUCKTONHITRIM` writer - blebuck_ton_hi_trim"]
pub struct BLEBUCKTONHITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEBUCKTONHITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `BLEBUCKTONLOWTRIM` reader - blebuck_ton_low_trim"]
pub struct BLEBUCKTONLOWTRIM_R(crate::FieldReader<u8, u8>);
impl BLEBUCKTONLOWTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLEBUCKTONLOWTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEBUCKTONLOWTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEBUCKTONLOWTRIM` writer - blebuck_ton_low_trim"]
pub struct BLEBUCKTONLOWTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEBUCKTONLOWTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:17 - blebuck_ton_trim"]
    #[inline(always)]
    pub fn blebucktond2atrim(&self) -> BLEBUCKTOND2ATRIM_R {
        BLEBUCKTOND2ATRIM_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - blebuck_ton_hi_trim"]
    #[inline(always)]
    pub fn blebucktonhitrim(&self) -> BLEBUCKTONHITRIM_R {
        BLEBUCKTONHITRIM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - blebuck_ton_low_trim"]
    #[inline(always)]
    pub fn blebucktonlowtrim(&self) -> BLEBUCKTONLOWTRIM_R {
        BLEBUCKTONLOWTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:17 - blebuck_ton_trim"]
    #[inline(always)]
    pub fn blebucktond2atrim(&mut self) -> BLEBUCKTOND2ATRIM_W {
        BLEBUCKTOND2ATRIM_W { w: self }
    }
    #[doc = "Bits 6:11 - blebuck_ton_hi_trim"]
    #[inline(always)]
    pub fn blebucktonhitrim(&mut self) -> BLEBUCKTONHITRIM_W {
        BLEBUCKTONHITRIM_W { w: self }
    }
    #[doc = "Bits 0:5 - blebuck_ton_low_trim"]
    #[inline(always)]
    pub fn blebucktonlowtrim(&mut self) -> BLEBUCKTONLOWTRIM_W {
        BLEBUCKTONLOWTRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLEBUCK2 Control Reg\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blebuck2](index.html) module"]
pub struct BLEBUCK2_SPEC;
impl crate::RegisterSpec for BLEBUCK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blebuck2::R](R) reader structure"]
impl crate::Readable for BLEBUCK2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blebuck2::W](W) writer structure"]
impl crate::Writable for BLEBUCK2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLEBUCK2 to value 0x4e"]
impl crate::Resettable for BLEBUCK2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4e
    }
}
