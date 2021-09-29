#[doc = "Register `LOCKSTAT` reader"]
pub struct R(crate::R<LOCKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKSTAT` writer"]
pub struct W(crate::W<LOCKSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCKSTAT_SPEC>;
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
impl From<crate::W<LOCKSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCKSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LOCK Status register. This register is a bitmask for which resources are currently unlocked. These bits are one-hot per resource.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum STATUS_A {
    #[doc = "1: Customer Key is unlocked (access is granted to top half of info0) value."]
    CUSTOMER_KEY = 1,
    #[doc = "0: No resources are unlocked value."]
    NONE = 0,
}
impl From<STATUS_A> for u32 {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATUS` reader - LOCK Status register. This register is a bitmask for which resources are currently unlocked. These bits are one-hot per resource."]
pub struct STATUS_R(crate::FieldReader<u32, STATUS_A>);
impl STATUS_R {
    pub(crate) fn new(bits: u32) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATUS_A> {
        match self.bits {
            1 => Some(STATUS_A::CUSTOMER_KEY),
            0 => Some(STATUS_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CUSTOMER_KEY`"]
    #[inline(always)]
    pub fn is_customer_key(&self) -> bool {
        **self == STATUS_A::CUSTOMER_KEY
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == STATUS_A::NONE
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<u32, STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS` writer - LOCK Status register. This register is a bitmask for which resources are currently unlocked. These bits are one-hot per resource."]
pub struct STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Customer Key is unlocked (access is granted to top half of info0) value."]
    #[inline(always)]
    pub fn customer_key(self) -> &'a mut W {
        self.variant(STATUS_A::CUSTOMER_KEY)
    }
    #[doc = "No resources are unlocked value."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(STATUS_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - LOCK Status register. This register is a bitmask for which resources are currently unlocked. These bits are one-hot per resource."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - LOCK Status register. This register is a bitmask for which resources are currently unlocked. These bits are one-hot per resource."]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LOCK Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockstat](index.html) module"]
pub struct LOCKSTAT_SPEC;
impl crate::RegisterSpec for LOCKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockstat::R](R) reader structure"]
impl crate::Readable for LOCKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockstat::W](W) writer structure"]
impl crate::Writable for LOCKSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCKSTAT to value 0"]
impl crate::Resettable for LOCKSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
