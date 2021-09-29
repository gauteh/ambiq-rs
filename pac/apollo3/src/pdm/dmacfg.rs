#[doc = "Register `DMACFG` reader"]
pub struct R(crate::R<DMACFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACFG` writer"]
pub struct W(crate::W<DMACFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACFG_SPEC>;
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
impl From<crate::W<DMACFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPWROFF` reader - Power Off the ADC System upon DMACPL."]
pub struct DPWROFF_R(crate::FieldReader<bool, bool>);
impl DPWROFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPWROFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPWROFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPWROFF` writer - Power Off the ADC System upon DMACPL."]
pub struct DPWROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DPWROFF_W<'a> {
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
#[doc = "Field `DAUTOHIP` reader - Raise priority to high on fifo full, and DMAPRI set to low"]
pub struct DAUTOHIP_R(crate::FieldReader<bool, bool>);
impl DAUTOHIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAUTOHIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAUTOHIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAUTOHIP` writer - Raise priority to high on fifo full, and DMAPRI set to low"]
pub struct DAUTOHIP_W<'a> {
    w: &'a mut W,
}
impl<'a> DAUTOHIP_W<'a> {
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
#[doc = "Sets the Priority of the DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAPRI_A {
    #[doc = "0: Low Priority (service as best effort) value."]
    LOW = 0,
    #[doc = "1: High Priority (service immediately) value."]
    HIGH = 1,
}
impl From<DMAPRI_A> for bool {
    #[inline(always)]
    fn from(variant: DMAPRI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAPRI` reader - Sets the Priority of the DMA request"]
pub struct DMAPRI_R(crate::FieldReader<bool, DMAPRI_A>);
impl DMAPRI_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAPRI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAPRI_A {
        match self.bits {
            false => DMAPRI_A::LOW,
            true => DMAPRI_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == DMAPRI_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == DMAPRI_A::HIGH
    }
}
impl core::ops::Deref for DMAPRI_R {
    type Target = crate::FieldReader<bool, DMAPRI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAPRI` writer - Sets the Priority of the DMA request"]
pub struct DMAPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAPRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAPRI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low Priority (service as best effort) value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DMAPRI_A::LOW)
    }
    #[doc = "High Priority (service immediately) value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DMAPRI_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMADIR_A {
    #[doc = "0: Peripheral to Memory (SRAM) transaction. THe PDM module will only DMA to memory. value."]
    P2M = 0,
    #[doc = "1: Memory to Peripheral transaction. Not available for PDM module value."]
    M2P = 1,
}
impl From<DMADIR_A> for bool {
    #[inline(always)]
    fn from(variant: DMADIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMADIR` reader - Direction"]
pub struct DMADIR_R(crate::FieldReader<bool, DMADIR_A>);
impl DMADIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMADIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADIR_A {
        match self.bits {
            false => DMADIR_A::P2M,
            true => DMADIR_A::M2P,
        }
    }
    #[doc = "Checks if the value of the field is `P2M`"]
    #[inline(always)]
    pub fn is_p2m(&self) -> bool {
        **self == DMADIR_A::P2M
    }
    #[doc = "Checks if the value of the field is `M2P`"]
    #[inline(always)]
    pub fn is_m2p(&self) -> bool {
        **self == DMADIR_A::M2P
    }
}
impl core::ops::Deref for DMADIR_R {
    type Target = crate::FieldReader<bool, DMADIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMADIR` writer - Direction"]
pub struct DMADIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Peripheral to Memory (SRAM) transaction. THe PDM module will only DMA to memory. value."]
    #[inline(always)]
    pub fn p2m(self) -> &'a mut W {
        self.variant(DMADIR_A::P2M)
    }
    #[doc = "Memory to Peripheral transaction. Not available for PDM module value."]
    #[inline(always)]
    pub fn m2p(self) -> &'a mut W {
        self.variant(DMADIR_A::M2P)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: Disable DMA Function value."]
    DIS = 0,
    #[doc = "1: Enable DMA Function value."]
    EN = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DIS,
            true => DMAEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == DMAEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == DMAEN_A::EN
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable DMA Function value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAEN_A::DIS)
    }
    #[doc = "Enable DMA Function value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAEN_A::EN)
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
    #[doc = "Bit 10 - Power Off the ADC System upon DMACPL."]
    #[inline(always)]
    pub fn dpwroff(&self) -> DPWROFF_R {
        DPWROFF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Raise priority to high on fifo full, and DMAPRI set to low"]
    #[inline(always)]
    pub fn dautohip(&self) -> DAUTOHIP_R {
        DAUTOHIP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sets the Priority of the DMA request"]
    #[inline(always)]
    pub fn dmapri(&self) -> DMAPRI_R {
        DMAPRI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Direction"]
    #[inline(always)]
    pub fn dmadir(&self) -> DMADIR_R {
        DMADIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Power Off the ADC System upon DMACPL."]
    #[inline(always)]
    pub fn dpwroff(&mut self) -> DPWROFF_W {
        DPWROFF_W { w: self }
    }
    #[doc = "Bit 9 - Raise priority to high on fifo full, and DMAPRI set to low"]
    #[inline(always)]
    pub fn dautohip(&mut self) -> DAUTOHIP_W {
        DAUTOHIP_W { w: self }
    }
    #[doc = "Bit 8 - Sets the Priority of the DMA request"]
    #[inline(always)]
    pub fn dmapri(&mut self) -> DMAPRI_W {
        DMAPRI_W { w: self }
    }
    #[doc = "Bit 2 - Direction"]
    #[inline(always)]
    pub fn dmadir(&mut self) -> DMADIR_W {
        DMADIR_W { w: self }
    }
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacfg](index.html) module"]
pub struct DMACFG_SPEC;
impl crate::RegisterSpec for DMACFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacfg::R](R) reader structure"]
impl crate::Readable for DMACFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacfg::W](W) writer structure"]
impl crate::Writable for DMACFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACFG to value 0"]
impl crate::Resettable for DMACFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
