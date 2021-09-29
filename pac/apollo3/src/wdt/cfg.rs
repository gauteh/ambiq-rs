#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select the frequency for the WDT. All values not enumerated below are undefined.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Low Power Mode.  This setting disables the watch dog timer. value."]
    OFF = 0,
    #[doc = "1: 128 Hz LFRC clock. value."]
    _128HZ = 1,
    #[doc = "2: 16 Hz LFRC clock. value."]
    _16HZ = 2,
    #[doc = "3: 1 Hz LFRC clock. value."]
    _1HZ = 3,
    #[doc = "4: 1/16th Hz LFRC clock. value."]
    _1_16HZ = 4,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKSEL` reader - Select the frequency for the WDT. All values not enumerated below are undefined."]
pub struct CLKSEL_R(crate::FieldReader<u8, CLKSEL_A>);
impl CLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::OFF),
            1 => Some(CLKSEL_A::_128HZ),
            2 => Some(CLKSEL_A::_16HZ),
            3 => Some(CLKSEL_A::_1HZ),
            4 => Some(CLKSEL_A::_1_16HZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CLKSEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `_128HZ`"]
    #[inline(always)]
    pub fn is_128hz(&self) -> bool {
        **self == CLKSEL_A::_128HZ
    }
    #[doc = "Checks if the value of the field is `_16HZ`"]
    #[inline(always)]
    pub fn is_16hz(&self) -> bool {
        **self == CLKSEL_A::_16HZ
    }
    #[doc = "Checks if the value of the field is `_1HZ`"]
    #[inline(always)]
    pub fn is_1hz(&self) -> bool {
        **self == CLKSEL_A::_1HZ
    }
    #[doc = "Checks if the value of the field is `_1_16HZ`"]
    #[inline(always)]
    pub fn is_1_16hz(&self) -> bool {
        **self == CLKSEL_A::_1_16HZ
    }
}
impl core::ops::Deref for CLKSEL_R {
    type Target = crate::FieldReader<u8, CLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSEL` writer - Select the frequency for the WDT. All values not enumerated below are undefined."]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low Power Mode. This setting disables the watch dog timer. value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLKSEL_A::OFF)
    }
    #[doc = "128 Hz LFRC clock. value."]
    #[inline(always)]
    pub fn _128hz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_128HZ)
    }
    #[doc = "16 Hz LFRC clock. value."]
    #[inline(always)]
    pub fn _16hz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_16HZ)
    }
    #[doc = "1 Hz LFRC clock. value."]
    #[inline(always)]
    pub fn _1hz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_1HZ)
    }
    #[doc = "1/16th Hz LFRC clock. value."]
    #[inline(always)]
    pub fn _1_16hz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_1_16HZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `INTVAL` reader - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
pub struct INTVAL_R(crate::FieldReader<u8, u8>);
impl INTVAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTVAL` writer - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
pub struct INTVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `RESVAL` reader - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
pub struct RESVAL_R(crate::FieldReader<u8, u8>);
impl RESVAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESVAL` writer - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
pub struct RESVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `RESEN` reader - This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
pub struct RESEN_R(crate::FieldReader<bool, bool>);
impl RESEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESEN` writer - This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
pub struct RESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEN_W<'a> {
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
#[doc = "Field `INTEN` reader - This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
pub struct INTEN_R(crate::FieldReader<bool, bool>);
impl INTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN` writer - This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
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
#[doc = "Field `WDTEN` reader - This bitfield enables the WDT."]
pub struct WDTEN_R(crate::FieldReader<bool, bool>);
impl WDTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTEN` writer - This bitfield enables the WDT."]
pub struct WDTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTEN_W<'a> {
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
    #[doc = "Bits 24:26 - Select the frequency for the WDT. All values not enumerated below are undefined."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline(always)]
    pub fn intval(&self) -> INTVAL_R {
        INTVAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
    #[inline(always)]
    pub fn resval(&self) -> RESVAL_R {
        RESVAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 2 - This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
    #[inline(always)]
    pub fn resen(&self) -> RESEN_R {
        RESEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bitfield enables the WDT."]
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select the frequency for the WDT. All values not enumerated below are undefined."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bits 16:23 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline(always)]
    pub fn intval(&mut self) -> INTVAL_W {
        INTVAL_W { w: self }
    }
    #[doc = "Bits 8:15 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
    #[inline(always)]
    pub fn resval(&mut self) -> RESVAL_W {
        RESVAL_W { w: self }
    }
    #[doc = "Bit 2 - This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
    #[inline(always)]
    pub fn resen(&mut self) -> RESEN_W {
        RESEN_W { w: self }
    }
    #[doc = "Bit 1 - This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Bit 0 - This bitfield enables the WDT."]
    #[inline(always)]
    pub fn wdten(&mut self) -> WDTEN_W {
        WDTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x00ff_ff00"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_ff00
    }
}
