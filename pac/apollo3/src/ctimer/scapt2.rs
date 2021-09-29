#[doc = "Register `SCAPT2` reader"]
pub struct R(crate::R<SCAPT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCAPT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCAPT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCAPT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCAPT2` writer"]
pub struct W(crate::W<SCAPT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCAPT2_SPEC>;
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
impl From<crate::W<SCAPT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCAPT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCAPT2` reader - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
pub struct SCAPT2_R(crate::FieldReader<u32, u32>);
impl SCAPT2_R {
    pub(crate) fn new(bits: u32) -> Self {
        SCAPT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCAPT2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCAPT2` writer - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
pub struct SCAPT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAPT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    pub fn scapt2(&self) -> SCAPT2_R {
        SCAPT2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline(always)]
    pub fn scapt2(&mut self) -> SCAPT2_W {
        SCAPT2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scapt2](index.html) module"]
pub struct SCAPT2_SPEC;
impl crate::RegisterSpec for SCAPT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scapt2::R](R) reader structure"]
impl crate::Readable for SCAPT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scapt2::W](W) writer structure"]
impl crate::Writable for SCAPT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCAPT2 to value 0"]
impl crate::Resettable for SCAPT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
