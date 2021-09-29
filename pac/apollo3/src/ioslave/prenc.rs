#[doc = "Register `PRENC` reader"]
pub struct R(crate::R<PRENC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRENC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRENC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRENC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRENC` writer"]
pub struct W(crate::W<PRENC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRENC_SPEC>;
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
impl From<crate::W<PRENC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRENC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRENC` reader - These bits hold the priority encode of the REGACC interrupts."]
pub struct PRENC_R(crate::FieldReader<u8, u8>);
impl PRENC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRENC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRENC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRENC` writer - These bits hold the priority encode of the REGACC interrupts."]
pub struct PRENC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRENC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - These bits hold the priority encode of the REGACC interrupts."]
    #[inline(always)]
    pub fn prenc(&self) -> PRENC_R {
        PRENC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - These bits hold the priority encode of the REGACC interrupts."]
    #[inline(always)]
    pub fn prenc(&mut self) -> PRENC_W {
        PRENC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Slave Interrupt Priority Encode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prenc](index.html) module"]
pub struct PRENC_SPEC;
impl crate::RegisterSpec for PRENC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prenc::R](R) reader structure"]
impl crate::Readable for PRENC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prenc::W](W) writer structure"]
impl crate::Writable for PRENC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRENC to value 0"]
impl crate::Resettable for PRENC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
