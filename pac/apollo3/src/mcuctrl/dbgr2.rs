#[doc = "Register `DBGR2` reader"]
pub struct R(crate::R<DBGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGR2` writer"]
pub struct W(crate::W<DBGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGR2_SPEC>;
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
impl From<crate::W<DBGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COOLCODE` reader - Read-only register for communication validation"]
pub struct COOLCODE_R(crate::FieldReader<u32, u32>);
impl COOLCODE_R {
    pub(crate) fn new(bits: u32) -> Self {
        COOLCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COOLCODE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COOLCODE` writer - Read-only register for communication validation"]
pub struct COOLCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COOLCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Read-only register for communication validation"]
    #[inline(always)]
    pub fn coolcode(&self) -> COOLCODE_R {
        COOLCODE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read-only register for communication validation"]
    #[inline(always)]
    pub fn coolcode(&mut self) -> COOLCODE_W {
        COOLCODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read-only debug register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgr2](index.html) module"]
pub struct DBGR2_SPEC;
impl crate::RegisterSpec for DBGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgr2::R](R) reader structure"]
impl crate::Readable for DBGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgr2::W](W) writer structure"]
impl crate::Writable for DBGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBGR2 to value 0xc001_c0de"]
impl crate::Resettable for DBGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc001_c0de
    }
}
