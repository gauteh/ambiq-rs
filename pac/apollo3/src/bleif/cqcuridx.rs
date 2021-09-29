#[doc = "Register `CQCURIDX` reader"]
pub struct R(crate::R<CQCURIDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQCURIDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQCURIDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQCURIDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQCURIDX` writer"]
pub struct W(crate::W<CQCURIDX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQCURIDX_SPEC>;
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
impl From<crate::W<CQCURIDX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQCURIDX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CQCURIDX` reader - Holds 8 bits of data that will be compared with the CQENDIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command quue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
pub struct CQCURIDX_R(crate::FieldReader<u8, u8>);
impl CQCURIDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        CQCURIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQCURIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQCURIDX` writer - Holds 8 bits of data that will be compared with the CQENDIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command quue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
pub struct CQCURIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CQCURIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Holds 8 bits of data that will be compared with the CQENDIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command quue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
    #[inline(always)]
    pub fn cqcuridx(&self) -> CQCURIDX_R {
        CQCURIDX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Holds 8 bits of data that will be compared with the CQENDIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command quue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
    #[inline(always)]
    pub fn cqcuridx(&mut self) -> CQCURIDX_W {
        CQCURIDX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOM Command Queue current index value . Compared to the CQENDIDX reg contents to generate the IDXEQ Pause event for command queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcuridx](index.html) module"]
pub struct CQCURIDX_SPEC;
impl crate::RegisterSpec for CQCURIDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqcuridx::R](R) reader structure"]
impl crate::Readable for CQCURIDX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqcuridx::W](W) writer structure"]
impl crate::Writable for CQCURIDX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQCURIDX to value 0"]
impl crate::Resettable for CQCURIDX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
