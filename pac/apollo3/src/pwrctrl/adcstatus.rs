#[doc = "Register `ADCSTATUS` reader"]
pub struct R(crate::R<ADCSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCSTATUS` writer"]
pub struct W(crate::W<ADCSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCSTATUS_SPEC>;
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
impl From<crate::W<ADCSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFBUFPWD` reader - This bit indicates that the ADC REFBUF is powered down"]
pub struct REFBUFPWD_R(crate::FieldReader<bool, bool>);
impl REFBUFPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFBUFPWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFBUFPWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFBUFPWD` writer - This bit indicates that the ADC REFBUF is powered down"]
pub struct REFBUFPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBUFPWD_W<'a> {
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
#[doc = "Field `REFKEEPPWD` reader - This bit indicates that the ADC REFKEEP is powered down"]
pub struct REFKEEPPWD_R(crate::FieldReader<bool, bool>);
impl REFKEEPPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFKEEPPWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFKEEPPWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFKEEPPWD` writer - This bit indicates that the ADC REFKEEP is powered down"]
pub struct REFKEEPPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> REFKEEPPWD_W<'a> {
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
#[doc = "Field `VBATPWD` reader - This bit indicates that the ADC VBAT resistor divider is powered down"]
pub struct VBATPWD_R(crate::FieldReader<bool, bool>);
impl VBATPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBATPWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBATPWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBATPWD` writer - This bit indicates that the ADC VBAT resistor divider is powered down"]
pub struct VBATPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATPWD_W<'a> {
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
#[doc = "Field `VPTATPWD` reader - This bit indicates that the ADC temperature sensor input buffer is powered down"]
pub struct VPTATPWD_R(crate::FieldReader<bool, bool>);
impl VPTATPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        VPTATPWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VPTATPWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VPTATPWD` writer - This bit indicates that the ADC temperature sensor input buffer is powered down"]
pub struct VPTATPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> VPTATPWD_W<'a> {
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
#[doc = "Field `BGTPWD` reader - This bit indicates that the ADC Band Gap is powered down"]
pub struct BGTPWD_R(crate::FieldReader<bool, bool>);
impl BGTPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGTPWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGTPWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGTPWD` writer - This bit indicates that the ADC Band Gap is powered down"]
pub struct BGTPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> BGTPWD_W<'a> {
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
#[doc = "Field `ADCPWD` reader - This bit indicates that the ADC is powered down"]
pub struct ADCPWD_R(crate::FieldReader<bool, bool>);
impl ADCPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCPWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCPWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCPWD` writer - This bit indicates that the ADC is powered down"]
pub struct ADCPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPWD_W<'a> {
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
    #[doc = "Bit 5 - This bit indicates that the ADC REFBUF is powered down"]
    #[inline(always)]
    pub fn refbufpwd(&self) -> REFBUFPWD_R {
        REFBUFPWD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that the ADC REFKEEP is powered down"]
    #[inline(always)]
    pub fn refkeeppwd(&self) -> REFKEEPPWD_R {
        REFKEEPPWD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that the ADC VBAT resistor divider is powered down"]
    #[inline(always)]
    pub fn vbatpwd(&self) -> VBATPWD_R {
        VBATPWD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit indicates that the ADC temperature sensor input buffer is powered down"]
    #[inline(always)]
    pub fn vptatpwd(&self) -> VPTATPWD_R {
        VPTATPWD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit indicates that the ADC Band Gap is powered down"]
    #[inline(always)]
    pub fn bgtpwd(&self) -> BGTPWD_R {
        BGTPWD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit indicates that the ADC is powered down"]
    #[inline(always)]
    pub fn adcpwd(&self) -> ADCPWD_R {
        ADCPWD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - This bit indicates that the ADC REFBUF is powered down"]
    #[inline(always)]
    pub fn refbufpwd(&mut self) -> REFBUFPWD_W {
        REFBUFPWD_W { w: self }
    }
    #[doc = "Bit 4 - This bit indicates that the ADC REFKEEP is powered down"]
    #[inline(always)]
    pub fn refkeeppwd(&mut self) -> REFKEEPPWD_W {
        REFKEEPPWD_W { w: self }
    }
    #[doc = "Bit 3 - This bit indicates that the ADC VBAT resistor divider is powered down"]
    #[inline(always)]
    pub fn vbatpwd(&mut self) -> VBATPWD_W {
        VBATPWD_W { w: self }
    }
    #[doc = "Bit 2 - This bit indicates that the ADC temperature sensor input buffer is powered down"]
    #[inline(always)]
    pub fn vptatpwd(&mut self) -> VPTATPWD_W {
        VPTATPWD_W { w: self }
    }
    #[doc = "Bit 1 - This bit indicates that the ADC Band Gap is powered down"]
    #[inline(always)]
    pub fn bgtpwd(&mut self) -> BGTPWD_W {
        BGTPWD_W { w: self }
    }
    #[doc = "Bit 0 - This bit indicates that the ADC is powered down"]
    #[inline(always)]
    pub fn adcpwd(&mut self) -> ADCPWD_W {
        ADCPWD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Status Register for ADC Block\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcstatus](index.html) module"]
pub struct ADCSTATUS_SPEC;
impl crate::RegisterSpec for ADCSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcstatus::R](R) reader structure"]
impl crate::Readable for ADCSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcstatus::W](W) writer structure"]
impl crate::Writable for ADCSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCSTATUS to value 0x3f"]
impl crate::Resettable for ADCSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
