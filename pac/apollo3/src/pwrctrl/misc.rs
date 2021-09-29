#[doc = "Register `MISC` reader"]
pub struct R(crate::R<MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC` writer"]
pub struct W(crate::W<MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_SPEC>;
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
impl From<crate::W<MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMVRLPBLE_A {
    #[doc = "1: Mem VR can go to lp mode even when BLE is powered on. value."]
    EN = 1,
    #[doc = "0: Mem VR will stay in active mode when BLE is powered on. value."]
    DIS = 0,
}
impl From<MEMVRLPBLE_A> for bool {
    #[inline(always)]
    fn from(variant: MEMVRLPBLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMVRLPBLE` reader - Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it."]
pub struct MEMVRLPBLE_R(crate::FieldReader<bool, MEMVRLPBLE_A>);
impl MEMVRLPBLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEMVRLPBLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMVRLPBLE_A {
        match self.bits {
            true => MEMVRLPBLE_A::EN,
            false => MEMVRLPBLE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == MEMVRLPBLE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == MEMVRLPBLE_A::DIS
    }
}
impl core::ops::Deref for MEMVRLPBLE_R {
    type Target = crate::FieldReader<bool, MEMVRLPBLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMVRLPBLE` writer - Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it."]
pub struct MEMVRLPBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMVRLPBLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMVRLPBLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mem VR can go to lp mode even when BLE is powered on. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MEMVRLPBLE_A::EN)
    }
    #[doc = "Mem VR will stay in active mode when BLE is powered on. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MEMVRLPBLE_A::DIS)
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
#[doc = "Field `FORCEMEMVRLPTIMERS` reader - Control Bit to force Mem VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
pub struct FORCEMEMVRLPTIMERS_R(crate::FieldReader<bool, bool>);
impl FORCEMEMVRLPTIMERS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCEMEMVRLPTIMERS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCEMEMVRLPTIMERS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCEMEMVRLPTIMERS` writer - Control Bit to force Mem VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
pub struct FORCEMEMVRLPTIMERS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEMEMVRLPTIMERS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it."]
    #[inline(always)]
    pub fn memvrlpble(&self) -> MEMVRLPBLE_R {
        MEMVRLPBLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control Bit to force Mem VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
    #[inline(always)]
    pub fn forcememvrlptimers(&self) -> FORCEMEMVRLPTIMERS_R {
        FORCEMEMVRLPTIMERS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it."]
    #[inline(always)]
    pub fn memvrlpble(&mut self) -> MEMVRLPBLE_W {
        MEMVRLPBLE_W { w: self }
    }
    #[doc = "Bit 3 - Control Bit to force Mem VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
    #[inline(always)]
    pub fn forcememvrlptimers(&mut self) -> FORCEMEMVRLPTIMERS_W {
        FORCEMEMVRLPTIMERS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Optimization Control Bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc](index.html) module"]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc::R](R) reader structure"]
impl crate::Readable for MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc::W](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
