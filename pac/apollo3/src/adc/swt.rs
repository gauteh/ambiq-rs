#[doc = "Register `SWT` reader"]
pub struct R(crate::R<SWT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWT` writer"]
pub struct W(crate::W<SWT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWT_SPEC>;
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
impl From<crate::W<SWT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Writing 0x37 to this register generates a software trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWT_A {
    #[doc = "55: Writing this value generates a software trigger. value."]
    GEN_SW_TRIGGER = 55,
}
impl From<SWT_A> for u8 {
    #[inline(always)]
    fn from(variant: SWT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SWT` reader - Writing 0x37 to this register generates a software trigger."]
pub struct SWT_R(crate::FieldReader<u8, SWT_A>);
impl SWT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWT_A> {
        match self.bits {
            55 => Some(SWT_A::GEN_SW_TRIGGER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GEN_SW_TRIGGER`"]
    #[inline(always)]
    pub fn is_gen_sw_trigger(&self) -> bool {
        **self == SWT_A::GEN_SW_TRIGGER
    }
}
impl core::ops::Deref for SWT_R {
    type Target = crate::FieldReader<u8, SWT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT` writer - Writing 0x37 to this register generates a software trigger."]
pub struct SWT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing this value generates a software trigger. value."]
    #[inline(always)]
    pub fn gen_sw_trigger(self) -> &'a mut W {
        self.variant(SWT_A::GEN_SW_TRIGGER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Writing 0x37 to this register generates a software trigger."]
    #[inline(always)]
    pub fn swt(&self) -> SWT_R {
        SWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing 0x37 to this register generates a software trigger."]
    #[inline(always)]
    pub fn swt(&mut self) -> SWT_W {
        SWT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swt](index.html) module"]
pub struct SWT_SPEC;
impl crate::RegisterSpec for SWT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swt::R](R) reader structure"]
impl crate::Readable for SWT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swt::W](W) writer structure"]
impl crate::Writable for SWT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWT to value 0"]
impl crate::Resettable for SWT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
