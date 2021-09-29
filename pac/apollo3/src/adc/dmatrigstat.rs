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
#[doc = "Field `DFULLSTAT` reader - Triggered DMA from FIFO 100 percent Full"]
pub struct DFULLSTAT_R(crate::FieldReader<bool, bool>);
impl DFULLSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFULLSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFULLSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFULLSTAT` writer - Triggered DMA from FIFO 100 percent Full"]
pub struct DFULLSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DFULLSTAT_W<'a> {
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
#[doc = "Field `D75STAT` reader - Triggered DMA from FIFO 75 percent Full"]
pub struct D75STAT_R(crate::FieldReader<bool, bool>);
impl D75STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        D75STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D75STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D75STAT` writer - Triggered DMA from FIFO 75 percent Full"]
pub struct D75STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> D75STAT_W<'a> {
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
    #[doc = "Bit 1 - Triggered DMA from FIFO 100 percent Full"]
    #[inline(always)]
    pub fn dfullstat(&self) -> DFULLSTAT_R {
        DFULLSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Triggered DMA from FIFO 75 percent Full"]
    #[inline(always)]
    pub fn d75stat(&self) -> D75STAT_R {
        D75STAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Triggered DMA from FIFO 100 percent Full"]
    #[inline(always)]
    pub fn dfullstat(&mut self) -> DFULLSTAT_W {
        DFULLSTAT_W { w: self }
    }
    #[doc = "Bit 0 - Triggered DMA from FIFO 75 percent Full"]
    #[inline(always)]
    pub fn d75stat(&mut self) -> D75STAT_W {
        D75STAT_W { w: self }
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
