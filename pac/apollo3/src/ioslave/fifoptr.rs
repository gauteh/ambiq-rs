#[doc = "Register `FIFOPTR` reader"]
pub struct R(crate::R<FIFOPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOPTR` writer"]
pub struct W(crate::W<FIFOPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOPTR_SPEC>;
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
impl From<crate::W<FIFOPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOSIZ` reader - The number of bytes currently in the hardware FIFO."]
pub struct FIFOSIZ_R(crate::FieldReader<u8, u8>);
impl FIFOSIZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOSIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOSIZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOSIZ` writer - The number of bytes currently in the hardware FIFO."]
pub struct FIFOSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOSIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `FIFOPTR` reader - Current FIFO pointer."]
pub struct FIFOPTR_R(crate::FieldReader<u8, u8>);
impl FIFOPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOPTR` writer - Current FIFO pointer."]
pub struct FIFOPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - The number of bytes currently in the hardware FIFO."]
    #[inline(always)]
    pub fn fifosiz(&self) -> FIFOSIZ_R {
        FIFOSIZ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Current FIFO pointer."]
    #[inline(always)]
    pub fn fifoptr(&self) -> FIFOPTR_R {
        FIFOPTR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - The number of bytes currently in the hardware FIFO."]
    #[inline(always)]
    pub fn fifosiz(&mut self) -> FIFOSIZ_W {
        FIFOSIZ_W { w: self }
    }
    #[doc = "Bits 0:7 - Current FIFO pointer."]
    #[inline(always)]
    pub fn fifoptr(&mut self) -> FIFOPTR_W {
        FIFOPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current FIFO Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoptr](index.html) module"]
pub struct FIFOPTR_SPEC;
impl crate::RegisterSpec for FIFOPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoptr::R](R) reader structure"]
impl crate::Readable for FIFOPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoptr::W](W) writer structure"]
impl crate::Writable for FIFOPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOPTR to value 0"]
impl crate::Resettable for FIFOPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
