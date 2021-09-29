#[doc = "Register `VOICESTAT` reader"]
pub struct R(crate::R<VOICESTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOICESTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOICESTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOICESTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VOICESTAT` writer"]
pub struct W(crate::W<VOICESTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VOICESTAT_SPEC>;
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
impl From<crate::W<VOICESTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VOICESTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOCNT` reader - Valid 32-bit entries currently in the FIFO."]
pub struct FIFOCNT_R(crate::FieldReader<u8, u8>);
impl FIFOCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOCNT` writer - Valid 32-bit entries currently in the FIFO."]
pub struct FIFOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Valid 32-bit entries currently in the FIFO."]
    #[inline(always)]
    pub fn fifocnt(&self) -> FIFOCNT_R {
        FIFOCNT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Valid 32-bit entries currently in the FIFO."]
    #[inline(always)]
    pub fn fifocnt(&mut self) -> FIFOCNT_W {
        FIFOCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voice Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [voicestat](index.html) module"]
pub struct VOICESTAT_SPEC;
impl crate::RegisterSpec for VOICESTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [voicestat::R](R) reader structure"]
impl crate::Readable for VOICESTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [voicestat::W](W) writer structure"]
impl crate::Writable for VOICESTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VOICESTAT to value 0"]
impl crate::Resettable for VOICESTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
