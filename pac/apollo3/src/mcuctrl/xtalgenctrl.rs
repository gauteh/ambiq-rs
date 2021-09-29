#[doc = "Register `XTALGENCTRL` reader"]
pub struct R(crate::R<XTALGENCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTALGENCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTALGENCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTALGENCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTALGENCTRL` writer"]
pub struct W(crate::W<XTALGENCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTALGENCTRL_SPEC>;
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
impl From<crate::W<XTALGENCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTALGENCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTALKSBIASTRIM` reader - XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
pub struct XTALKSBIASTRIM_R(crate::FieldReader<u8, u8>);
impl XTALKSBIASTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        XTALKSBIASTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTALKSBIASTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTALKSBIASTRIM` writer - XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
pub struct XTALKSBIASTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALKSBIASTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `XTALBIASTRIM` reader - XTAL BIAS trim"]
pub struct XTALBIASTRIM_R(crate::FieldReader<u8, u8>);
impl XTALBIASTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        XTALBIASTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTALBIASTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTALBIASTRIM` writer - XTAL BIAS trim"]
pub struct XTALBIASTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALBIASTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | ((value as u32 & 0x3f) << 2);
        self.w
    }
}
#[doc = "Auto-calibration delay control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACWARMUP_A {
    #[doc = "0: Warmup period of 1-2 seconds value."]
    SEC1 = 0,
    #[doc = "1: Warmup period of 2-4 seconds value."]
    SEC2 = 1,
    #[doc = "2: Warmup period of 4-8 seconds value."]
    SEC4 = 2,
    #[doc = "3: Warmup period of 8-16 seconds value."]
    SEC8 = 3,
}
impl From<ACWARMUP_A> for u8 {
    #[inline(always)]
    fn from(variant: ACWARMUP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACWARMUP` reader - Auto-calibration delay control"]
pub struct ACWARMUP_R(crate::FieldReader<u8, ACWARMUP_A>);
impl ACWARMUP_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACWARMUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACWARMUP_A {
        match self.bits {
            0 => ACWARMUP_A::SEC1,
            1 => ACWARMUP_A::SEC2,
            2 => ACWARMUP_A::SEC4,
            3 => ACWARMUP_A::SEC8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEC1`"]
    #[inline(always)]
    pub fn is_sec1(&self) -> bool {
        **self == ACWARMUP_A::SEC1
    }
    #[doc = "Checks if the value of the field is `SEC2`"]
    #[inline(always)]
    pub fn is_sec2(&self) -> bool {
        **self == ACWARMUP_A::SEC2
    }
    #[doc = "Checks if the value of the field is `SEC4`"]
    #[inline(always)]
    pub fn is_sec4(&self) -> bool {
        **self == ACWARMUP_A::SEC4
    }
    #[doc = "Checks if the value of the field is `SEC8`"]
    #[inline(always)]
    pub fn is_sec8(&self) -> bool {
        **self == ACWARMUP_A::SEC8
    }
}
impl core::ops::Deref for ACWARMUP_R {
    type Target = crate::FieldReader<u8, ACWARMUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACWARMUP` writer - Auto-calibration delay control"]
pub struct ACWARMUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACWARMUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACWARMUP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Warmup period of 1-2 seconds value."]
    #[inline(always)]
    pub fn sec1(self) -> &'a mut W {
        self.variant(ACWARMUP_A::SEC1)
    }
    #[doc = "Warmup period of 2-4 seconds value."]
    #[inline(always)]
    pub fn sec2(self) -> &'a mut W {
        self.variant(ACWARMUP_A::SEC2)
    }
    #[doc = "Warmup period of 4-8 seconds value."]
    #[inline(always)]
    pub fn sec4(self) -> &'a mut W {
        self.variant(ACWARMUP_A::SEC4)
    }
    #[doc = "Warmup period of 8-16 seconds value."]
    #[inline(always)]
    pub fn sec8(self) -> &'a mut W {
        self.variant(ACWARMUP_A::SEC8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13 - XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
    #[inline(always)]
    pub fn xtalksbiastrim(&self) -> XTALKSBIASTRIM_R {
        XTALKSBIASTRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 2:7 - XTAL BIAS trim"]
    #[inline(always)]
    pub fn xtalbiastrim(&self) -> XTALBIASTRIM_R {
        XTALBIASTRIM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 0:1 - Auto-calibration delay control"]
    #[inline(always)]
    pub fn acwarmup(&self) -> ACWARMUP_R {
        ACWARMUP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:13 - XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
    #[inline(always)]
    pub fn xtalksbiastrim(&mut self) -> XTALKSBIASTRIM_W {
        XTALKSBIASTRIM_W { w: self }
    }
    #[doc = "Bits 2:7 - XTAL BIAS trim"]
    #[inline(always)]
    pub fn xtalbiastrim(&mut self) -> XTALBIASTRIM_W {
        XTALBIASTRIM_W { w: self }
    }
    #[doc = "Bits 0:1 - Auto-calibration delay control"]
    #[inline(always)]
    pub fn acwarmup(&mut self) -> ACWARMUP_W {
        ACWARMUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTAL Oscillator General Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalgenctrl](index.html) module"]
pub struct XTALGENCTRL_SPEC;
impl crate::RegisterSpec for XTALGENCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtalgenctrl::R](R) reader structure"]
impl crate::Readable for XTALGENCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtalgenctrl::W](W) writer structure"]
impl crate::Writable for XTALGENCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTALGENCTRL to value 0x0100"]
impl crate::Resettable for XTALGENCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
