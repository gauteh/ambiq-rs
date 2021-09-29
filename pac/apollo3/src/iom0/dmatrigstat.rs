#[doc = "Register `DMATRIGSTAT` reader"]
pub struct R(crate::R<DMATRIGSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATRIGSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATRIGSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATRIGSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATRIGSTAT` writer"]
pub struct W(crate::W<DMATRIGSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATRIGSTAT_SPEC>;
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
impl From<crate::W<DMATRIGSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATRIGSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTOTCMP` reader - DMA triggered when DCMDCMP = 0, and the amount of data in the FIFO was enough to complete the DMA operation (greater than or equal to current TOTCOUNT) when the command completed. This trigger is default active when the DCMDCMP trigger is disabled and there is enough data in the FIFO to complete the DMA operation."]
pub struct DTOTCMP_R(crate::FieldReader<bool, bool>);
impl DTOTCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTOTCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOTCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOTCMP` writer - DMA triggered when DCMDCMP = 0, and the amount of data in the FIFO was enough to complete the DMA operation (greater than or equal to current TOTCOUNT) when the command completed. This trigger is default active when the DCMDCMP trigger is disabled and there is enough data in the FIFO to complete the DMA operation."]
pub struct DTOTCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOTCMP_W<'a> {
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
#[doc = "Field `DTHR` reader - Triggered DMA from THR event. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
pub struct DTHR_R(crate::FieldReader<bool, bool>);
impl DTHR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTHR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTHR` writer - Triggered DMA from THR event. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
pub struct DTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHR_W<'a> {
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
#[doc = "Field `DCMDCMP` reader - Triggered DMA from Command complete event. Bit is read only and can be cleared by disabling the DCMDCMP trigger enable or by disabling DMA."]
pub struct DCMDCMP_R(crate::FieldReader<bool, bool>);
impl DCMDCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMDCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCMDCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMDCMP` writer - Triggered DMA from Command complete event. Bit is read only and can be cleared by disabling the DCMDCMP trigger enable or by disabling DMA."]
pub struct DCMDCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMDCMP_W<'a> {
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
    #[doc = "Bit 2 - DMA triggered when DCMDCMP = 0, and the amount of data in the FIFO was enough to complete the DMA operation (greater than or equal to current TOTCOUNT) when the command completed. This trigger is default active when the DCMDCMP trigger is disabled and there is enough data in the FIFO to complete the DMA operation."]
    #[inline(always)]
    pub fn dtotcmp(&self) -> DTOTCMP_R {
        DTOTCMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Triggered DMA from THR event. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
    #[inline(always)]
    pub fn dthr(&self) -> DTHR_R {
        DTHR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Triggered DMA from Command complete event. Bit is read only and can be cleared by disabling the DCMDCMP trigger enable or by disabling DMA."]
    #[inline(always)]
    pub fn dcmdcmp(&self) -> DCMDCMP_R {
        DCMDCMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DMA triggered when DCMDCMP = 0, and the amount of data in the FIFO was enough to complete the DMA operation (greater than or equal to current TOTCOUNT) when the command completed. This trigger is default active when the DCMDCMP trigger is disabled and there is enough data in the FIFO to complete the DMA operation."]
    #[inline(always)]
    pub fn dtotcmp(&mut self) -> DTOTCMP_W {
        DTOTCMP_W { w: self }
    }
    #[doc = "Bit 1 - Triggered DMA from THR event. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
    #[inline(always)]
    pub fn dthr(&mut self) -> DTHR_W {
        DTHR_W { w: self }
    }
    #[doc = "Bit 0 - Triggered DMA from Command complete event. Bit is read only and can be cleared by disabling the DCMDCMP trigger enable or by disabling DMA."]
    #[inline(always)]
    pub fn dcmdcmp(&mut self) -> DCMDCMP_W {
        DCMDCMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Trigger Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatrigstat](index.html) module"]
pub struct DMATRIGSTAT_SPEC;
impl crate::RegisterSpec for DMATRIGSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmatrigstat::R](R) reader structure"]
impl crate::Readable for DMATRIGSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatrigstat::W](W) writer structure"]
impl crate::Writable for DMATRIGSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMATRIGSTAT to value 0"]
impl crate::Resettable for DMATRIGSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
