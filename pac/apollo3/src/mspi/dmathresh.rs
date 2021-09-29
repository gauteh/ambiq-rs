#[doc = "Register `DMATHRESH` reader"]
pub struct R(crate::R<DMATHRESH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATHRESH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATHRESH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATHRESH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATHRESH` writer"]
pub struct W(crate::W<DMATHRESH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATHRESH_SPEC>;
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
impl From<crate::W<DMATHRESH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATHRESH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMATHRESH` reader - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
pub struct DMATHRESH_R(crate::FieldReader<u8, u8>);
impl DMATHRESH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMATHRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMATHRESH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMATHRESH` writer - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
pub struct DMATHRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATHRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
    #[inline(always)]
    pub fn dmathresh(&self) -> DMATHRESH_R {
        DMATHRESH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
    #[inline(always)]
    pub fn dmathresh(&mut self) -> DMATHRESH_W {
        DMATHRESH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Transmit Trigger Threshhold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmathresh](index.html) module"]
pub struct DMATHRESH_SPEC;
impl crate::RegisterSpec for DMATHRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmathresh::R](R) reader structure"]
impl crate::Readable for DMATHRESH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmathresh::W](W) writer structure"]
impl crate::Writable for DMATHRESH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMATHRESH to value 0x08"]
impl crate::Resettable for DMATHRESH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
