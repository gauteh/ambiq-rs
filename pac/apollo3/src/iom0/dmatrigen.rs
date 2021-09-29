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
#[doc = "Field `DTHREN` reader - Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
pub struct DTHREN_R(crate::FieldReader<bool, bool>);
impl DTHREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTHREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTHREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTHREN` writer - Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
pub struct DTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHREN_W<'a> {
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
#[doc = "Field `DCMDCMPEN` reader - Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or"]
pub struct DCMDCMPEN_R(crate::FieldReader<bool, bool>);
impl DCMDCMPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMDCMPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCMDCMPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMDCMPEN` writer - Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or"]
pub struct DCMDCMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMDCMPEN_W<'a> {
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
    #[doc = "Bit 1 - Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
    #[inline(always)]
    pub fn dthren(&self) -> DTHREN_R {
        DTHREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or"]
    #[inline(always)]
    pub fn dcmdcmpen(&self) -> DCMDCMPEN_R {
        DCMDCMPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
    #[inline(always)]
    pub fn dthren(&mut self) -> DTHREN_W {
        DTHREN_W { w: self }
    }
    #[doc = "Bit 0 - Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or"]
    #[inline(always)]
    pub fn dcmdcmpen(&mut self) -> DCMDCMPEN_W {
        DCMDCMPEN_W { w: self }
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
