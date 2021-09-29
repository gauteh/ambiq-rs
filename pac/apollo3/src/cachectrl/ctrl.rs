#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH1_SLM_ENABLE` reader - Enable Flash Sleep Mode. Write to 1 to put flash 1 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
pub struct FLASH1_SLM_ENABLE_R(crate::FieldReader<bool, bool>);
impl FLASH1_SLM_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH1_SLM_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH1_SLM_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH1_SLM_ENABLE` writer - Enable Flash Sleep Mode. Write to 1 to put flash 1 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
pub struct FLASH1_SLM_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_SLM_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FLASH1_SLM_DISABLE` reader - Disable Flash Sleep Mode. Write 1 to wake flash1 from sleep mode (reading the array will also automatically wake it)."]
pub struct FLASH1_SLM_DISABLE_R(crate::FieldReader<bool, bool>);
impl FLASH1_SLM_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH1_SLM_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH1_SLM_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH1_SLM_DISABLE` writer - Disable Flash Sleep Mode. Write 1 to wake flash1 from sleep mode (reading the array will also automatically wake it)."]
pub struct FLASH1_SLM_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_SLM_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `FLASH1_SLM_STATUS` reader - Flash Sleep Mode Status. 1 indicates that flash1 is in sleep mode, 0 indicates flash1 is in normal mode."]
pub struct FLASH1_SLM_STATUS_R(crate::FieldReader<bool, bool>);
impl FLASH1_SLM_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH1_SLM_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH1_SLM_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH1_SLM_STATUS` writer - Flash Sleep Mode Status. 1 indicates that flash1 is in sleep mode, 0 indicates flash1 is in normal mode."]
pub struct FLASH1_SLM_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_SLM_STATUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `FLASH0_SLM_ENABLE` reader - Enable Flash Sleep Mode. Write to 1 to put flash 0 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
pub struct FLASH0_SLM_ENABLE_R(crate::FieldReader<bool, bool>);
impl FLASH0_SLM_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH0_SLM_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH0_SLM_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH0_SLM_ENABLE` writer - Enable Flash Sleep Mode. Write to 1 to put flash 0 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
pub struct FLASH0_SLM_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_SLM_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `FLASH0_SLM_DISABLE` reader - Disable Flash Sleep Mode. Write 1 to wake flash0 from sleep mode (reading the array will also automatically wake it)."]
pub struct FLASH0_SLM_DISABLE_R(crate::FieldReader<bool, bool>);
impl FLASH0_SLM_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH0_SLM_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH0_SLM_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH0_SLM_DISABLE` writer - Disable Flash Sleep Mode. Write 1 to wake flash0 from sleep mode (reading the array will also automatically wake it)."]
pub struct FLASH0_SLM_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_SLM_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `FLASH0_SLM_STATUS` reader - Flash Sleep Mode Status. 1 indicates that flash0 is in sleep mode, 0 indicates flash0 is in normal mode."]
pub struct FLASH0_SLM_STATUS_R(crate::FieldReader<bool, bool>);
impl FLASH0_SLM_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH0_SLM_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH0_SLM_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH0_SLM_STATUS` writer - Flash Sleep Mode Status. 1 indicates that flash0 is in sleep mode, 0 indicates flash0 is in normal mode."]
pub struct FLASH0_SLM_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_SLM_STATUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CACHE_READY` reader - Cache Ready Status (enabled and not processing an invalidate operation)"]
pub struct CACHE_READY_R(crate::FieldReader<bool, bool>);
impl CACHE_READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_READY` writer - Cache Ready Status (enabled and not processing an invalidate operation)"]
pub struct CACHE_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_READY_W<'a> {
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
#[doc = "Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_STAT_A {
    #[doc = "1: Clear Cache Stats value."]
    CLEAR = 1,
}
impl From<RESET_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET_STAT` reader - Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
pub struct RESET_STAT_R(crate::FieldReader<bool, RESET_STAT_A>);
impl RESET_STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_STAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESET_STAT_A> {
        match self.bits {
            true => Some(RESET_STAT_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == RESET_STAT_A::CLEAR
    }
}
impl core::ops::Deref for RESET_STAT_R {
    type Target = crate::FieldReader<bool, RESET_STAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_STAT` writer - Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
pub struct RESET_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_STAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_STAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear Cache Stats value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RESET_STAT_A::CLEAR)
    }
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
#[doc = "Field `INVALIDATE` reader - Writing a 1 to this bitfield invalidates the flash cache contents."]
pub struct INVALIDATE_R(crate::FieldReader<bool, bool>);
impl INVALIDATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVALIDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVALIDATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVALIDATE` writer - Writing a 1 to this bitfield invalidates the flash cache contents."]
pub struct INVALIDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVALIDATE_W<'a> {
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
    #[doc = "Bit 10 - Enable Flash Sleep Mode. Write to 1 to put flash 1 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline(always)]
    pub fn flash1_slm_enable(&self) -> FLASH1_SLM_ENABLE_R {
        FLASH1_SLM_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Disable Flash Sleep Mode. Write 1 to wake flash1 from sleep mode (reading the array will also automatically wake it)."]
    #[inline(always)]
    pub fn flash1_slm_disable(&self) -> FLASH1_SLM_DISABLE_R {
        FLASH1_SLM_DISABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash Sleep Mode Status. 1 indicates that flash1 is in sleep mode, 0 indicates flash1 is in normal mode."]
    #[inline(always)]
    pub fn flash1_slm_status(&self) -> FLASH1_SLM_STATUS_R {
        FLASH1_SLM_STATUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Flash Sleep Mode. Write to 1 to put flash 0 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline(always)]
    pub fn flash0_slm_enable(&self) -> FLASH0_SLM_ENABLE_R {
        FLASH0_SLM_ENABLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable Flash Sleep Mode. Write 1 to wake flash0 from sleep mode (reading the array will also automatically wake it)."]
    #[inline(always)]
    pub fn flash0_slm_disable(&self) -> FLASH0_SLM_DISABLE_R {
        FLASH0_SLM_DISABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash Sleep Mode Status. 1 indicates that flash0 is in sleep mode, 0 indicates flash0 is in normal mode."]
    #[inline(always)]
    pub fn flash0_slm_status(&self) -> FLASH0_SLM_STATUS_R {
        FLASH0_SLM_STATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cache Ready Status (enabled and not processing an invalidate operation)"]
    #[inline(always)]
    pub fn cache_ready(&self) -> CACHE_READY_R {
        CACHE_READY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
    #[inline(always)]
    pub fn reset_stat(&self) -> RESET_STAT_R {
        RESET_STAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Writing a 1 to this bitfield invalidates the flash cache contents."]
    #[inline(always)]
    pub fn invalidate(&self) -> INVALIDATE_R {
        INVALIDATE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Enable Flash Sleep Mode. Write to 1 to put flash 1 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline(always)]
    pub fn flash1_slm_enable(&mut self) -> FLASH1_SLM_ENABLE_W {
        FLASH1_SLM_ENABLE_W { w: self }
    }
    #[doc = "Bit 9 - Disable Flash Sleep Mode. Write 1 to wake flash1 from sleep mode (reading the array will also automatically wake it)."]
    #[inline(always)]
    pub fn flash1_slm_disable(&mut self) -> FLASH1_SLM_DISABLE_W {
        FLASH1_SLM_DISABLE_W { w: self }
    }
    #[doc = "Bit 8 - Flash Sleep Mode Status. 1 indicates that flash1 is in sleep mode, 0 indicates flash1 is in normal mode."]
    #[inline(always)]
    pub fn flash1_slm_status(&mut self) -> FLASH1_SLM_STATUS_W {
        FLASH1_SLM_STATUS_W { w: self }
    }
    #[doc = "Bit 6 - Enable Flash Sleep Mode. Write to 1 to put flash 0 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline(always)]
    pub fn flash0_slm_enable(&mut self) -> FLASH0_SLM_ENABLE_W {
        FLASH0_SLM_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - Disable Flash Sleep Mode. Write 1 to wake flash0 from sleep mode (reading the array will also automatically wake it)."]
    #[inline(always)]
    pub fn flash0_slm_disable(&mut self) -> FLASH0_SLM_DISABLE_W {
        FLASH0_SLM_DISABLE_W { w: self }
    }
    #[doc = "Bit 4 - Flash Sleep Mode Status. 1 indicates that flash0 is in sleep mode, 0 indicates flash0 is in normal mode."]
    #[inline(always)]
    pub fn flash0_slm_status(&mut self) -> FLASH0_SLM_STATUS_W {
        FLASH0_SLM_STATUS_W { w: self }
    }
    #[doc = "Bit 2 - Cache Ready Status (enabled and not processing an invalidate operation)"]
    #[inline(always)]
    pub fn cache_ready(&mut self) -> CACHE_READY_W {
        CACHE_READY_W { w: self }
    }
    #[doc = "Bit 1 - Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
    #[inline(always)]
    pub fn reset_stat(&mut self) -> RESET_STAT_W {
        RESET_STAT_W { w: self }
    }
    #[doc = "Bit 0 - Writing a 1 to this bitfield invalidates the flash cache contents."]
    #[inline(always)]
    pub fn invalidate(&mut self) -> INVALIDATE_W {
        INVALIDATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
