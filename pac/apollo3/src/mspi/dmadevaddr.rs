#[doc = "Register `DMADEVADDR` reader"]
pub struct R(crate::R<DMADEVADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMADEVADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMADEVADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMADEVADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMADEVADDR` writer"]
pub struct W(crate::W<DMADEVADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMADEVADDR_SPEC>;
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
impl From<crate::W<DMADEVADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMADEVADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVADDR` reader - SPI Device address for automated DMA transactions (both read and write)."]
pub struct DEVADDR_R(crate::FieldReader<u32, u32>);
impl DEVADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DEVADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVADDR` writer - SPI Device address for automated DMA transactions (both read and write)."]
pub struct DEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SPI Device address for automated DMA transactions (both read and write)."]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI Device address for automated DMA transactions (both read and write)."]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Device Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmadevaddr](index.html) module"]
pub struct DMADEVADDR_SPEC;
impl crate::RegisterSpec for DMADEVADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmadevaddr::R](R) reader structure"]
impl crate::Readable for DMADEVADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmadevaddr::W](W) writer structure"]
impl crate::Writable for DMADEVADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMADEVADDR to value 0"]
impl crate::Resettable for DMADEVADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
