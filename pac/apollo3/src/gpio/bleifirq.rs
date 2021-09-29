#[doc = "Register `BLEIFIRQ` reader"]
pub struct R(crate::R<BLEIFIRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLEIFIRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLEIFIRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLEIFIRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLEIFIRQ` writer"]
pub struct W(crate::W<BLEIFIRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLEIFIRQ_SPEC>;
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
impl From<crate::W<BLEIFIRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLEIFIRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLEIFIRQ` reader - BLEIF IRQ pad select."]
pub struct BLEIFIRQ_R(crate::FieldReader<u8, u8>);
impl BLEIFIRQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLEIFIRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEIFIRQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEIFIRQ` writer - BLEIF IRQ pad select."]
pub struct BLEIFIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEIFIRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - BLEIF IRQ pad select."]
    #[inline(always)]
    pub fn bleifirq(&self) -> BLEIFIRQ_R {
        BLEIFIRQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - BLEIF IRQ pad select."]
    #[inline(always)]
    pub fn bleifirq(&mut self) -> BLEIFIRQ_W {
        BLEIFIRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLEIF Flow Control IRQ Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bleifirq](index.html) module"]
pub struct BLEIFIRQ_SPEC;
impl crate::RegisterSpec for BLEIFIRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bleifirq::R](R) reader structure"]
impl crate::Readable for BLEIFIRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bleifirq::W](W) writer structure"]
impl crate::Writable for BLEIFIRQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLEIFIRQ to value 0x3f"]
impl crate::Resettable for BLEIFIRQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
