#[doc = "Register `DEBUGGER` reader"]
pub struct R(crate::R<DEBUGGER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUGGER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUGGER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUGGER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUGGER` writer"]
pub struct W(crate::W<DEBUGGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUGGER_SPEC>;
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
impl From<crate::W<DEBUGGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUGGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKOUT` reader - Lockout of debugger (SWD)."]
pub struct LOCKOUT_R(crate::FieldReader<bool, bool>);
impl LOCKOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKOUT` writer - Lockout of debugger (SWD)."]
pub struct LOCKOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKOUT_W<'a> {
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
    #[doc = "Bit 0 - Lockout of debugger (SWD)."]
    #[inline(always)]
    pub fn lockout(&self) -> LOCKOUT_R {
        LOCKOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lockout of debugger (SWD)."]
    #[inline(always)]
    pub fn lockout(&mut self) -> LOCKOUT_W {
        LOCKOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debugger Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debugger](index.html) module"]
pub struct DEBUGGER_SPEC;
impl crate::RegisterSpec for DEBUGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debugger::R](R) reader structure"]
impl crate::Readable for DEBUGGER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debugger::W](W) writer structure"]
impl crate::Writable for DEBUGGER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUGGER to value 0"]
impl crate::Resettable for DEBUGGER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
