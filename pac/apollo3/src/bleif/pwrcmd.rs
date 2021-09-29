#[doc = "Register `PWRCMD` reader"]
pub struct R(crate::R<PWRCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCMD` writer"]
pub struct W(crate::W<PWRCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCMD_SPEC>;
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
impl From<crate::W<PWRCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESTART` reader - Restart the BLE Core after going into the shutdown state. Only valid when in the shutdown state."]
pub struct RESTART_R(crate::FieldReader<bool, bool>);
impl RESTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESTART` writer - Restart the BLE Core after going into the shutdown state. Only valid when in the shutdown state."]
pub struct RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTART_W<'a> {
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
#[doc = "Field `WAKEREQ` reader - Wake request from the MCU. When asserted (1), the BLE Interface logic will assert the wakeup request signal to the BLE Core. Only recognized when in the sleep state"]
pub struct WAKEREQ_R(crate::FieldReader<bool, bool>);
impl WAKEREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEREQ` writer - Wake request from the MCU. When asserted (1), the BLE Interface logic will assert the wakeup request signal to the BLE Core. Only recognized when in the sleep state"]
pub struct WAKEREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEREQ_W<'a> {
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
    #[doc = "Bit 1 - Restart the BLE Core after going into the shutdown state. Only valid when in the shutdown state."]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Wake request from the MCU. When asserted (1), the BLE Interface logic will assert the wakeup request signal to the BLE Core. Only recognized when in the sleep state"]
    #[inline(always)]
    pub fn wakereq(&self) -> WAKEREQ_R {
        WAKEREQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Restart the BLE Core after going into the shutdown state. Only valid when in the shutdown state."]
    #[inline(always)]
    pub fn restart(&mut self) -> RESTART_W {
        RESTART_W { w: self }
    }
    #[doc = "Bit 0 - Wake request from the MCU. When asserted (1), the BLE Interface logic will assert the wakeup request signal to the BLE Core. Only recognized when in the sleep state"]
    #[inline(always)]
    pub fn wakereq(&mut self) -> WAKEREQ_W {
        WAKEREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLE Power command interface\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcmd](index.html) module"]
pub struct PWRCMD_SPEC;
impl crate::RegisterSpec for PWRCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrcmd::R](R) reader structure"]
impl crate::Readable for PWRCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrcmd::W](W) writer structure"]
impl crate::Writable for PWRCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRCMD to value 0"]
impl crate::Resettable for PWRCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
