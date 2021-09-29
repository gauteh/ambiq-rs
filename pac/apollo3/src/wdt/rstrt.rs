#[doc = "Register `RSTRT` reader"]
pub struct R(crate::R<RSTRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTRT` writer"]
pub struct W(crate::W<RSTRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTRT_SPEC>;
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
impl From<crate::W<RSTRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTRT_A {
    #[doc = "178: This is the key value to write to WDTRSTRT to restart the WDT.  This is a write only register. value."]
    KEYVALUE = 178,
}
impl From<RSTRT_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTRT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RSTRT` reader - Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0."]
pub struct RSTRT_R(crate::FieldReader<u8, RSTRT_A>);
impl RSTRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSTRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSTRT_A> {
        match self.bits {
            178 => Some(RSTRT_A::KEYVALUE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEYVALUE`"]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        **self == RSTRT_A::KEYVALUE
    }
}
impl core::ops::Deref for RSTRT_R {
    type Target = crate::FieldReader<u8, RSTRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTRT` writer - Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0."]
pub struct RSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTRT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "This is the key value to write to WDTRSTRT to restart the WDT. This is a write only register. value."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut W {
        self.variant(RSTRT_A::KEYVALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0."]
    #[inline(always)]
    pub fn rstrt(&self) -> RSTRT_R {
        RSTRT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0."]
    #[inline(always)]
    pub fn rstrt(&mut self) -> RSTRT_W {
        RSTRT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Restart the watchdog timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstrt](index.html) module"]
pub struct RSTRT_SPEC;
impl crate::RegisterSpec for RSTRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstrt::R](R) reader structure"]
impl crate::Readable for RSTRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstrt::W](W) writer structure"]
impl crate::Writable for RSTRT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTRT to value 0"]
impl crate::Resettable for RSTRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
