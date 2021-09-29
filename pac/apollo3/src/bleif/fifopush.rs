#[doc = "Register `FIFOPUSH` reader"]
pub struct R(crate::R<FIFOPUSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOPUSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOPUSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOPUSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOPUSH` writer"]
pub struct W(crate::W<FIFOPUSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOPUSH_SPEC>;
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
impl From<crate::W<FIFOPUSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOPUSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFODIN` reader - This register is used to write the FIFORAM in FIFO mode and will cause a push event to occur to the next open slot within the FIFORAM. Writing to this register will cause the write point to increment by 1 word(4 bytes)."]
pub struct FIFODIN_R(crate::FieldReader<u32, u32>);
impl FIFODIN_R {
    pub(crate) fn new(bits: u32) -> Self {
        FIFODIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFODIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFODIN` writer - This register is used to write the FIFORAM in FIFO mode and will cause a push event to occur to the next open slot within the FIFORAM. Writing to this register will cause the write point to increment by 1 word(4 bytes)."]
pub struct FIFODIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFODIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register is used to write the FIFORAM in FIFO mode and will cause a push event to occur to the next open slot within the FIFORAM. Writing to this register will cause the write point to increment by 1 word(4 bytes)."]
    #[inline(always)]
    pub fn fifodin(&self) -> FIFODIN_R {
        FIFODIN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used to write the FIFORAM in FIFO mode and will cause a push event to occur to the next open slot within the FIFORAM. Writing to this register will cause the write point to increment by 1 word(4 bytes)."]
    #[inline(always)]
    pub fn fifodin(&mut self) -> FIFODIN_W {
        FIFODIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO PUSH register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifopush](index.html) module"]
pub struct FIFOPUSH_SPEC;
impl crate::RegisterSpec for FIFOPUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifopush::R](R) reader structure"]
impl crate::Readable for FIFOPUSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifopush::W](W) writer structure"]
impl crate::Writable for FIFOPUSH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOPUSH to value 0"]
impl crate::Resettable for FIFOPUSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
