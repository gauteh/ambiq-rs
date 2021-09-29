#[doc = "Register `RDA` reader"]
pub struct R(crate::R<RDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDA` writer"]
pub struct W(crate::W<RDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDA_SPEC>;
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
impl From<crate::W<RDA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDA` reader - GPIO31-0 read data."]
pub struct RDA_R(crate::FieldReader<u32, u32>);
impl RDA_R {
    pub(crate) fn new(bits: u32) -> Self {
        RDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDA` writer - GPIO31-0 read data."]
pub struct RDA_W<'a> {
    w: &'a mut W,
}
impl<'a> RDA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO31-0 read data."]
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO31-0 read data."]
    #[inline(always)]
    pub fn rda(&mut self) -> RDA_W {
        RDA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Input Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rda](index.html) module"]
pub struct RDA_SPEC;
impl crate::RegisterSpec for RDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rda::R](R) reader structure"]
impl crate::Readable for RDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rda::W](W) writer structure"]
impl crate::Writable for RDA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RDA to value 0"]
impl crate::Resettable for RDA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
