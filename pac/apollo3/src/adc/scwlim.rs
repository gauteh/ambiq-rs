#[doc = "Register `SCWLIM` reader"]
pub struct R(crate::R<SCWLIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCWLIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCWLIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCWLIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCWLIM` writer"]
pub struct W(crate::W<SCWLIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCWLIM_SPEC>;
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
impl From<crate::W<SCWLIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCWLIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCWLIMEN` reader - Scale the window limits compare values per precision mode. When set to 0x0 (default), the values in the 20-bit limits registers will compare directly with the FIFO values regardless of the precision mode the slot is configured to. When set to 0x1, the compare values will be divided by the difference in precision bits while performing the window limit comparisons."]
pub struct SCWLIMEN_R(crate::FieldReader<bool, bool>);
impl SCWLIMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCWLIMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCWLIMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCWLIMEN` writer - Scale the window limits compare values per precision mode. When set to 0x0 (default), the values in the 20-bit limits registers will compare directly with the FIFO values regardless of the precision mode the slot is configured to. When set to 0x1, the compare values will be divided by the difference in precision bits while performing the window limit comparisons."]
pub struct SCWLIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCWLIMEN_W<'a> {
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
    #[doc = "Bit 0 - Scale the window limits compare values per precision mode. When set to 0x0 (default), the values in the 20-bit limits registers will compare directly with the FIFO values regardless of the precision mode the slot is configured to. When set to 0x1, the compare values will be divided by the difference in precision bits while performing the window limit comparisons."]
    #[inline(always)]
    pub fn scwlimen(&self) -> SCWLIMEN_R {
        SCWLIMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scale the window limits compare values per precision mode. When set to 0x0 (default), the values in the 20-bit limits registers will compare directly with the FIFO values regardless of the precision mode the slot is configured to. When set to 0x1, the compare values will be divided by the difference in precision bits while performing the window limit comparisons."]
    #[inline(always)]
    pub fn scwlimen(&mut self) -> SCWLIMEN_W {
        SCWLIMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scale Window Comparator Limits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scwlim](index.html) module"]
pub struct SCWLIM_SPEC;
impl crate::RegisterSpec for SCWLIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scwlim::R](R) reader structure"]
impl crate::Readable for SCWLIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scwlim::W](W) writer structure"]
impl crate::Writable for SCWLIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCWLIM to value 0"]
impl crate::Resettable for SCWLIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
