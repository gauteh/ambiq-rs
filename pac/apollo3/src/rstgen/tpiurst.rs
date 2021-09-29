#[doc = "Register `TPIURST` reader"]
pub struct R(crate::R<TPIURST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPIURST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPIURST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPIURST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPIURST` writer"]
pub struct W(crate::W<TPIURST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPIURST_SPEC>;
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
impl From<crate::W<TPIURST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPIURST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPIURST` reader - Static reset for the TPIU. Write to '1' to assert reset to TPIU. Write to '0' to clear the reset."]
pub struct TPIURST_R(crate::FieldReader<bool, bool>);
impl TPIURST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPIURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPIURST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPIURST` writer - Static reset for the TPIU. Write to '1' to assert reset to TPIU. Write to '0' to clear the reset."]
pub struct TPIURST_W<'a> {
    w: &'a mut W,
}
impl<'a> TPIURST_W<'a> {
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
    #[doc = "Bit 0 - Static reset for the TPIU. Write to '1' to assert reset to TPIU. Write to '0' to clear the reset."]
    #[inline(always)]
    pub fn tpiurst(&self) -> TPIURST_R {
        TPIURST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Static reset for the TPIU. Write to '1' to assert reset to TPIU. Write to '0' to clear the reset."]
    #[inline(always)]
    pub fn tpiurst(&mut self) -> TPIURST_W {
        TPIURST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TPIU reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpiurst](index.html) module"]
pub struct TPIURST_SPEC;
impl crate::RegisterSpec for TPIURST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpiurst::R](R) reader structure"]
impl crate::Readable for TPIURST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpiurst::W](W) writer structure"]
impl crate::Writable for TPIURST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPIURST to value 0"]
impl crate::Resettable for TPIURST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
