#[doc = "Register `TXFIFO` reader"]
pub struct R(crate::R<TXFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXFIFO` writer"]
pub struct W(crate::W<TXFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFIFO_SPEC>;
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
impl From<crate::W<TXFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFIFO` reader - Data to be transmitted. Data should normall be aligned to the LSB (pad the upper bits with zeros) unless BIGENDIAN is set."]
pub struct TXFIFO_R(crate::FieldReader<u32, u32>);
impl TXFIFO_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXFIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO` writer - Data to be transmitted. Data should normall be aligned to the LSB (pad the upper bits with zeros) unless BIGENDIAN is set."]
pub struct TXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data to be transmitted. Data should normall be aligned to the LSB (pad the upper bits with zeros) unless BIGENDIAN is set."]
    #[inline(always)]
    pub fn txfifo(&self) -> TXFIFO_R {
        TXFIFO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to be transmitted. Data should normall be aligned to the LSB (pad the upper bits with zeros) unless BIGENDIAN is set."]
    #[inline(always)]
    pub fn txfifo(&mut self) -> TXFIFO_W {
        TXFIFO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Data FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfifo](index.html) module"]
pub struct TXFIFO_SPEC;
impl crate::RegisterSpec for TXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfifo::R](R) reader structure"]
impl crate::Readable for TXFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txfifo::W](W) writer structure"]
impl crate::Writable for TXFIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXFIFO to value 0"]
impl crate::Resettable for TXFIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
