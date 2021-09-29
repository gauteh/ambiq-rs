#[doc = "Register `TPIUCTRL` reader"]
pub struct R(crate::R<TPIUCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPIUCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPIUCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPIUCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPIUCTRL` writer"]
pub struct W(crate::W<TPIUCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPIUCTRL_SPEC>;
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
impl From<crate::W<TPIUCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPIUCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "This field selects the frequency of the ARM M4 TPIU port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Low power state. value."]
    LOWPWR = 0,
    #[doc = "1: Selects HFRC divided by 2 as the source TPIU clk value."]
    HFRCDIV2 = 1,
    #[doc = "2: Selects HFRC divided by 8 as the source TPIU clk value."]
    HFRCDIV8 = 2,
    #[doc = "3: Selects HFRC divided by 16 as the source TPIU clk value."]
    HFRCDIV16 = 3,
    #[doc = "4: Selects HFRC divided by 32 as the source TPIU clk value."]
    HFRCDIV32 = 4,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKSEL` reader - This field selects the frequency of the ARM M4 TPIU port."]
pub struct CLKSEL_R(crate::FieldReader<u8, CLKSEL_A>);
impl CLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::LOWPWR),
            1 => Some(CLKSEL_A::HFRCDIV2),
            2 => Some(CLKSEL_A::HFRCDIV8),
            3 => Some(CLKSEL_A::HFRCDIV16),
            4 => Some(CLKSEL_A::HFRCDIV32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOWPWR`"]
    #[inline(always)]
    pub fn is_lowpwr(&self) -> bool {
        **self == CLKSEL_A::LOWPWR
    }
    #[doc = "Checks if the value of the field is `HFRCDIV2`"]
    #[inline(always)]
    pub fn is_hfrcdiv2(&self) -> bool {
        **self == CLKSEL_A::HFRCDIV2
    }
    #[doc = "Checks if the value of the field is `HFRCDIV8`"]
    #[inline(always)]
    pub fn is_hfrcdiv8(&self) -> bool {
        **self == CLKSEL_A::HFRCDIV8
    }
    #[doc = "Checks if the value of the field is `HFRCDIV16`"]
    #[inline(always)]
    pub fn is_hfrcdiv16(&self) -> bool {
        **self == CLKSEL_A::HFRCDIV16
    }
    #[doc = "Checks if the value of the field is `HFRCDIV32`"]
    #[inline(always)]
    pub fn is_hfrcdiv32(&self) -> bool {
        **self == CLKSEL_A::HFRCDIV32
    }
}
impl core::ops::Deref for CLKSEL_R {
    type Target = crate::FieldReader<u8, CLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSEL` writer - This field selects the frequency of the ARM M4 TPIU port."]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low power state. value."]
    #[inline(always)]
    pub fn lowpwr(self) -> &'a mut W {
        self.variant(CLKSEL_A::LOWPWR)
    }
    #[doc = "Selects HFRC divided by 2 as the source TPIU clk value."]
    #[inline(always)]
    pub fn hfrcdiv2(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCDIV2)
    }
    #[doc = "Selects HFRC divided by 8 as the source TPIU clk value."]
    #[inline(always)]
    pub fn hfrcdiv8(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCDIV8)
    }
    #[doc = "Selects HFRC divided by 16 as the source TPIU clk value."]
    #[inline(always)]
    pub fn hfrcdiv16(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCDIV16)
    }
    #[doc = "Selects HFRC divided by 32 as the source TPIU clk value."]
    #[inline(always)]
    pub fn hfrcdiv32(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCDIV32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU's SWO port using the ARM ITM and TPIU modules.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable the TPIU. value."]
    DIS = 0,
    #[doc = "1: Enable the TPIU. value."]
    EN = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU's SWO port using the ARM ITM and TPIU modules."]
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DIS,
            true => ENABLE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == ENABLE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == ENABLE_A::EN
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU's SWO port using the ARM ITM and TPIU modules."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the TPIU. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENABLE_A::DIS)
    }
    #[doc = "Enable the TPIU. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ENABLE_A::EN)
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
    #[doc = "Bits 8:10 - This field selects the frequency of the ARM M4 TPIU port."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 0 - TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU's SWO port using the ARM ITM and TPIU modules."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - This field selects the frequency of the ARM M4 TPIU port."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 0 - TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU's SWO port using the ARM ITM and TPIU modules."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpiuctrl](index.html) module"]
pub struct TPIUCTRL_SPEC;
impl crate::RegisterSpec for TPIUCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpiuctrl::R](R) reader structure"]
impl crate::Readable for TPIUCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpiuctrl::W](W) writer structure"]
impl crate::Writable for TPIUCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPIUCTRL to value 0"]
impl crate::Resettable for TPIUCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
