#[doc = "Register `OCTRL` reader"]
pub struct R(crate::R<OCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTRL` writer"]
pub struct W(crate::W<OCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTRL_SPEC>;
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
impl From<crate::W<OCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Autocalibration control. This selects the source to be used in the autocalibration flow. This flow can also be used to measure an internal clock against an external clock source, with the external clock normally used as the reference.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACAL_A {
    #[doc = "0: Disable Autocalibration value."]
    DIS = 0,
    #[doc = "2: Autocalibrate every 1024 seconds.  Once autocalibration is done, an interrupt will be triggered at the end of 1024 seconds. value."]
    _1024SEC = 2,
    #[doc = "3: Autocalibrate every 512 seconds.  Once autocalibration is done, an interrupt will be trigged at the end of 512 seconds. value."]
    _512SEC = 3,
    #[doc = "6: Frequency measurement using XT.  The XT clock is normally considered much more accurate than the LFRC clock source. value."]
    XTFREQ = 6,
    #[doc = "7: Frequency measurement using external clock. value."]
    EXTFREQ = 7,
}
impl From<ACAL_A> for u8 {
    #[inline(always)]
    fn from(variant: ACAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACAL` reader - Autocalibration control. This selects the source to be used in the autocalibration flow. This flow can also be used to measure an internal clock against an external clock source, with the external clock normally used as the reference."]
pub struct ACAL_R(crate::FieldReader<u8, ACAL_A>);
impl ACAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACAL_A> {
        match self.bits {
            0 => Some(ACAL_A::DIS),
            2 => Some(ACAL_A::_1024SEC),
            3 => Some(ACAL_A::_512SEC),
            6 => Some(ACAL_A::XTFREQ),
            7 => Some(ACAL_A::EXTFREQ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == ACAL_A::DIS
    }
    #[doc = "Checks if the value of the field is `_1024SEC`"]
    #[inline(always)]
    pub fn is_1024sec(&self) -> bool {
        **self == ACAL_A::_1024SEC
    }
    #[doc = "Checks if the value of the field is `_512SEC`"]
    #[inline(always)]
    pub fn is_512sec(&self) -> bool {
        **self == ACAL_A::_512SEC
    }
    #[doc = "Checks if the value of the field is `XTFREQ`"]
    #[inline(always)]
    pub fn is_xtfreq(&self) -> bool {
        **self == ACAL_A::XTFREQ
    }
    #[doc = "Checks if the value of the field is `EXTFREQ`"]
    #[inline(always)]
    pub fn is_extfreq(&self) -> bool {
        **self == ACAL_A::EXTFREQ
    }
}
impl core::ops::Deref for ACAL_R {
    type Target = crate::FieldReader<u8, ACAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACAL` writer - Autocalibration control. This selects the source to be used in the autocalibration flow. This flow can also be used to measure an internal clock against an external clock source, with the external clock normally used as the reference."]
pub struct ACAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable Autocalibration value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACAL_A::DIS)
    }
    #[doc = "Autocalibrate every 1024 seconds. Once autocalibration is done, an interrupt will be triggered at the end of 1024 seconds. value."]
    #[inline(always)]
    pub fn _1024sec(self) -> &'a mut W {
        self.variant(ACAL_A::_1024SEC)
    }
    #[doc = "Autocalibrate every 512 seconds. Once autocalibration is done, an interrupt will be trigged at the end of 512 seconds. value."]
    #[inline(always)]
    pub fn _512sec(self) -> &'a mut W {
        self.variant(ACAL_A::_512SEC)
    }
    #[doc = "Frequency measurement using XT. The XT clock is normally considered much more accurate than the LFRC clock source. value."]
    #[inline(always)]
    pub fn xtfreq(self) -> &'a mut W {
        self.variant(ACAL_A::XTFREQ)
    }
    #[doc = "Frequency measurement using external clock. value."]
    #[inline(always)]
    pub fn extfreq(self) -> &'a mut W {
        self.variant(ACAL_A::EXTFREQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Selects the RTC oscillator (1 => LFRC, 0 => XT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSEL_A {
    #[doc = "0: RTC uses the XT value."]
    RTC_XT = 0,
    #[doc = "1: RTC uses the LFRC value."]
    RTC_LFRC = 1,
}
impl From<OSEL_A> for bool {
    #[inline(always)]
    fn from(variant: OSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSEL` reader - Selects the RTC oscillator (1 => LFRC, 0 => XT)"]
pub struct OSEL_R(crate::FieldReader<bool, OSEL_A>);
impl OSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSEL_A {
        match self.bits {
            false => OSEL_A::RTC_XT,
            true => OSEL_A::RTC_LFRC,
        }
    }
    #[doc = "Checks if the value of the field is `RTC_XT`"]
    #[inline(always)]
    pub fn is_rtc_xt(&self) -> bool {
        **self == OSEL_A::RTC_XT
    }
    #[doc = "Checks if the value of the field is `RTC_LFRC`"]
    #[inline(always)]
    pub fn is_rtc_lfrc(&self) -> bool {
        **self == OSEL_A::RTC_LFRC
    }
}
impl core::ops::Deref for OSEL_R {
    type Target = crate::FieldReader<bool, OSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSEL` writer - Selects the RTC oscillator (1 => LFRC, 0 => XT)"]
pub struct OSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC uses the XT value."]
    #[inline(always)]
    pub fn rtc_xt(self) -> &'a mut W {
        self.variant(OSEL_A::RTC_XT)
    }
    #[doc = "RTC uses the LFRC value."]
    #[inline(always)]
    pub fn rtc_lfrc(self) -> &'a mut W {
        self.variant(OSEL_A::RTC_LFRC)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Oscillator switch on failure function. If this is set, then LFRC clock source will switch from XT to RC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOS_A {
    #[doc = "0: Disable the oscillator switch on failure function. value."]
    DIS = 0,
    #[doc = "1: Enable the oscillator switch on failure function. value."]
    EN = 1,
}
impl From<FOS_A> for bool {
    #[inline(always)]
    fn from(variant: FOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOS` reader - Oscillator switch on failure function. If this is set, then LFRC clock source will switch from XT to RC."]
pub struct FOS_R(crate::FieldReader<bool, FOS_A>);
impl FOS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOS_A {
        match self.bits {
            false => FOS_A::DIS,
            true => FOS_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == FOS_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == FOS_A::EN
    }
}
impl core::ops::Deref for FOS_R {
    type Target = crate::FieldReader<bool, FOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOS` writer - Oscillator switch on failure function. If this is set, then LFRC clock source will switch from XT to RC."]
pub struct FOS_W<'a> {
    w: &'a mut W,
}
impl<'a> FOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the oscillator switch on failure function. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FOS_A::DIS)
    }
    #[doc = "Enable the oscillator switch on failure function. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FOS_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Stop the LFRC Oscillator to the RTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPRC_A {
    #[doc = "0: Enable the LFRC Oscillator to drive the RTC value."]
    EN = 0,
    #[doc = "1: Stop the LFRC Oscillator when driving the RTC value."]
    STOP = 1,
}
impl From<STOPRC_A> for bool {
    #[inline(always)]
    fn from(variant: STOPRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPRC` reader - Stop the LFRC Oscillator to the RTC"]
pub struct STOPRC_R(crate::FieldReader<bool, STOPRC_A>);
impl STOPRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOPRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPRC_A {
        match self.bits {
            false => STOPRC_A::EN,
            true => STOPRC_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == STOPRC_A::EN
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == STOPRC_A::STOP
    }
}
impl core::ops::Deref for STOPRC_R {
    type Target = crate::FieldReader<bool, STOPRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOPRC` writer - Stop the LFRC Oscillator to the RTC"]
pub struct STOPRC_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the LFRC Oscillator to drive the RTC value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STOPRC_A::EN)
    }
    #[doc = "Stop the LFRC Oscillator when driving the RTC value."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOPRC_A::STOP)
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
#[doc = "Stop the XT Oscillator to the RTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPXT_A {
    #[doc = "0: Enable the XT Oscillator to drive the RTC value."]
    EN = 0,
    #[doc = "1: Stop the XT Oscillator when driving the RTC value."]
    STOP = 1,
}
impl From<STOPXT_A> for bool {
    #[inline(always)]
    fn from(variant: STOPXT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPXT` reader - Stop the XT Oscillator to the RTC"]
pub struct STOPXT_R(crate::FieldReader<bool, STOPXT_A>);
impl STOPXT_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOPXT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPXT_A {
        match self.bits {
            false => STOPXT_A::EN,
            true => STOPXT_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == STOPXT_A::EN
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == STOPXT_A::STOP
    }
}
impl core::ops::Deref for STOPXT_R {
    type Target = crate::FieldReader<bool, STOPXT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOPXT` writer - Stop the XT Oscillator to the RTC"]
pub struct STOPXT_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPXT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the XT Oscillator to drive the RTC value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STOPXT_A::EN)
    }
    #[doc = "Stop the XT Oscillator when driving the RTC value."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOPXT_A::STOP)
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
    #[doc = "Bits 8:10 - Autocalibration control. This selects the source to be used in the autocalibration flow. This flow can also be used to measure an internal clock against an external clock source, with the external clock normally used as the reference."]
    #[inline(always)]
    pub fn acal(&self) -> ACAL_R {
        ACAL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Selects the RTC oscillator (1 => LFRC, 0 => XT)"]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Oscillator switch on failure function. If this is set, then LFRC clock source will switch from XT to RC."]
    #[inline(always)]
    pub fn fos(&self) -> FOS_R {
        FOS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Stop the LFRC Oscillator to the RTC"]
    #[inline(always)]
    pub fn stoprc(&self) -> STOPRC_R {
        STOPRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Stop the XT Oscillator to the RTC"]
    #[inline(always)]
    pub fn stopxt(&self) -> STOPXT_R {
        STOPXT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - Autocalibration control. This selects the source to be used in the autocalibration flow. This flow can also be used to measure an internal clock against an external clock source, with the external clock normally used as the reference."]
    #[inline(always)]
    pub fn acal(&mut self) -> ACAL_W {
        ACAL_W { w: self }
    }
    #[doc = "Bit 7 - Selects the RTC oscillator (1 => LFRC, 0 => XT)"]
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W {
        OSEL_W { w: self }
    }
    #[doc = "Bit 6 - Oscillator switch on failure function. If this is set, then LFRC clock source will switch from XT to RC."]
    #[inline(always)]
    pub fn fos(&mut self) -> FOS_W {
        FOS_W { w: self }
    }
    #[doc = "Bit 1 - Stop the LFRC Oscillator to the RTC"]
    #[inline(always)]
    pub fn stoprc(&mut self) -> STOPRC_W {
        STOPRC_W { w: self }
    }
    #[doc = "Bit 0 - Stop the XT Oscillator to the RTC"]
    #[inline(always)]
    pub fn stopxt(&mut self) -> STOPXT_W {
        STOPXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octrl](index.html) module"]
pub struct OCTRL_SPEC;
impl crate::RegisterSpec for OCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octrl::R](R) reader structure"]
impl crate::Readable for OCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octrl::W](W) writer structure"]
impl crate::Writable for OCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCTRL to value 0"]
impl crate::Resettable for OCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
