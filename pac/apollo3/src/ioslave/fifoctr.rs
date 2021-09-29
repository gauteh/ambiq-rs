#[doc = "Register `FIFOCTR` reader"]
pub struct R(crate::R<FIFOCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOCTR` writer"]
pub struct W(crate::W<FIFOCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOCTR_SPEC>;
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
impl From<crate::W<FIFOCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOCTR` reader - Virtual FIFO byte count"]
pub struct FIFOCTR_R(crate::FieldReader<u16, u16>);
impl FIFOCTR_R {
    pub(crate) fn new(bits: u16) -> Self {
        FIFOCTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOCTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOCTR` writer - Virtual FIFO byte count"]
pub struct FIFOCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Virtual FIFO byte count"]
    #[inline(always)]
    pub fn fifoctr(&self) -> FIFOCTR_R {
        FIFOCTR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Virtual FIFO byte count"]
    #[inline(always)]
    pub fn fifoctr(&mut self) -> FIFOCTR_W {
        FIFOCTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Overall FIFO Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoctr](index.html) module"]
pub struct FIFOCTR_SPEC;
impl crate::RegisterSpec for FIFOCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoctr::R](R) reader structure"]
impl crate::Readable for FIFOCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoctr::W](W) writer structure"]
impl crate::Writable for FIFOCTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOCTR to value 0"]
impl crate::Resettable for FIFOCTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
