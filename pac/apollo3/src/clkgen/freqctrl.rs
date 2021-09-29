#[doc = "Register `FREQCTRL` reader"]
pub struct R(crate::R<FREQCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQCTRL` writer"]
pub struct W(crate::W<FREQCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQCTRL_SPEC>;
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
impl From<crate::W<FREQCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BURSTSTATUS` reader - This represents frequency burst status."]
pub struct BURSTSTATUS_R(crate::FieldReader<bool, bool>);
impl BURSTSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BURSTSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BURSTSTATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURSTSTATUS` writer - This represents frequency burst status."]
pub struct BURSTSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTSTATUS_W<'a> {
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
#[doc = "Field `BURSTACK` reader - Frequency Burst Request Acknowledge. Frequency burst requested is always acknowledged whether burst is granted or not depending on feature enable."]
pub struct BURSTACK_R(crate::FieldReader<bool, bool>);
impl BURSTACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BURSTACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BURSTACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURSTACK` writer - Frequency Burst Request Acknowledge. Frequency burst requested is always acknowledged whether burst is granted or not depending on feature enable."]
pub struct BURSTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTACK_W<'a> {
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
#[doc = "Frequency Burst Enable Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTREQ_A {
    #[doc = "0: Frequency for ARM core stays at 48MHz value."]
    DIS = 0,
    #[doc = "1: Frequency for ARM core is increased to 96MHz value."]
    EN = 1,
}
impl From<BURSTREQ_A> for bool {
    #[inline(always)]
    fn from(variant: BURSTREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURSTREQ` reader - Frequency Burst Enable Request"]
pub struct BURSTREQ_R(crate::FieldReader<bool, BURSTREQ_A>);
impl BURSTREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        BURSTREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTREQ_A {
        match self.bits {
            false => BURSTREQ_A::DIS,
            true => BURSTREQ_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == BURSTREQ_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == BURSTREQ_A::EN
    }
}
impl core::ops::Deref for BURSTREQ_R {
    type Target = crate::FieldReader<bool, BURSTREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURSTREQ` writer - Frequency Burst Enable Request"]
pub struct BURSTREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTREQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Frequency for ARM core stays at 48MHz value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BURSTREQ_A::DIS)
    }
    #[doc = "Frequency for ARM core is increased to 96MHz value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BURSTREQ_A::EN)
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
    #[doc = "Bit 2 - This represents frequency burst status."]
    #[inline(always)]
    pub fn burststatus(&self) -> BURSTSTATUS_R {
        BURSTSTATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frequency Burst Request Acknowledge. Frequency burst requested is always acknowledged whether burst is granted or not depending on feature enable."]
    #[inline(always)]
    pub fn burstack(&self) -> BURSTACK_R {
        BURSTACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Frequency Burst Enable Request"]
    #[inline(always)]
    pub fn burstreq(&self) -> BURSTREQ_R {
        BURSTREQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This represents frequency burst status."]
    #[inline(always)]
    pub fn burststatus(&mut self) -> BURSTSTATUS_W {
        BURSTSTATUS_W { w: self }
    }
    #[doc = "Bit 1 - Frequency Burst Request Acknowledge. Frequency burst requested is always acknowledged whether burst is granted or not depending on feature enable."]
    #[inline(always)]
    pub fn burstack(&mut self) -> BURSTACK_W {
        BURSTACK_W { w: self }
    }
    #[doc = "Bit 0 - Frequency Burst Enable Request"]
    #[inline(always)]
    pub fn burstreq(&mut self) -> BURSTREQ_W {
        BURSTREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFRC Frequency Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqctrl](index.html) module"]
pub struct FREQCTRL_SPEC;
impl crate::RegisterSpec for FREQCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freqctrl::R](R) reader structure"]
impl crate::Readable for FREQCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freqctrl::W](W) writer structure"]
impl crate::Writable for FREQCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQCTRL to value 0"]
impl crate::Resettable for FREQCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
