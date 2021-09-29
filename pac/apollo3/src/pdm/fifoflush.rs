#[doc = "Register `FIFOFLUSH` reader"]
pub struct R(crate::R<FIFOFLUSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOFLUSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOFLUSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOFLUSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOFLUSH` writer"]
pub struct W(crate::W<FIFOFLUSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOFLUSH_SPEC>;
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
impl From<crate::W<FIFOFLUSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOFLUSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOFLUSH` reader - FIFO FLUSH."]
pub struct FIFOFLUSH_R(crate::FieldReader<bool, bool>);
impl FIFOFLUSH_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFOFLUSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOFLUSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOFLUSH` writer - FIFO FLUSH."]
pub struct FIFOFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOFLUSH_W<'a> {
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
    #[doc = "Bit 0 - FIFO FLUSH."]
    #[inline(always)]
    pub fn fifoflush(&self) -> FIFOFLUSH_R {
        FIFOFLUSH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO FLUSH."]
    #[inline(always)]
    pub fn fifoflush(&mut self) -> FIFOFLUSH_W {
        FIFOFLUSH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Flush\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoflush](index.html) module"]
pub struct FIFOFLUSH_SPEC;
impl crate::RegisterSpec for FIFOFLUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoflush::R](R) reader structure"]
impl crate::Readable for FIFOFLUSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoflush::W](W) writer structure"]
impl crate::Writable for FIFOFLUSH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOFLUSH to value 0"]
impl crate::Resettable for FIFOFLUSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
