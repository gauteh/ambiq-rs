#[doc = "Register `FUPD` reader"]
pub struct R(crate::R<FUPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUPD` writer"]
pub struct W(crate::W<FUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUPD_SPEC>;
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
impl From<crate::W<FUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOREAD` reader - This bitfield indicates an IO read is active."]
pub struct IOREAD_R(crate::FieldReader<bool, bool>);
impl IOREAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOREAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOREAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOREAD` writer - This bitfield indicates an IO read is active."]
pub struct IOREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> IOREAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `FIFOUPD` reader - This bit indicates that a FIFO update is underway."]
pub struct FIFOUPD_R(crate::FieldReader<bool, bool>);
impl FIFOUPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFOUPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOUPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOUPD` writer - This bit indicates that a FIFO update is underway."]
pub struct FIFOUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOUPD_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - This bitfield indicates an IO read is active."]
    #[inline(always)]
    pub fn ioread(&self) -> IOREAD_R {
        IOREAD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit indicates that a FIFO update is underway."]
    #[inline(always)]
    pub fn fifoupd(&self) -> FIFOUPD_R {
        FIFOUPD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bitfield indicates an IO read is active."]
    #[inline(always)]
    pub fn ioread(&mut self) -> IOREAD_W {
        IOREAD_W { w: self }
    }
    #[doc = "Bit 0 - This bit indicates that a FIFO update is underway."]
    #[inline(always)]
    pub fn fifoupd(&mut self) -> FIFOUPD_W {
        FIFOUPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Update Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fupd](index.html) module"]
pub struct FUPD_SPEC;
impl crate::RegisterSpec for FUPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fupd::R](R) reader structure"]
impl crate::Readable for FUPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fupd::W](W) writer structure"]
impl crate::Writable for FUPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUPD to value 0"]
impl crate::Resettable for FUPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
