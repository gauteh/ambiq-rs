#[doc = "Register `CQCFG` reader"]
pub struct R(crate::R<CQCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQCFG` writer"]
pub struct W(crate::W<CQCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQCFG_SPEC>;
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
impl From<crate::W<CQCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CQAUTOCLEARMASK` reader - Eanble clear of CQMASK after each pause operation. This may be useful when using software flags to pause CQ."]
pub struct CQAUTOCLEARMASK_R(crate::FieldReader<bool, bool>);
impl CQAUTOCLEARMASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQAUTOCLEARMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQAUTOCLEARMASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQAUTOCLEARMASK` writer - Eanble clear of CQMASK after each pause operation. This may be useful when using software flags to pause CQ."]
pub struct CQAUTOCLEARMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CQAUTOCLEARMASK_W<'a> {
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
#[doc = "Field `CQPWROFF` reader - Power off MSPI domain upon completion of DMA operation."]
pub struct CQPWROFF_R(crate::FieldReader<bool, bool>);
impl CQPWROFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQPWROFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQPWROFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQPWROFF` writer - Power off MSPI domain upon completion of DMA operation."]
pub struct CQPWROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CQPWROFF_W<'a> {
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
#[doc = "Sets the Priority of the command queue dma request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CQPRI_A {
    #[doc = "0: Low Priority (service as best effort) value."]
    LOW = 0,
    #[doc = "1: High Priority (service immediately) value."]
    HIGH = 1,
}
impl From<CQPRI_A> for bool {
    #[inline(always)]
    fn from(variant: CQPRI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CQPRI` reader - Sets the Priority of the command queue dma request"]
pub struct CQPRI_R(crate::FieldReader<bool, CQPRI_A>);
impl CQPRI_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQPRI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CQPRI_A {
        match self.bits {
            false => CQPRI_A::LOW,
            true => CQPRI_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == CQPRI_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == CQPRI_A::HIGH
    }
}
impl core::ops::Deref for CQPRI_R {
    type Target = crate::FieldReader<bool, CQPRI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQPRI` writer - Sets the Priority of the command queue dma request"]
pub struct CQPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> CQPRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CQPRI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low Priority (service as best effort) value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CQPRI_A::LOW)
    }
    #[doc = "High Priority (service immediately) value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CQPRI_A::HIGH)
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
#[doc = "Command queue enable. When set, will enable the processing of the command queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CQEN_A {
    #[doc = "0: Disable CQ Function value."]
    DIS = 0,
    #[doc = "1: Enable CQ Function value."]
    EN = 1,
}
impl From<CQEN_A> for bool {
    #[inline(always)]
    fn from(variant: CQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CQEN` reader - Command queue enable. When set, will enable the processing of the command queue"]
pub struct CQEN_R(crate::FieldReader<bool, CQEN_A>);
impl CQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CQEN_A {
        match self.bits {
            false => CQEN_A::DIS,
            true => CQEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CQEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CQEN_A::EN
    }
}
impl core::ops::Deref for CQEN_R {
    type Target = crate::FieldReader<bool, CQEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQEN` writer - Command queue enable. When set, will enable the processing of the command queue"]
pub struct CQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CQEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable CQ Function value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CQEN_A::DIS)
    }
    #[doc = "Enable CQ Function value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CQEN_A::EN)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Eanble clear of CQMASK after each pause operation. This may be useful when using software flags to pause CQ."]
    #[inline(always)]
    pub fn cqautoclearmask(&self) -> CQAUTOCLEARMASK_R {
        CQAUTOCLEARMASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power off MSPI domain upon completion of DMA operation."]
    #[inline(always)]
    pub fn cqpwroff(&self) -> CQPWROFF_R {
        CQPWROFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sets the Priority of the command queue dma request"]
    #[inline(always)]
    pub fn cqpri(&self) -> CQPRI_R {
        CQPRI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command queue enable. When set, will enable the processing of the command queue"]
    #[inline(always)]
    pub fn cqen(&self) -> CQEN_R {
        CQEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Eanble clear of CQMASK after each pause operation. This may be useful when using software flags to pause CQ."]
    #[inline(always)]
    pub fn cqautoclearmask(&mut self) -> CQAUTOCLEARMASK_W {
        CQAUTOCLEARMASK_W { w: self }
    }
    #[doc = "Bit 2 - Power off MSPI domain upon completion of DMA operation."]
    #[inline(always)]
    pub fn cqpwroff(&mut self) -> CQPWROFF_W {
        CQPWROFF_W { w: self }
    }
    #[doc = "Bit 1 - Sets the Priority of the command queue dma request"]
    #[inline(always)]
    pub fn cqpri(&mut self) -> CQPRI_W {
        CQPRI_W { w: self }
    }
    #[doc = "Bit 0 - Command queue enable. When set, will enable the processing of the command queue"]
    #[inline(always)]
    pub fn cqen(&mut self) -> CQEN_W {
        CQEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Queue Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcfg](index.html) module"]
pub struct CQCFG_SPEC;
impl crate::RegisterSpec for CQCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqcfg::R](R) reader structure"]
impl crate::Readable for CQCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqcfg::W](W) writer structure"]
impl crate::Writable for CQCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQCFG to value 0"]
impl crate::Resettable for CQCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
