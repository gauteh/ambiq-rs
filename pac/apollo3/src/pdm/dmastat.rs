#[doc = "Register `DMASTAT` reader"]
pub struct R(crate::R<DMASTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASTAT` writer"]
pub struct W(crate::W<DMASTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASTAT_SPEC>;
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
impl From<crate::W<DMASTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAERR` reader - DMA Error"]
pub struct DMAERR_R(crate::FieldReader<bool, bool>);
impl DMAERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAERR` writer - DMA Error"]
pub struct DMAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DMACPL` reader - DMA Transfer Complete"]
pub struct DMACPL_R(crate::FieldReader<bool, bool>);
impl DMACPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMACPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMACPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMACPL` writer - DMA Transfer Complete"]
pub struct DMACPL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACPL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DMATIP` reader - DMA Transfer In Progress"]
pub struct DMATIP_R(crate::FieldReader<bool, bool>);
impl DMATIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMATIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMATIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMATIP` writer - DMA Transfer In Progress"]
pub struct DMATIP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATIP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - DMA Error"]
    #[inline(always)]
    pub fn dmaerr(&self) -> DMAERR_R {
        DMAERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Transfer Complete"]
    #[inline(always)]
    pub fn dmacpl(&self) -> DMACPL_R {
        DMACPL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA Transfer In Progress"]
    #[inline(always)]
    pub fn dmatip(&self) -> DMATIP_R {
        DMATIP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DMA Error"]
    #[inline(always)]
    pub fn dmaerr(&mut self) -> DMAERR_W {
        DMAERR_W { w: self }
    }
    #[doc = "Bit 1 - DMA Transfer Complete"]
    #[inline(always)]
    pub fn dmacpl(&mut self) -> DMACPL_W {
        DMACPL_W { w: self }
    }
    #[doc = "Bit 0 - DMA Transfer In Progress"]
    #[inline(always)]
    pub fn dmatip(&mut self) -> DMATIP_W {
        DMATIP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmastat](index.html) module"]
pub struct DMASTAT_SPEC;
impl crate::RegisterSpec for DMASTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmastat::R](R) reader structure"]
impl crate::Readable for DMASTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmastat::W](W) writer structure"]
impl crate::Writable for DMASTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMASTAT to value 0"]
impl crate::Resettable for DMASTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
