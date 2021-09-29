#[doc = "Register `SUPPLYSRC` reader"]
pub struct R(crate::R<SUPPLYSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUPPLYSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUPPLYSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUPPLYSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUPPLYSRC` writer"]
pub struct W(crate::W<SUPPLYSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUPPLYSRC_SPEC>;
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
impl From<crate::W<SUPPLYSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUPPLYSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enables and Selects the BLE Buck as the supply for the BLE power domain or for Burst LDO. It takes the initial value from Customer INFO space. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate feature is allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEBUCKEN_A {
    #[doc = "1: Enable the BLE Buck. value."]
    EN = 1,
    #[doc = "0: Disable the BLE Buck. value."]
    DIS = 0,
}
impl From<BLEBUCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: BLEBUCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLEBUCKEN` reader - Enables and Selects the BLE Buck as the supply for the BLE power domain or for Burst LDO. It takes the initial value from Customer INFO space. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate feature is allowed."]
pub struct BLEBUCKEN_R(crate::FieldReader<bool, BLEBUCKEN_A>);
impl BLEBUCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEBUCKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLEBUCKEN_A {
        match self.bits {
            true => BLEBUCKEN_A::EN,
            false => BLEBUCKEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == BLEBUCKEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == BLEBUCKEN_A::DIS
    }
}
impl core::ops::Deref for BLEBUCKEN_R {
    type Target = crate::FieldReader<bool, BLEBUCKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEBUCKEN` writer - Enables and Selects the BLE Buck as the supply for the BLE power domain or for Burst LDO. It takes the initial value from Customer INFO space. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate feature is allowed."]
pub struct BLEBUCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEBUCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEBUCKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the BLE Buck. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BLEBUCKEN_A::EN)
    }
    #[doc = "Disable the BLE Buck. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BLEBUCKEN_A::DIS)
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
    #[doc = "Bit 0 - Enables and Selects the BLE Buck as the supply for the BLE power domain or for Burst LDO. It takes the initial value from Customer INFO space. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate feature is allowed."]
    #[inline(always)]
    pub fn blebucken(&self) -> BLEBUCKEN_R {
        BLEBUCKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables and Selects the BLE Buck as the supply for the BLE power domain or for Burst LDO. It takes the initial value from Customer INFO space. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate feature is allowed."]
    #[inline(always)]
    pub fn blebucken(&mut self) -> BLEBUCKEN_W {
        BLEBUCKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Regulator Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supplysrc](index.html) module"]
pub struct SUPPLYSRC_SPEC;
impl crate::RegisterSpec for SUPPLYSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [supplysrc::R](R) reader structure"]
impl crate::Readable for SUPPLYSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [supplysrc::W](W) writer structure"]
impl crate::Writable for SUPPLYSRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUPPLYSRC to value 0"]
impl crate::Resettable for SUPPLYSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
