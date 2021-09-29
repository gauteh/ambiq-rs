#[doc = "Register `DMATRIGEN` reader"]
pub struct R(crate::R<DMATRIGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATRIGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATRIGEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATRIGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATRIGEN` writer"]
pub struct W(crate::W<DMATRIGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATRIGEN_SPEC>;
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
impl From<crate::W<DMATRIGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATRIGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFIFOFULL` reader - Trigger DMA upon FIFO 100 percent Full"]
pub struct DFIFOFULL_R(crate::FieldReader<bool, bool>);
impl DFIFOFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFIFOFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFIFOFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFIFOFULL` writer - Trigger DMA upon FIFO 100 percent Full"]
pub struct DFIFOFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFIFOFULL_W<'a> {
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
#[doc = "Field `DFIFO75` reader - Trigger DMA upon FIFO 75 percent Full"]
pub struct DFIFO75_R(crate::FieldReader<bool, bool>);
impl DFIFO75_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFIFO75_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFIFO75_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFIFO75` writer - Trigger DMA upon FIFO 75 percent Full"]
pub struct DFIFO75_W<'a> {
    w: &'a mut W,
}
impl<'a> DFIFO75_W<'a> {
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
    #[doc = "Bit 1 - Trigger DMA upon FIFO 100 percent Full"]
    #[inline(always)]
    pub fn dfifofull(&self) -> DFIFOFULL_R {
        DFIFOFULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Trigger DMA upon FIFO 75 percent Full"]
    #[inline(always)]
    pub fn dfifo75(&self) -> DFIFO75_R {
        DFIFO75_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Trigger DMA upon FIFO 100 percent Full"]
    #[inline(always)]
    pub fn dfifofull(&mut self) -> DFIFOFULL_W {
        DFIFOFULL_W { w: self }
    }
    #[doc = "Bit 0 - Trigger DMA upon FIFO 75 percent Full"]
    #[inline(always)]
    pub fn dfifo75(&mut self) -> DFIFO75_W {
        DFIFO75_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Trigger Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatrigen](index.html) module"]
pub struct DMATRIGEN_SPEC;
impl crate::RegisterSpec for DMATRIGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmatrigen::R](R) reader structure"]
impl crate::Readable for DMATRIGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatrigen::W](W) writer structure"]
impl crate::Writable for DMATRIGEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMATRIGEN to value 0"]
impl crate::Resettable for DMATRIGEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
