#[doc = "Register `FIFOTHR` reader"]
pub struct R(crate::R<FIFOTHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOTHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOTHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOTHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOTHR` writer"]
pub struct W(crate::W<FIFOTHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOTHR_SPEC>;
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
impl From<crate::W<FIFOTHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOTHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOTHR` reader - FIFO Threshold value. When the FIFO count is equal to, or larger than this value (in words), a THR interrupt is generated (if enabled)"]
pub struct FIFOTHR_R(crate::FieldReader<u8, u8>);
impl FIFOTHR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOTHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOTHR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOTHR` writer - FIFO Threshold value. When the FIFO count is equal to, or larger than this value (in words), a THR interrupt is generated (if enabled)"]
pub struct FIFOTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOTHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - FIFO Threshold value. When the FIFO count is equal to, or larger than this value (in words), a THR interrupt is generated (if enabled)"]
    #[inline(always)]
    pub fn fifothr(&self) -> FIFOTHR_R {
        FIFOTHR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - FIFO Threshold value. When the FIFO count is equal to, or larger than this value (in words), a THR interrupt is generated (if enabled)"]
    #[inline(always)]
    pub fn fifothr(&mut self) -> FIFOTHR_W {
        FIFOTHR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifothr](index.html) module"]
pub struct FIFOTHR_SPEC;
impl crate::RegisterSpec for FIFOTHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifothr::R](R) reader structure"]
impl crate::Readable for FIFOTHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifothr::W](W) writer structure"]
impl crate::Writable for FIFOTHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOTHR to value 0x10"]
impl crate::Resettable for FIFOTHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
