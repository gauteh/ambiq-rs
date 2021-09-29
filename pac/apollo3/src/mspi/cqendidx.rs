#[doc = "Register `CQENDIDX` reader"]
pub struct R(crate::R<CQENDIDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQENDIDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQENDIDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQENDIDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQENDIDX` writer"]
pub struct W(crate::W<CQENDIDX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQENDIDX_SPEC>;
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
impl From<crate::W<CQENDIDX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQENDIDX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CQENDIDX` reader - Can be used to indicate the end position of the command queue. A CQ hardware status bit indices when CURIDX != ENDIDX so that the CQ can be paused when it reaches the end pointer."]
pub struct CQENDIDX_R(crate::FieldReader<u8, u8>);
impl CQENDIDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        CQENDIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQENDIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQENDIDX` writer - Can be used to indicate the end position of the command queue. A CQ hardware status bit indices when CURIDX != ENDIDX so that the CQ can be paused when it reaches the end pointer."]
pub struct CQENDIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CQENDIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Can be used to indicate the end position of the command queue. A CQ hardware status bit indices when CURIDX != ENDIDX so that the CQ can be paused when it reaches the end pointer."]
    #[inline(always)]
    pub fn cqendidx(&self) -> CQENDIDX_R {
        CQENDIDX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Can be used to indicate the end position of the command queue. A CQ hardware status bit indices when CURIDX != ENDIDX so that the CQ can be paused when it reaches the end pointer."]
    #[inline(always)]
    pub fn cqendidx(&mut self) -> CQENDIDX_W {
        CQENDIDX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Queue End Index\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqendidx](index.html) module"]
pub struct CQENDIDX_SPEC;
impl crate::RegisterSpec for CQENDIDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqendidx::R](R) reader structure"]
impl crate::Readable for CQENDIDX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqendidx::W](W) writer structure"]
impl crate::Writable for CQENDIDX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQENDIDX to value 0"]
impl crate::Resettable for CQENDIDX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
