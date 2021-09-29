#[doc = "Register `SNVR0` reader"]
pub struct R(crate::R<SNVR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNVR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNVR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNVR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNVR0` writer"]
pub struct W(crate::W<SNVR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNVR0_SPEC>;
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
impl From<crate::W<SNVR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNVR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNVR0` reader - Value of the 32-bit counter as it ticks over."]
pub struct SNVR0_R(crate::FieldReader<u32, u32>);
impl SNVR0_R {
    pub(crate) fn new(bits: u32) -> Self {
        SNVR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNVR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNVR0` writer - Value of the 32-bit counter as it ticks over."]
pub struct SNVR0_W<'a> {
    w: &'a mut W,
}
impl<'a> SNVR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    pub fn snvr0(&self) -> SNVR0_R {
        SNVR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    pub fn snvr0(&mut self) -> SNVR0_W {
        SNVR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Timer NVRAM_A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snvr0](index.html) module"]
pub struct SNVR0_SPEC;
impl crate::RegisterSpec for SNVR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [snvr0::R](R) reader structure"]
impl crate::Readable for SNVR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snvr0::W](W) writer structure"]
impl crate::Writable for SNVR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SNVR0 to value 0"]
impl crate::Resettable for SNVR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
