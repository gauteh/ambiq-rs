#[doc = "Register `GENADD` reader"]
pub struct R(crate::R<GENADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GENADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GENADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GENADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GENADD` writer"]
pub struct W(crate::W<GENADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GENADD_SPEC>;
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
impl From<crate::W<GENADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GENADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GADATA` reader - The data supplied on the last General Address reference."]
pub struct GADATA_R(crate::FieldReader<u8, u8>);
impl GADATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        GADATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GADATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GADATA` writer - The data supplied on the last General Address reference."]
pub struct GADATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GADATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The data supplied on the last General Address reference."]
    #[inline(always)]
    pub fn gadata(&self) -> GADATA_R {
        GADATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The data supplied on the last General Address reference."]
    #[inline(always)]
    pub fn gadata(&mut self) -> GADATA_W {
        GADATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Address Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [genadd](index.html) module"]
pub struct GENADD_SPEC;
impl crate::RegisterSpec for GENADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [genadd::R](R) reader structure"]
impl crate::Readable for GENADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [genadd::W](W) writer structure"]
impl crate::Writable for GENADD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GENADD to value 0"]
impl crate::Resettable for GENADD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
