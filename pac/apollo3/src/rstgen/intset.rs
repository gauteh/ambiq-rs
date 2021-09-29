#[doc = "Register `INTSET` reader"]
pub struct R(crate::R<INTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSET` writer"]
pub struct W(crate::W<INTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSET_SPEC>;
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
impl From<crate::W<INTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BODH` reader - Enables an interrupt that triggers when VCC is below BODH level."]
pub struct BODH_R(crate::FieldReader<bool, bool>);
impl BODH_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODH` writer - Enables an interrupt that triggers when VCC is below BODH level."]
pub struct BODH_W<'a> {
    w: &'a mut W,
}
impl<'a> BODH_W<'a> {
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
    #[doc = "Bit 0 - Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline(always)]
    pub fn bodh(&self) -> BODH_R {
        BODH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline(always)]
    pub fn bodh(&mut self) -> BODH_W {
        BODH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Interrupt register: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](index.html) module"]
pub struct INTSET_SPEC;
impl crate::RegisterSpec for INTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intset::R](R) reader structure"]
impl crate::Readable for INTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intset::W](W) writer structure"]
impl crate::Writable for INTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSET to value 0"]
impl crate::Resettable for INTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
