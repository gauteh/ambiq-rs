#[doc = "Register `DMATARGADDR` reader"]
pub struct R(crate::R<DMATARGADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATARGADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATARGADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATARGADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATARGADDR` writer"]
pub struct W(crate::W<DMATARGADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATARGADDR_SPEC>;
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
impl From<crate::W<DMATARGADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATARGADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTARGADDR` reader - SRAM Target"]
pub struct UTARGADDR_R(crate::FieldReader<u16, u16>);
impl UTARGADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        UTARGADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTARGADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTARGADDR` writer - SRAM Target"]
pub struct UTARGADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UTARGADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | ((value as u32 & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Field `LTARGADDR` reader - DMA Target Address. This register is not updated with the current address of the DMA, but will remain static with the original address during the DMA transfer."]
pub struct LTARGADDR_R(crate::FieldReader<u32, u32>);
impl LTARGADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        LTARGADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTARGADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTARGADDR` writer - DMA Target Address. This register is not updated with the current address of the DMA, but will remain static with the original address during the DMA transfer."]
pub struct LTARGADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LTARGADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - SRAM Target"]
    #[inline(always)]
    pub fn utargaddr(&self) -> UTARGADDR_R {
        UTARGADDR_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:19 - DMA Target Address. This register is not updated with the current address of the DMA, but will remain static with the original address during the DMA transfer."]
    #[inline(always)]
    pub fn ltargaddr(&self) -> LTARGADDR_R {
        LTARGADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 20:31 - SRAM Target"]
    #[inline(always)]
    pub fn utargaddr(&mut self) -> UTARGADDR_W {
        UTARGADDR_W { w: self }
    }
    #[doc = "Bits 0:19 - DMA Target Address. This register is not updated with the current address of the DMA, but will remain static with the original address during the DMA transfer."]
    #[inline(always)]
    pub fn ltargaddr(&mut self) -> LTARGADDR_W {
        LTARGADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Target Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatargaddr](index.html) module"]
pub struct DMATARGADDR_SPEC;
impl crate::RegisterSpec for DMATARGADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmatargaddr::R](R) reader structure"]
impl crate::Readable for DMATARGADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatargaddr::W](W) writer structure"]
impl crate::Writable for DMATARGADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMATARGADDR to value 0x1000_0000"]
impl crate::Resettable for DMATARGADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0000
    }
}
