#[doc = "Register `LEN` reader"]
pub struct R(crate::R<LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEN` writer"]
pub struct W(crate::W<LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEN_SPEC>;
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
impl From<crate::W<LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN` reader - Buffer size (bottom two bits assumed to be zero to ensure a multiple of 4 bytes)"]
pub struct LEN_R(crate::FieldReader<u32, u32>);
impl LEN_R {
    pub(crate) fn new(bits: u32) -> Self {
        LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEN` writer - Buffer size (bottom two bits assumed to be zero to ensure a multiple of 4 bytes)"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 2)) | ((value as u32 & 0x0003_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:19 - Buffer size (bottom two bits assumed to be zero to ensure a multiple of 4 bytes)"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 2) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:19 - Buffer size (bottom two bits assumed to be zero to ensure a multiple of 4 bytes)"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [len](index.html) module"]
pub struct LEN_SPEC;
impl crate::RegisterSpec for LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [len::R](R) reader structure"]
impl crate::Readable for LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [len::W](W) writer structure"]
impl crate::Writable for LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEN to value 0"]
impl crate::Resettable for LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
