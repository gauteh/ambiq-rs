#[doc = "Register `FAULTCAPTUREEN` reader"]
pub struct R(crate::R<FAULTCAPTUREEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAULTCAPTUREEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAULTCAPTUREEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAULTCAPTUREEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAULTCAPTUREEN` writer"]
pub struct W(crate::W<FAULTCAPTUREEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAULTCAPTUREEN_SPEC>;
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
impl From<crate::W<FAULTCAPTUREEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAULTCAPTUREEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTCAPTUREEN_A {
    #[doc = "0: Disable fault capture. value."]
    DIS = 0,
    #[doc = "1: Enable fault capture. value."]
    EN = 1,
}
impl From<FAULTCAPTUREEN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTCAPTUREEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULTCAPTUREEN` reader - Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
pub struct FAULTCAPTUREEN_R(crate::FieldReader<bool, FAULTCAPTUREEN_A>);
impl FAULTCAPTUREEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAULTCAPTUREEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTCAPTUREEN_A {
        match self.bits {
            false => FAULTCAPTUREEN_A::DIS,
            true => FAULTCAPTUREEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == FAULTCAPTUREEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == FAULTCAPTUREEN_A::EN
    }
}
impl core::ops::Deref for FAULTCAPTUREEN_R {
    type Target = crate::FieldReader<bool, FAULTCAPTUREEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTCAPTUREEN` writer - Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
pub struct FAULTCAPTUREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTCAPTUREEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTCAPTUREEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable fault capture. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FAULTCAPTUREEN_A::DIS)
    }
    #[doc = "Enable fault capture. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FAULTCAPTUREEN_A::EN)
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
    #[doc = "Bit 0 - Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
    #[inline(always)]
    pub fn faultcaptureen(&self) -> FAULTCAPTUREEN_R {
        FAULTCAPTUREEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
    #[inline(always)]
    pub fn faultcaptureen(&mut self) -> FAULTCAPTUREEN_W {
        FAULTCAPTUREEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable the fault capture registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faultcaptureen](index.html) module"]
pub struct FAULTCAPTUREEN_SPEC;
impl crate::RegisterSpec for FAULTCAPTUREEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [faultcaptureen::R](R) reader structure"]
impl crate::Readable for FAULTCAPTUREEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [faultcaptureen::W](W) writer structure"]
impl crate::Writable for FAULTCAPTUREEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FAULTCAPTUREEN to value 0"]
impl crate::Resettable for FAULTCAPTUREEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
