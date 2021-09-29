#[doc = "Register `FLASHCFG` reader"]
pub struct R(crate::R<FLASHCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHCFG` writer"]
pub struct W(crate::W<FLASHCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHCFG_SPEC>;
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
impl From<crate::W<FLASHCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Controls flash low power modes (control of LPM pin).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMMODE_A {
    #[doc = "0: High power mode (LPM not used). value."]
    NEVER = 0,
    #[doc = "1: Fast Standby mode.  LPM deasserted for read operations, but asserted while flash IDLE. value."]
    STANDBY = 1,
    #[doc = "2: Low Power mode.  LPM always asserted for reads.  LPM_RD_WAIT must be programmed to accomodate longer read access times. value."]
    ALWAYS = 2,
}
impl From<LPMMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPMMODE` reader - Controls flash low power modes (control of LPM pin)."]
pub struct LPMMODE_R(crate::FieldReader<u8, LPMMODE_A>);
impl LPMMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPMMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPMMODE_A> {
        match self.bits {
            0 => Some(LPMMODE_A::NEVER),
            1 => Some(LPMMODE_A::STANDBY),
            2 => Some(LPMMODE_A::ALWAYS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NEVER`"]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        **self == LPMMODE_A::NEVER
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        **self == LPMMODE_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        **self == LPMMODE_A::ALWAYS
    }
}
impl core::ops::Deref for LPMMODE_R {
    type Target = crate::FieldReader<u8, LPMMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMMODE` writer - Controls flash low power modes (control of LPM pin)."]
pub struct LPMMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "High power mode (LPM not used). value."]
    #[inline(always)]
    pub fn never(self) -> &'a mut W {
        self.variant(LPMMODE_A::NEVER)
    }
    #[doc = "Fast Standby mode. LPM deasserted for read operations, but asserted while flash IDLE. value."]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(LPMMODE_A::STANDBY)
    }
    #[doc = "Low Power mode. LPM always asserted for reads. LPM_RD_WAIT must be programmed to accomodate longer read access times. value."]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(LPMMODE_A::ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `LPM_RD_WAIT` reader - Sets flash waitstates when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
pub struct LPM_RD_WAIT_R(crate::FieldReader<u8, u8>);
impl LPM_RD_WAIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPM_RD_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_RD_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_RD_WAIT` writer - Sets flash waitstates when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
pub struct LPM_RD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_RD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `SEDELAY` reader - Sets SE delay (flash address setup). A value of 5 is recommended."]
pub struct SEDELAY_R(crate::FieldReader<u8, u8>);
impl SEDELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEDELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEDELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEDELAY` writer - Sets SE delay (flash address setup). A value of 5 is recommended."]
pub struct SEDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> SEDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `RD_WAIT` reader - Sets read waitstates for normal (fast) operation. A value of 1 is recommended."]
pub struct RD_WAIT_R(crate::FieldReader<u8, u8>);
impl RD_WAIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RD_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_WAIT` writer - Sets read waitstates for normal (fast) operation. A value of 1 is recommended."]
pub struct RD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:13 - Controls flash low power modes (control of LPM pin)."]
    #[inline(always)]
    pub fn lpmmode(&self) -> LPMMODE_R {
        LPMMODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Sets flash waitstates when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
    #[inline(always)]
    pub fn lpm_rd_wait(&self) -> LPM_RD_WAIT_R {
        LPM_RD_WAIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Sets SE delay (flash address setup). A value of 5 is recommended."]
    #[inline(always)]
    pub fn sedelay(&self) -> SEDELAY_R {
        SEDELAY_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Sets read waitstates for normal (fast) operation. A value of 1 is recommended."]
    #[inline(always)]
    pub fn rd_wait(&self) -> RD_WAIT_R {
        RD_WAIT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13 - Controls flash low power modes (control of LPM pin)."]
    #[inline(always)]
    pub fn lpmmode(&mut self) -> LPMMODE_W {
        LPMMODE_W { w: self }
    }
    #[doc = "Bits 8:11 - Sets flash waitstates when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
    #[inline(always)]
    pub fn lpm_rd_wait(&mut self) -> LPM_RD_WAIT_W {
        LPM_RD_WAIT_W { w: self }
    }
    #[doc = "Bits 4:6 - Sets SE delay (flash address setup). A value of 5 is recommended."]
    #[inline(always)]
    pub fn sedelay(&mut self) -> SEDELAY_W {
        SEDELAY_W { w: self }
    }
    #[doc = "Bits 0:3 - Sets read waitstates for normal (fast) operation. A value of 1 is recommended."]
    #[inline(always)]
    pub fn rd_wait(&mut self) -> RD_WAIT_W {
        RD_WAIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcfg](index.html) module"]
pub struct FLASHCFG_SPEC;
impl crate::RegisterSpec for FLASHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashcfg::R](R) reader structure"]
impl crate::Readable for FLASHCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashcfg::W](W) writer structure"]
impl crate::Writable for FLASHCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHCFG to value 0x0873"]
impl crate::Resettable for FLASHCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0873
    }
}
