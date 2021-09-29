#[doc = "Register `CQFLAGS` reader"]
pub struct R(crate::R<CQFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQFLAGS` writer"]
pub struct W(crate::W<CQFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQFLAGS_SPEC>;
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
impl From<crate::W<CQFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CQIRQMASK` reader - Mask the bits used to generate the command queue interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE"]
pub struct CQIRQMASK_R(crate::FieldReader<u16, u16>);
impl CQIRQMASK_R {
    pub(crate) fn new(bits: u16) -> Self {
        CQIRQMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQIRQMASK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQIRQMASK` writer - Mask the bits used to generate the command queue interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE"]
pub struct CQIRQMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CQIRQMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CQFLAGS` reader - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
pub struct CQFLAGS_R(crate::FieldReader<u16, u16>);
impl CQFLAGS_R {
    pub(crate) fn new(bits: u16) -> Self {
        CQFLAGS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQFLAGS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQFLAGS` writer - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
pub struct CQFLAGS_W<'a> {
    w: &'a mut W,
}
impl<'a> CQFLAGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Mask the bits used to generate the command queue interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE"]
    #[inline(always)]
    pub fn cqirqmask(&self) -> CQIRQMASK_R {
        CQIRQMASK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
    #[inline(always)]
    pub fn cqflags(&self) -> CQFLAGS_R {
        CQFLAGS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Mask the bits used to generate the command queue interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE"]
    #[inline(always)]
    pub fn cqirqmask(&mut self) -> CQIRQMASK_W {
        CQIRQMASK_W { w: self }
    }
    #[doc = "Bits 0:15 - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
    #[inline(always)]
    pub fn cqflags(&mut self) -> CQFLAGS_W {
        CQFLAGS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Queue Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqflags](index.html) module"]
pub struct CQFLAGS_SPEC;
impl crate::RegisterSpec for CQFLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqflags::R](R) reader structure"]
impl crate::Readable for CQFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqflags::W](W) writer structure"]
impl crate::Writable for CQFLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQFLAGS to value 0"]
impl crate::Resettable for CQFLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
