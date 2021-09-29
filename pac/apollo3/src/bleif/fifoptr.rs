#[doc = "Register `FIFOPTR` reader"]
pub struct R(crate::R<FIFOPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOPTR` writer"]
pub struct W(crate::W<FIFOPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOPTR_SPEC>;
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
impl From<crate::W<FIFOPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO1REM` reader - The number of remaining data bytes slots currently in FIFO 1 (written by interface, read by MCU)"]
pub struct FIFO1REM_R(crate::FieldReader<u8, u8>);
impl FIFO1REM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFO1REM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO1REM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO1REM` writer - The number of remaining data bytes slots currently in FIFO 1 (written by interface, read by MCU)"]
pub struct FIFO1REM_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO1REM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `FIFO1SIZ` reader - The number of valid data bytes currently in FIFO 1 (written by interface, read by MCU)"]
pub struct FIFO1SIZ_R(crate::FieldReader<u8, u8>);
impl FIFO1SIZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFO1SIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO1SIZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO1SIZ` writer - The number of valid data bytes currently in FIFO 1 (written by interface, read by MCU)"]
pub struct FIFO1SIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO1SIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `FIFO0REM` reader - The number of remaining data bytes slots currently in FIFO 0 (written by MCU, read by interface)"]
pub struct FIFO0REM_R(crate::FieldReader<u8, u8>);
impl FIFO0REM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFO0REM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO0REM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO0REM` writer - The number of remaining data bytes slots currently in FIFO 0 (written by MCU, read by interface)"]
pub struct FIFO0REM_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO0REM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `FIFO0SIZ` reader - The number of valid data bytes currently in the FIFO 0 (written by MCU, read by interface)"]
pub struct FIFO0SIZ_R(crate::FieldReader<u8, u8>);
impl FIFO0SIZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFO0SIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO0SIZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO0SIZ` writer - The number of valid data bytes currently in the FIFO 0 (written by MCU, read by interface)"]
pub struct FIFO0SIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO0SIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - The number of remaining data bytes slots currently in FIFO 1 (written by interface, read by MCU)"]
    #[inline(always)]
    pub fn fifo1rem(&self) -> FIFO1REM_R {
        FIFO1REM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The number of valid data bytes currently in FIFO 1 (written by interface, read by MCU)"]
    #[inline(always)]
    pub fn fifo1siz(&self) -> FIFO1SIZ_R {
        FIFO1SIZ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The number of remaining data bytes slots currently in FIFO 0 (written by MCU, read by interface)"]
    #[inline(always)]
    pub fn fifo0rem(&self) -> FIFO0REM_R {
        FIFO0REM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - The number of valid data bytes currently in the FIFO 0 (written by MCU, read by interface)"]
    #[inline(always)]
    pub fn fifo0siz(&self) -> FIFO0SIZ_R {
        FIFO0SIZ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - The number of remaining data bytes slots currently in FIFO 1 (written by interface, read by MCU)"]
    #[inline(always)]
    pub fn fifo1rem(&mut self) -> FIFO1REM_W {
        FIFO1REM_W { w: self }
    }
    #[doc = "Bits 16:23 - The number of valid data bytes currently in FIFO 1 (written by interface, read by MCU)"]
    #[inline(always)]
    pub fn fifo1siz(&mut self) -> FIFO1SIZ_W {
        FIFO1SIZ_W { w: self }
    }
    #[doc = "Bits 8:15 - The number of remaining data bytes slots currently in FIFO 0 (written by MCU, read by interface)"]
    #[inline(always)]
    pub fn fifo0rem(&mut self) -> FIFO0REM_W {
        FIFO0REM_W { w: self }
    }
    #[doc = "Bits 0:7 - The number of valid data bytes currently in the FIFO 0 (written by MCU, read by interface)"]
    #[inline(always)]
    pub fn fifo0siz(&mut self) -> FIFO0SIZ_W {
        FIFO0SIZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO size and remaining slots open values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoptr](index.html) module"]
pub struct FIFOPTR_SPEC;
impl crate::RegisterSpec for FIFOPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoptr::R](R) reader structure"]
impl crate::Readable for FIFOPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoptr::W](W) writer structure"]
impl crate::Writable for FIFOPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOPTR to value 0"]
impl crate::Resettable for FIFOPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
