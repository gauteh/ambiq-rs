#[doc = "Register `BODCTRL` reader"]
pub struct R(crate::R<BODCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BODCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BODCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BODCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BODCTRL` writer"]
pub struct W(crate::W<BODCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BODCTRL_SPEC>;
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
impl From<crate::W<BODCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BODCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BODHVREFSEL` reader - BODH External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
pub struct BODHVREFSEL_R(crate::FieldReader<bool, bool>);
impl BODHVREFSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODHVREFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODHVREFSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODHVREFSEL` writer - BODH External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
pub struct BODHVREFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BODHVREFSEL_W<'a> {
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
#[doc = "Field `BODLVREFSEL` reader - BODL External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
pub struct BODLVREFSEL_R(crate::FieldReader<bool, bool>);
impl BODLVREFSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODLVREFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODLVREFSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODLVREFSEL` writer - BODL External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
pub struct BODLVREFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BODLVREFSEL_W<'a> {
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
#[doc = "Field `BODFPWD` reader - BODF Power Down."]
pub struct BODFPWD_R(crate::FieldReader<bool, bool>);
impl BODFPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODFPWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODFPWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODFPWD` writer - BODF Power Down."]
pub struct BODFPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> BODFPWD_W<'a> {
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
#[doc = "Field `BODCPWD` reader - BODC Power Down."]
pub struct BODCPWD_R(crate::FieldReader<bool, bool>);
impl BODCPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODCPWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODCPWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODCPWD` writer - BODC Power Down."]
pub struct BODCPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> BODCPWD_W<'a> {
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
#[doc = "Field `BODHPWD` reader - BODH Power Down."]
pub struct BODHPWD_R(crate::FieldReader<bool, bool>);
impl BODHPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODHPWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODHPWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODHPWD` writer - BODH Power Down."]
pub struct BODHPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> BODHPWD_W<'a> {
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
#[doc = "Field `BODLPWD` reader - BODL Power Down."]
pub struct BODLPWD_R(crate::FieldReader<bool, bool>);
impl BODLPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODLPWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODLPWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODLPWD` writer - BODL Power Down."]
pub struct BODLPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> BODLPWD_W<'a> {
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
    #[doc = "Bit 5 - BODH External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub fn bodhvrefsel(&self) -> BODHVREFSEL_R {
        BODHVREFSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BODL External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub fn bodlvrefsel(&self) -> BODLVREFSEL_R {
        BODLVREFSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BODF Power Down."]
    #[inline(always)]
    pub fn bodfpwd(&self) -> BODFPWD_R {
        BODFPWD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BODC Power Down."]
    #[inline(always)]
    pub fn bodcpwd(&self) -> BODCPWD_R {
        BODCPWD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - BODH Power Down."]
    #[inline(always)]
    pub fn bodhpwd(&self) -> BODHPWD_R {
        BODHPWD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - BODL Power Down."]
    #[inline(always)]
    pub fn bodlpwd(&self) -> BODLPWD_R {
        BODLPWD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - BODH External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub fn bodhvrefsel(&mut self) -> BODHVREFSEL_W {
        BODHVREFSEL_W { w: self }
    }
    #[doc = "Bit 4 - BODL External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub fn bodlvrefsel(&mut self) -> BODLVREFSEL_W {
        BODLVREFSEL_W { w: self }
    }
    #[doc = "Bit 3 - BODF Power Down."]
    #[inline(always)]
    pub fn bodfpwd(&mut self) -> BODFPWD_W {
        BODFPWD_W { w: self }
    }
    #[doc = "Bit 2 - BODC Power Down."]
    #[inline(always)]
    pub fn bodcpwd(&mut self) -> BODCPWD_W {
        BODCPWD_W { w: self }
    }
    #[doc = "Bit 1 - BODH Power Down."]
    #[inline(always)]
    pub fn bodhpwd(&mut self) -> BODHPWD_W {
        BODHPWD_W { w: self }
    }
    #[doc = "Bit 0 - BODL Power Down."]
    #[inline(always)]
    pub fn bodlpwd(&mut self) -> BODLPWD_W {
        BODLPWD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodctrl](index.html) module"]
pub struct BODCTRL_SPEC;
impl crate::RegisterSpec for BODCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bodctrl::R](R) reader structure"]
impl crate::Readable for BODCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bodctrl::W](W) writer structure"]
impl crate::Writable for BODCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BODCTRL to value 0"]
impl crate::Resettable for BODCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
