#[doc = "Register `DBGR1` reader"]
pub struct R(crate::R<DBGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGR1` writer"]
pub struct W(crate::W<DBGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGR1_SPEC>;
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
impl From<crate::W<DBGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ONETO8` reader - Read-only register for communication validation"]
pub struct ONETO8_R(crate::FieldReader<u32, u32>);
impl ONETO8_R {
    pub(crate) fn new(bits: u32) -> Self {
        ONETO8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONETO8_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONETO8` writer - Read-only register for communication validation"]
pub struct ONETO8_W<'a> {
    w: &'a mut W,
}
impl<'a> ONETO8_W<'a> {
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
    pub fn oneto8(&self) -> ONETO8_R {
        ONETO8_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read-only register for communication validation"]
    #[inline(always)]
    pub fn oneto8(&mut self) -> ONETO8_W {
        ONETO8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read-only debug register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgr1](index.html) module"]
pub struct DBGR1_SPEC;
impl crate::RegisterSpec for DBGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgr1::R](R) reader structure"]
impl crate::Readable for DBGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgr1::W](W) writer structure"]
impl crate::Writable for DBGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBGR1 to value 0x1234_5678"]
impl crate::Resettable for DBGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1234_5678
    }
}
