#[doc = "Register `CCTRL` reader"]
pub struct R(crate::R<CCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCTRL` writer"]
pub struct W(crate::W<CCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCTRL_SPEC>;
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
impl From<crate::W<CCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Core Clock divisor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORESEL_A {
    #[doc = "0: Core Clock is HFRC value."]
    HFRC = 0,
    #[doc = "1: Core Clock is HFRC / 2 value."]
    HFRC_DIV2 = 1,
}
impl From<CORESEL_A> for bool {
    #[inline(always)]
    fn from(variant: CORESEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORESEL` reader - Core Clock divisor"]
pub struct CORESEL_R(crate::FieldReader<bool, CORESEL_A>);
impl CORESEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORESEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CORESEL_A {
        match self.bits {
            false => CORESEL_A::HFRC,
            true => CORESEL_A::HFRC_DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        **self == CORESEL_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2`"]
    #[inline(always)]
    pub fn is_hfrc_div2(&self) -> bool {
        **self == CORESEL_A::HFRC_DIV2
    }
}
impl core::ops::Deref for CORESEL_R {
    type Target = crate::FieldReader<bool, CORESEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORESEL` writer - Core Clock divisor"]
pub struct CORESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CORESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CORESEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Core Clock is HFRC value."]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(CORESEL_A::HFRC)
    }
    #[doc = "Core Clock is HFRC / 2 value."]
    #[inline(always)]
    pub fn hfrc_div2(self) -> &'a mut W {
        self.variant(CORESEL_A::HFRC_DIV2)
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
    #[doc = "Bit 0 - Core Clock divisor"]
    #[inline(always)]
    pub fn coresel(&self) -> CORESEL_R {
        CORESEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Clock divisor"]
    #[inline(always)]
    pub fn coresel(&mut self) -> CORESEL_W {
        CORESEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFRC Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cctrl](index.html) module"]
pub struct CCTRL_SPEC;
impl crate::RegisterSpec for CCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cctrl::R](R) reader structure"]
impl crate::Readable for CCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cctrl::W](W) writer structure"]
impl crate::Writable for CCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCTRL to value 0x01"]
impl crate::Resettable for CCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
