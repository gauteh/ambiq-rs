#[doc = "Register `DMABCOUNT` reader"]
pub struct R(crate::R<DMABCOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMABCOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMABCOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMABCOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMABCOUNT` writer"]
pub struct W(crate::W<DMABCOUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMABCOUNT_SPEC>;
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
impl From<crate::W<DMABCOUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMABCOUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCOUNT` reader - Burst transfer size in bytes. This is the number of bytes transferred when a FIFO trigger event occurs. Recommended values are 16 or 32."]
pub struct BCOUNT_R(crate::FieldReader<u8, u8>);
impl BCOUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        BCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCOUNT` writer - Burst transfer size in bytes. This is the number of bytes transferred when a FIFO trigger event occurs. Recommended values are 16 or 32."]
pub struct BCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Burst transfer size in bytes. This is the number of bytes transferred when a FIFO trigger event occurs. Recommended values are 16 or 32."]
    #[inline(always)]
    pub fn bcount(&self) -> BCOUNT_R {
        BCOUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Burst transfer size in bytes. This is the number of bytes transferred when a FIFO trigger event occurs. Recommended values are 16 or 32."]
    #[inline(always)]
    pub fn bcount(&mut self) -> BCOUNT_W {
        BCOUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA BYTE Transfer Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmabcount](index.html) module"]
pub struct DMABCOUNT_SPEC;
impl crate::RegisterSpec for DMABCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmabcount::R](R) reader structure"]
impl crate::Readable for DMABCOUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmabcount::W](W) writer structure"]
impl crate::Writable for DMABCOUNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMABCOUNT to value 0"]
impl crate::Resettable for DMABCOUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
