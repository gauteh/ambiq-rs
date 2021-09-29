#[doc = "Register `CQADDR` reader"]
pub struct R(crate::R<CQADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQADDR` writer"]
pub struct W(crate::W<CQADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQADDR_SPEC>;
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
impl From<crate::W<CQADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CQADDR28` reader - Bit 28 of target byte address for source of CQ (read only). Used to denote Flash (0) or SRAM (1) access"]
pub struct CQADDR28_R(crate::FieldReader<bool, bool>);
impl CQADDR28_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQADDR28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQADDR28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQADDR28` writer - Bit 28 of target byte address for source of CQ (read only). Used to denote Flash (0) or SRAM (1) access"]
pub struct CQADDR28_W<'a> {
    w: &'a mut W,
}
impl<'a> CQADDR28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CQADDR` reader - Bits 19:2 of target byte address for source of CQ (read only). The buffer must be aligned on a word boundary"]
pub struct CQADDR_R(crate::FieldReader<u32, u32>);
impl CQADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CQADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQADDR` writer - Bits 19:2 of target byte address for source of CQ (read only). The buffer must be aligned on a word boundary"]
pub struct CQADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CQADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 2)) | ((value as u32 & 0x0003_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 28 - Bit 28 of target byte address for source of CQ (read only). Used to denote Flash (0) or SRAM (1) access"]
    #[inline(always)]
    pub fn cqaddr28(&self) -> CQADDR28_R {
        CQADDR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 2:19 - Bits 19:2 of target byte address for source of CQ (read only). The buffer must be aligned on a word boundary"]
    #[inline(always)]
    pub fn cqaddr(&self) -> CQADDR_R {
        CQADDR_R::new(((self.bits >> 2) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 28 - Bit 28 of target byte address for source of CQ (read only). Used to denote Flash (0) or SRAM (1) access"]
    #[inline(always)]
    pub fn cqaddr28(&mut self) -> CQADDR28_W {
        CQADDR28_W { w: self }
    }
    #[doc = "Bits 2:19 - Bits 19:2 of target byte address for source of CQ (read only). The buffer must be aligned on a word boundary"]
    #[inline(always)]
    pub fn cqaddr(&mut self) -> CQADDR_W {
        CQADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CQ Target Read Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqaddr](index.html) module"]
pub struct CQADDR_SPEC;
impl crate::RegisterSpec for CQADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqaddr::R](R) reader structure"]
impl crate::Readable for CQADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqaddr::W](W) writer structure"]
impl crate::Writable for CQADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQADDR to value 0"]
impl crate::Resettable for CQADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
