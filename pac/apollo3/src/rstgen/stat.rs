#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBOOT` reader - Set when booting securely (SBL)."]
pub struct SBOOT_R(crate::FieldReader<bool, bool>);
impl SBOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBOOT` writer - Set when booting securely (SBL)."]
pub struct SBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> SBOOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `FBOOT` reader - Set if current boot was initiated by soft reset and resulted in Fast Boot (SBL)."]
pub struct FBOOT_R(crate::FieldReader<bool, bool>);
impl FBOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FBOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBOOT` writer - Set if current boot was initiated by soft reset and resulted in Fast Boot (SBL)."]
pub struct FBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> FBOOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `BOBSTAT` reader - A BLE/Burst Regulator Brownout Event occurred (SBL)."]
pub struct BOBSTAT_R(crate::FieldReader<bool, bool>);
impl BOBSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOBSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOBSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOBSTAT` writer - A BLE/Burst Regulator Brownout Event occurred (SBL)."]
pub struct BOBSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOBSTAT_W<'a> {
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
#[doc = "Field `BOFSTAT` reader - A Memory Regulator Brownout Event occurred (SBL)."]
pub struct BOFSTAT_R(crate::FieldReader<bool, bool>);
impl BOFSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOFSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOFSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOFSTAT` writer - A Memory Regulator Brownout Event occurred (SBL)."]
pub struct BOFSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFSTAT_W<'a> {
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
#[doc = "Field `BOCSTAT` reader - A Core Regulator Brownout Event occurred (SBL)."]
pub struct BOCSTAT_R(crate::FieldReader<bool, bool>);
impl BOCSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOCSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOCSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOCSTAT` writer - A Core Regulator Brownout Event occurred (SBL)."]
pub struct BOCSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOCSTAT_W<'a> {
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
#[doc = "Field `BOUSTAT` reader - An Unregulated Supply Brownout Event occurred (SBL)."]
pub struct BOUSTAT_R(crate::FieldReader<bool, bool>);
impl BOUSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOUSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOUSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOUSTAT` writer - An Unregulated Supply Brownout Event occurred (SBL)."]
pub struct BOUSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOUSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `WDRSTAT` reader - Reset was initiated by a Watchdog Timer Reset (SBL)."]
pub struct WDRSTAT_R(crate::FieldReader<bool, bool>);
impl WDRSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDRSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDRSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDRSTAT` writer - Reset was initiated by a Watchdog Timer Reset (SBL)."]
pub struct WDRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRSTAT_W<'a> {
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
#[doc = "Field `DBGRSTAT` reader - Reset was a initiated by Debugger Reset (SBL)."]
pub struct DBGRSTAT_R(crate::FieldReader<bool, bool>);
impl DBGRSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGRSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGRSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGRSTAT` writer - Reset was a initiated by Debugger Reset (SBL)."]
pub struct DBGRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGRSTAT_W<'a> {
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
#[doc = "Field `POIRSTAT` reader - Reset was a initiated by Software POI Reset (SBL)."]
pub struct POIRSTAT_R(crate::FieldReader<bool, bool>);
impl POIRSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        POIRSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POIRSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POIRSTAT` writer - Reset was a initiated by Software POI Reset (SBL)."]
pub struct POIRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> POIRSTAT_W<'a> {
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
#[doc = "Field `SWRSTAT` reader - Reset was a initiated by SW POR or AIRCR Reset (SBL)."]
pub struct SWRSTAT_R(crate::FieldReader<bool, bool>);
impl SWRSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRSTAT` writer - Reset was a initiated by SW POR or AIRCR Reset (SBL)."]
pub struct SWRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `BORSTAT` reader - Reset was initiated by a Brown-Out Reset (SBL)."]
pub struct BORSTAT_R(crate::FieldReader<bool, bool>);
impl BORSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BORSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BORSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BORSTAT` writer - Reset was initiated by a Brown-Out Reset (SBL)."]
pub struct BORSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BORSTAT_W<'a> {
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
#[doc = "Field `PORSTAT` reader - Reset was initiated by a Power-On Reset (SBL)."]
pub struct PORSTAT_R(crate::FieldReader<bool, bool>);
impl PORSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORSTAT` writer - Reset was initiated by a Power-On Reset (SBL)."]
pub struct PORSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORSTAT_W<'a> {
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
#[doc = "Field `EXRSTAT` reader - Reset was initiated by an External Reset (SBL)."]
pub struct EXRSTAT_R(crate::FieldReader<bool, bool>);
impl EXRSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXRSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXRSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXRSTAT` writer - Reset was initiated by an External Reset (SBL)."]
pub struct EXRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXRSTAT_W<'a> {
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
    #[doc = "Bit 31 - Set when booting securely (SBL)."]
    #[inline(always)]
    pub fn sboot(&self) -> SBOOT_R {
        SBOOT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Set if current boot was initiated by soft reset and resulted in Fast Boot (SBL)."]
    #[inline(always)]
    pub fn fboot(&self) -> FBOOT_R {
        FBOOT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 10 - A BLE/Burst Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn bobstat(&self) -> BOBSTAT_R {
        BOBSTAT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - A Memory Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn bofstat(&self) -> BOFSTAT_R {
        BOFSTAT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - A Core Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn bocstat(&self) -> BOCSTAT_R {
        BOCSTAT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - An Unregulated Supply Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn boustat(&self) -> BOUSTAT_R {
        BOUSTAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reset was initiated by a Watchdog Timer Reset (SBL)."]
    #[inline(always)]
    pub fn wdrstat(&self) -> WDRSTAT_R {
        WDRSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reset was a initiated by Debugger Reset (SBL)."]
    #[inline(always)]
    pub fn dbgrstat(&self) -> DBGRSTAT_R {
        DBGRSTAT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset was a initiated by Software POI Reset (SBL)."]
    #[inline(always)]
    pub fn poirstat(&self) -> POIRSTAT_R {
        POIRSTAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset was a initiated by SW POR or AIRCR Reset (SBL)."]
    #[inline(always)]
    pub fn swrstat(&self) -> SWRSTAT_R {
        SWRSTAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset was initiated by a Brown-Out Reset (SBL)."]
    #[inline(always)]
    pub fn borstat(&self) -> BORSTAT_R {
        BORSTAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset was initiated by a Power-On Reset (SBL)."]
    #[inline(always)]
    pub fn porstat(&self) -> PORSTAT_R {
        PORSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Reset was initiated by an External Reset (SBL)."]
    #[inline(always)]
    pub fn exrstat(&self) -> EXRSTAT_R {
        EXRSTAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Set when booting securely (SBL)."]
    #[inline(always)]
    pub fn sboot(&mut self) -> SBOOT_W {
        SBOOT_W { w: self }
    }
    #[doc = "Bit 30 - Set if current boot was initiated by soft reset and resulted in Fast Boot (SBL)."]
    #[inline(always)]
    pub fn fboot(&mut self) -> FBOOT_W {
        FBOOT_W { w: self }
    }
    #[doc = "Bit 10 - A BLE/Burst Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn bobstat(&mut self) -> BOBSTAT_W {
        BOBSTAT_W { w: self }
    }
    #[doc = "Bit 9 - A Memory Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn bofstat(&mut self) -> BOFSTAT_W {
        BOFSTAT_W { w: self }
    }
    #[doc = "Bit 8 - A Core Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn bocstat(&mut self) -> BOCSTAT_W {
        BOCSTAT_W { w: self }
    }
    #[doc = "Bit 7 - An Unregulated Supply Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub fn boustat(&mut self) -> BOUSTAT_W {
        BOUSTAT_W { w: self }
    }
    #[doc = "Bit 6 - Reset was initiated by a Watchdog Timer Reset (SBL)."]
    #[inline(always)]
    pub fn wdrstat(&mut self) -> WDRSTAT_W {
        WDRSTAT_W { w: self }
    }
    #[doc = "Bit 5 - Reset was a initiated by Debugger Reset (SBL)."]
    #[inline(always)]
    pub fn dbgrstat(&mut self) -> DBGRSTAT_W {
        DBGRSTAT_W { w: self }
    }
    #[doc = "Bit 4 - Reset was a initiated by Software POI Reset (SBL)."]
    #[inline(always)]
    pub fn poirstat(&mut self) -> POIRSTAT_W {
        POIRSTAT_W { w: self }
    }
    #[doc = "Bit 3 - Reset was a initiated by SW POR or AIRCR Reset (SBL)."]
    #[inline(always)]
    pub fn swrstat(&mut self) -> SWRSTAT_W {
        SWRSTAT_W { w: self }
    }
    #[doc = "Bit 2 - Reset was initiated by a Brown-Out Reset (SBL)."]
    #[inline(always)]
    pub fn borstat(&mut self) -> BORSTAT_W {
        BORSTAT_W { w: self }
    }
    #[doc = "Bit 1 - Reset was initiated by a Power-On Reset (SBL)."]
    #[inline(always)]
    pub fn porstat(&mut self) -> PORSTAT_W {
        PORSTAT_W { w: self }
    }
    #[doc = "Bit 0 - Reset was initiated by an External Reset (SBL)."]
    #[inline(always)]
    pub fn exrstat(&mut self) -> EXRSTAT_W {
        EXRSTAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register (SBL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
