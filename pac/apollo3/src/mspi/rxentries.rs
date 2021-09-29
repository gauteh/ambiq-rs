#[doc = "Register `RXENTRIES` reader"]
pub struct R(crate::R<RXENTRIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXENTRIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXENTRIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXENTRIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXENTRIES` writer"]
pub struct W(crate::W<RXENTRIES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXENTRIES_SPEC>;
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
impl From<crate::W<RXENTRIES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXENTRIES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXENTRIES` reader - Number of 32-bit words/entries in RX FIFO"]
pub struct RXENTRIES_R(crate::FieldReader<u8, u8>);
impl RXENTRIES_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXENTRIES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXENTRIES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXENTRIES` writer - Number of 32-bit words/entries in RX FIFO"]
pub struct RXENTRIES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENTRIES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of 32-bit words/entries in RX FIFO"]
    #[inline(always)]
    pub fn rxentries(&self) -> RXENTRIES_R {
        RXENTRIES_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 32-bit words/entries in RX FIFO"]
    #[inline(always)]
    pub fn rxentries(&mut self) -> RXENTRIES_W {
        RXENTRIES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX FIFO Entries\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxentries](index.html) module"]
pub struct RXENTRIES_SPEC;
impl crate::RegisterSpec for RXENTRIES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxentries::R](R) reader structure"]
impl crate::Readable for RXENTRIES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxentries::W](W) writer structure"]
impl crate::Writable for RXENTRIES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXENTRIES to value 0"]
impl crate::Resettable for RXENTRIES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
