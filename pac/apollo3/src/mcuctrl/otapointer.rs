#[doc = "Register `OTAPOINTER` reader"]
pub struct R(crate::R<OTAPOINTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTAPOINTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTAPOINTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTAPOINTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTAPOINTER` writer"]
pub struct W(crate::W<OTAPOINTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTAPOINTER_SPEC>;
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
impl From<crate::W<OTAPOINTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTAPOINTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTAPOINTER` reader - Flash page pointer with updated OTA image"]
pub struct OTAPOINTER_R(crate::FieldReader<u32, u32>);
impl OTAPOINTER_R {
    pub(crate) fn new(bits: u32) -> Self {
        OTAPOINTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTAPOINTER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTAPOINTER` writer - Flash page pointer with updated OTA image"]
pub struct OTAPOINTER_W<'a> {
    w: &'a mut W,
}
impl<'a> OTAPOINTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Field `OTASBLUPDATE` reader - Indicates that the sbl_init has been updated"]
pub struct OTASBLUPDATE_R(crate::FieldReader<bool, bool>);
impl OTASBLUPDATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTASBLUPDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTASBLUPDATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTASBLUPDATE` writer - Indicates that the sbl_init has been updated"]
pub struct OTASBLUPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> OTASBLUPDATE_W<'a> {
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
#[doc = "Field `OTAVALID` reader - Indicates that an OTA update is valid"]
pub struct OTAVALID_R(crate::FieldReader<bool, bool>);
impl OTAVALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTAVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTAVALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTAVALID` writer - Indicates that an OTA update is valid"]
pub struct OTAVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> OTAVALID_W<'a> {
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
    #[doc = "Bits 2:31 - Flash page pointer with updated OTA image"]
    #[inline(always)]
    pub fn otapointer(&self) -> OTAPOINTER_R {
        OTAPOINTER_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - Indicates that the sbl_init has been updated"]
    #[inline(always)]
    pub fn otasblupdate(&self) -> OTASBLUPDATE_R {
        OTASBLUPDATE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates that an OTA update is valid"]
    #[inline(always)]
    pub fn otavalid(&self) -> OTAVALID_R {
        OTAVALID_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - Flash page pointer with updated OTA image"]
    #[inline(always)]
    pub fn otapointer(&mut self) -> OTAPOINTER_W {
        OTAPOINTER_W { w: self }
    }
    #[doc = "Bit 1 - Indicates that the sbl_init has been updated"]
    #[inline(always)]
    pub fn otasblupdate(&mut self) -> OTASBLUPDATE_W {
        OTASBLUPDATE_W { w: self }
    }
    #[doc = "Bit 0 - Indicates that an OTA update is valid"]
    #[inline(always)]
    pub fn otavalid(&mut self) -> OTAVALID_W {
        OTAVALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTA (Over the Air) Update Pointer/Status. Reset only by POA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otapointer](index.html) module"]
pub struct OTAPOINTER_SPEC;
impl crate::RegisterSpec for OTAPOINTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otapointer::R](R) reader structure"]
impl crate::Readable for OTAPOINTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otapointer::W](W) writer structure"]
impl crate::Writable for OTAPOINTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTAPOINTER to value 0"]
impl crate::Resettable for OTAPOINTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
