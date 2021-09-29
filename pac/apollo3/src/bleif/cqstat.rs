#[doc = "Register `CQSTAT` reader"]
pub struct R(crate::R<CQSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQSTAT` writer"]
pub struct W(crate::W<CQSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQSTAT_SPEC>;
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
impl From<crate::W<CQSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CQERR` reader - Command queue processing Error. This active high bit signals that an error was encountered during the CQ operation."]
pub struct CQERR_R(crate::FieldReader<bool, bool>);
impl CQERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQERR` writer - Command queue processing Error. This active high bit signals that an error was encountered during the CQ operation."]
pub struct CQERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CQERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CQPAUSED` reader - Command queue operation is currently paused."]
pub struct CQPAUSED_R(crate::FieldReader<bool, bool>);
impl CQPAUSED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQPAUSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQPAUSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQPAUSED` writer - Command queue operation is currently paused."]
pub struct CQPAUSED_W<'a> {
    w: &'a mut W,
}
impl<'a> CQPAUSED_W<'a> {
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
#[doc = "Field `CQTIP` reader - Command queue Transfer In Progress indicator. 1 will indicate that a CQ transfer is active and this will remain active even when paused waiting for external event."]
pub struct CQTIP_R(crate::FieldReader<bool, bool>);
impl CQTIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQTIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQTIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQTIP` writer - Command queue Transfer In Progress indicator. 1 will indicate that a CQ transfer is active and this will remain active even when paused waiting for external event."]
pub struct CQTIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CQTIP_W<'a> {
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
    #[doc = "Bit 2 - Command queue processing Error. This active high bit signals that an error was encountered during the CQ operation."]
    #[inline(always)]
    pub fn cqerr(&self) -> CQERR_R {
        CQERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command queue operation is currently paused."]
    #[inline(always)]
    pub fn cqpaused(&self) -> CQPAUSED_R {
        CQPAUSED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command queue Transfer In Progress indicator. 1 will indicate that a CQ transfer is active and this will remain active even when paused waiting for external event."]
    #[inline(always)]
    pub fn cqtip(&self) -> CQTIP_R {
        CQTIP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Command queue processing Error. This active high bit signals that an error was encountered during the CQ operation."]
    #[inline(always)]
    pub fn cqerr(&mut self) -> CQERR_W {
        CQERR_W { w: self }
    }
    #[doc = "Bit 1 - Command queue operation is currently paused."]
    #[inline(always)]
    pub fn cqpaused(&mut self) -> CQPAUSED_W {
        CQPAUSED_W { w: self }
    }
    #[doc = "Bit 0 - Command queue Transfer In Progress indicator. 1 will indicate that a CQ transfer is active and this will remain active even when paused waiting for external event."]
    #[inline(always)]
    pub fn cqtip(&mut self) -> CQTIP_W {
        CQTIP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqstat](index.html) module"]
pub struct CQSTAT_SPEC;
impl crate::RegisterSpec for CQSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqstat::R](R) reader structure"]
impl crate::Readable for CQSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqstat::W](W) writer structure"]
impl crate::Writable for CQSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQSTAT to value 0"]
impl crate::Resettable for CQSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
