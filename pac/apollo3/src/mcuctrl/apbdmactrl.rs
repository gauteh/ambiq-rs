#[doc = "Register `APBDMACTRL` reader"]
pub struct R(crate::R<APBDMACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBDMACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBDMACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBDMACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBDMACTRL` writer"]
pub struct W(crate::W<APBDMACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBDMACTRL_SPEC>;
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
impl From<crate::W<APBDMACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBDMACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HYSTERESIS` reader - This field determines how long the DMA will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms"]
pub struct HYSTERESIS_R(crate::FieldReader<u8, u8>);
impl HYSTERESIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HYSTERESIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HYSTERESIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYSTERESIS` writer - This field determines how long the DMA will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms"]
pub struct HYSTERESIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSTERESIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECODEABORT_A {
    #[doc = "0: Bus operations to powered down peripherals are quietly discarded value."]
    DISABLE = 0,
    #[doc = "1: Bus operations to powered down peripherals result in a bus fault. value."]
    ENABLE = 1,
}
impl From<DECODEABORT_A> for bool {
    #[inline(always)]
    fn from(variant: DECODEABORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DECODEABORT` reader - APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
pub struct DECODEABORT_R(crate::FieldReader<bool, DECODEABORT_A>);
impl DECODEABORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DECODEABORT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECODEABORT_A {
        match self.bits {
            false => DECODEABORT_A::DISABLE,
            true => DECODEABORT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DECODEABORT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DECODEABORT_A::ENABLE
    }
}
impl core::ops::Deref for DECODEABORT_R {
    type Target = crate::FieldReader<bool, DECODEABORT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECODEABORT` writer - APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
pub struct DECODEABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> DECODEABORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECODEABORT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus operations to powered down peripherals are quietly discarded value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DECODEABORT_A::DISABLE)
    }
    #[doc = "Bus operations to powered down peripherals result in a bus fault. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DECODEABORT_A::ENABLE)
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
#[doc = "Enable the DMA controller. When disabled, DMA requests will be ignored by the controller\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ENABLE_A {
    #[doc = "0: DMA operations disabled value."]
    DISABLE = 0,
    #[doc = "1: DMA operations enabled value."]
    ENABLE = 1,
}
impl From<DMA_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_ENABLE` reader - Enable the DMA controller. When disabled, DMA requests will be ignored by the controller"]
pub struct DMA_ENABLE_R(crate::FieldReader<bool, DMA_ENABLE_A>);
impl DMA_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_ENABLE_A {
        match self.bits {
            false => DMA_ENABLE_A::DISABLE,
            true => DMA_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DMA_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DMA_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for DMA_ENABLE_R {
    type Target = crate::FieldReader<bool, DMA_ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_ENABLE` writer - Enable the DMA controller. When disabled, DMA requests will be ignored by the controller"]
pub struct DMA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA operations disabled value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_ENABLE_A::DISABLE)
    }
    #[doc = "DMA operations enabled value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_ENABLE_A::ENABLE)
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
    #[doc = "Bits 8:15 - This field determines how long the DMA will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms"]
    #[inline(always)]
    pub fn hysteresis(&self) -> HYSTERESIS_R {
        HYSTERESIS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 1 - APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
    #[inline(always)]
    pub fn decodeabort(&self) -> DECODEABORT_R {
        DECODEABORT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable the DMA controller. When disabled, DMA requests will be ignored by the controller"]
    #[inline(always)]
    pub fn dma_enable(&self) -> DMA_ENABLE_R {
        DMA_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - This field determines how long the DMA will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms"]
    #[inline(always)]
    pub fn hysteresis(&mut self) -> HYSTERESIS_W {
        HYSTERESIS_W { w: self }
    }
    #[doc = "Bit 1 - APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
    #[inline(always)]
    pub fn decodeabort(&mut self) -> DECODEABORT_W {
        DECODEABORT_W { w: self }
    }
    #[doc = "Bit 0 - Enable the DMA controller. When disabled, DMA requests will be ignored by the controller"]
    #[inline(always)]
    pub fn dma_enable(&mut self) -> DMA_ENABLE_W {
        DMA_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control Register. Determines misc settings for DMA operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbdmactrl](index.html) module"]
pub struct APBDMACTRL_SPEC;
impl crate::RegisterSpec for APBDMACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbdmactrl::R](R) reader structure"]
impl crate::Readable for APBDMACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbdmactrl::W](W) writer structure"]
impl crate::Writable for APBDMACTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBDMACTRL to value 0x0203"]
impl crate::Resettable for APBDMACTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0203
    }
}
