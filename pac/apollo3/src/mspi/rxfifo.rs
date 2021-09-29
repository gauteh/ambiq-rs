#[doc = "Register `RXFIFO` reader"]
pub struct R(crate::R<RXFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFIFO` writer"]
pub struct W(crate::W<RXFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFIFO_SPEC>;
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
impl From<crate::W<RXFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO` reader - Receive data. Data is aligned to the LSB (padded zeros on upper bits) unless BIGENDIAN is set."]
pub struct RXFIFO_R(crate::FieldReader<u32, u32>);
impl RXFIFO_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXFIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO` writer - Receive data. Data is aligned to the LSB (padded zeros on upper bits) unless BIGENDIAN is set."]
pub struct RXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive data. Data is aligned to the LSB (padded zeros on upper bits) unless BIGENDIAN is set."]
    #[inline(always)]
    pub fn rxfifo(&self) -> RXFIFO_R {
        RXFIFO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive data. Data is aligned to the LSB (padded zeros on upper bits) unless BIGENDIAN is set."]
    #[inline(always)]
    pub fn rxfifo(&mut self) -> RXFIFO_W {
        RXFIFO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Data FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifo](index.html) module"]
pub struct RXFIFO_SPEC;
impl crate::RegisterSpec for RXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfifo::R](R) reader structure"]
impl crate::Readable for RXFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxfifo::W](W) writer structure"]
impl crate::Writable for RXFIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXFIFO to value 0"]
impl crate::Resettable for RXFIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
