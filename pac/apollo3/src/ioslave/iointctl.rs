#[doc = "Register `IOINTCTL` reader"]
pub struct R(crate::R<IOINTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOINTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOINTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOINTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOINTCTL` writer"]
pub struct W(crate::W<IOINTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOINTCTL_SPEC>;
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
impl From<crate::W<IOINTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOINTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOINTSET` reader - These bits set the IOINT interrupts when written with a 1."]
pub struct IOINTSET_R(crate::FieldReader<u8, u8>);
impl IOINTSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOINTSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOINTSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOINTSET` writer - These bits set the IOINT interrupts when written with a 1."]
pub struct IOINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> IOINTSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `IOINTCLR` reader - This bit clears all of the IOINT interrupts when written with a 1."]
pub struct IOINTCLR_R(crate::FieldReader<bool, bool>);
impl IOINTCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOINTCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOINTCLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOINTCLR` writer - This bit clears all of the IOINT interrupts when written with a 1."]
pub struct IOINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IOINTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `IOINT` reader - These bits read the IOINT interrupts."]
pub struct IOINT_R(crate::FieldReader<u8, u8>);
impl IOINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOINT` writer - These bits read the IOINT interrupts."]
pub struct IOINT_W<'a> {
    w: &'a mut W,
}
impl<'a> IOINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `IOINTEN` reader - These read-only bits indicate whether the IOINT interrupts are enabled."]
pub struct IOINTEN_R(crate::FieldReader<u8, u8>);
impl IOINTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOINTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOINTEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOINTEN` writer - These read-only bits indicate whether the IOINT interrupts are enabled."]
pub struct IOINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOINTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - These bits set the IOINT interrupts when written with a 1."]
    #[inline(always)]
    pub fn iointset(&self) -> IOINTSET_R {
        IOINTSET_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 16 - This bit clears all of the IOINT interrupts when written with a 1."]
    #[inline(always)]
    pub fn iointclr(&self) -> IOINTCLR_R {
        IOINTCLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - These bits read the IOINT interrupts."]
    #[inline(always)]
    pub fn ioint(&self) -> IOINT_R {
        IOINT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - These read-only bits indicate whether the IOINT interrupts are enabled."]
    #[inline(always)]
    pub fn iointen(&self) -> IOINTEN_R {
        IOINTEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - These bits set the IOINT interrupts when written with a 1."]
    #[inline(always)]
    pub fn iointset(&mut self) -> IOINTSET_W {
        IOINTSET_W { w: self }
    }
    #[doc = "Bit 16 - This bit clears all of the IOINT interrupts when written with a 1."]
    #[inline(always)]
    pub fn iointclr(&mut self) -> IOINTCLR_W {
        IOINTCLR_W { w: self }
    }
    #[doc = "Bits 8:15 - These bits read the IOINT interrupts."]
    #[inline(always)]
    pub fn ioint(&mut self) -> IOINT_W {
        IOINT_W { w: self }
    }
    #[doc = "Bits 0:7 - These read-only bits indicate whether the IOINT interrupts are enabled."]
    #[inline(always)]
    pub fn iointen(&mut self) -> IOINTEN_W {
        IOINTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iointctl](index.html) module"]
pub struct IOINTCTL_SPEC;
impl crate::RegisterSpec for IOINTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iointctl::R](R) reader structure"]
impl crate::Readable for IOINTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iointctl::W](W) writer structure"]
impl crate::Writable for IOINTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOINTCTL to value 0"]
impl crate::Resettable for IOINTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
