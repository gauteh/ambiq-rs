#[doc = "Register `FIFOREAD` reader"]
pub struct R(crate::R<FIFOREAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOREAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOREAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOREAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOREAD` writer"]
pub struct W(crate::W<FIFOREAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOREAD_SPEC>;
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
impl From<crate::W<FIFOREAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOREAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOREAD` reader - FIFO read data."]
pub struct FIFOREAD_R(crate::FieldReader<u32, u32>);
impl FIFOREAD_R {
    pub(crate) fn new(bits: u32) -> Self {
        FIFOREAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOREAD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOREAD` writer - FIFO read data."]
pub struct FIFOREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOREAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FIFO read data."]
    #[inline(always)]
    pub fn fiforead(&self) -> FIFOREAD_R {
        FIFOREAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO read data."]
    #[inline(always)]
    pub fn fiforead(&mut self) -> FIFOREAD_W {
        FIFOREAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Read\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fiforead](index.html) module"]
pub struct FIFOREAD_SPEC;
impl crate::RegisterSpec for FIFOREAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fiforead::R](R) reader structure"]
impl crate::Readable for FIFOREAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fiforead::W](W) writer structure"]
impl crate::Writable for FIFOREAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOREAD to value 0"]
impl crate::Resettable for FIFOREAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
