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
#[doc = "Field `WDREN` reader - Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset. This includes enabling the RESEN bit in WDTCFG register in Watch dog timer block."]
pub struct WDREN_R(crate::FieldReader<bool, bool>);
impl WDREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDREN` writer - Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset. This includes enabling the RESEN bit in WDTCFG register in Watch dog timer block."]
pub struct WDREN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDREN_W<'a> {
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
#[doc = "Field `BODHREN` reader - Brown out high (2.1v) reset enable."]
pub struct BODHREN_R(crate::FieldReader<bool, bool>);
impl BODHREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODHREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODHREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODHREN` writer - Brown out high (2.1v) reset enable."]
pub struct BODHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BODHREN_W<'a> {
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
    #[doc = "Bit 1 - Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset. This includes enabling the RESEN bit in WDTCFG register in Watch dog timer block."]
    #[inline(always)]
    pub fn wdren(&self) -> WDREN_R {
        WDREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Brown out high (2.1v) reset enable."]
    #[inline(always)]
    pub fn bodhren(&self) -> BODHREN_R {
        BODHREN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset. This includes enabling the RESEN bit in WDTCFG register in Watch dog timer block."]
    #[inline(always)]
    pub fn wdren(&mut self) -> WDREN_W {
        WDREN_W { w: self }
    }
    #[doc = "Bit 0 - Brown out high (2.1v) reset enable."]
    #[inline(always)]
    pub fn bodhren(&mut self) -> BODHREN_W {
        BODHREN_W { w: self }
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
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
