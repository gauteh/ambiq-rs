#[doc = "Register `CHIPREV` reader"]
pub struct R(crate::R<CHIPREV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIPREV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIPREV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIPREV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHIPREV` writer"]
pub struct W(crate::W<CHIPREV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHIPREV_SPEC>;
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
impl From<crate::W<CHIPREV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHIPREV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIPART` reader - Silicon Part ID"]
pub struct SIPART_R(crate::FieldReader<u16, u16>);
impl SIPART_R {
    pub(crate) fn new(bits: u16) -> Self {
        SIPART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIPART_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIPART` writer - Silicon Part ID"]
pub struct SIPART_W<'a> {
    w: &'a mut W,
}
impl<'a> SIPART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | ((value as u32 & 0x0fff) << 8);
        self.w
    }
}
#[doc = "Major Revision ID.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REVMAJ_A {
    #[doc = "2: Apollo3 revision B value."]
    B = 2,
    #[doc = "1: Apollo3 revision A value."]
    A = 1,
}
impl From<REVMAJ_A> for u8 {
    #[inline(always)]
    fn from(variant: REVMAJ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REVMAJ` reader - Major Revision ID."]
pub struct REVMAJ_R(crate::FieldReader<u8, REVMAJ_A>);
impl REVMAJ_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVMAJ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REVMAJ_A> {
        match self.bits {
            2 => Some(REVMAJ_A::B),
            1 => Some(REVMAJ_A::A),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        **self == REVMAJ_A::B
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        **self == REVMAJ_A::A
    }
}
impl core::ops::Deref for REVMAJ_R {
    type Target = crate::FieldReader<u8, REVMAJ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVMAJ` writer - Major Revision ID."]
pub struct REVMAJ_W<'a> {
    w: &'a mut W,
}
impl<'a> REVMAJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REVMAJ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo3 revision B value."]
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(REVMAJ_A::B)
    }
    #[doc = "Apollo3 revision A value."]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(REVMAJ_A::A)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Minor Revision ID.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REVMIN_A {
    #[doc = "2: Apollo3 minor rev 1. value."]
    REV1 = 2,
    #[doc = "1: Apollo3 minor rev 0.  Minor revision value, succeeding minor revisions will increment from this value. value."]
    REV0 = 1,
}
impl From<REVMIN_A> for u8 {
    #[inline(always)]
    fn from(variant: REVMIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REVMIN` reader - Minor Revision ID."]
pub struct REVMIN_R(crate::FieldReader<u8, REVMIN_A>);
impl REVMIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVMIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REVMIN_A> {
        match self.bits {
            2 => Some(REVMIN_A::REV1),
            1 => Some(REVMIN_A::REV0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REV1`"]
    #[inline(always)]
    pub fn is_rev1(&self) -> bool {
        **self == REVMIN_A::REV1
    }
    #[doc = "Checks if the value of the field is `REV0`"]
    #[inline(always)]
    pub fn is_rev0(&self) -> bool {
        **self == REVMIN_A::REV0
    }
}
impl core::ops::Deref for REVMIN_R {
    type Target = crate::FieldReader<u8, REVMIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVMIN` writer - Minor Revision ID."]
pub struct REVMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> REVMIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REVMIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo3 minor rev 1. value."]
    #[inline(always)]
    pub fn rev1(self) -> &'a mut W {
        self.variant(REVMIN_A::REV1)
    }
    #[doc = "Apollo3 minor rev 0. Minor revision value, succeeding minor revisions will increment from this value. value."]
    #[inline(always)]
    pub fn rev0(self) -> &'a mut W {
        self.variant(REVMIN_A::REV0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:19 - Silicon Part ID"]
    #[inline(always)]
    pub fn sipart(&self) -> SIPART_R {
        SIPART_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 4:7 - Major Revision ID."]
    #[inline(always)]
    pub fn revmaj(&self) -> REVMAJ_R {
        REVMAJ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Minor Revision ID."]
    #[inline(always)]
    pub fn revmin(&self) -> REVMIN_R {
        REVMIN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:19 - Silicon Part ID"]
    #[inline(always)]
    pub fn sipart(&mut self) -> SIPART_W {
        SIPART_W { w: self }
    }
    #[doc = "Bits 4:7 - Major Revision ID."]
    #[inline(always)]
    pub fn revmaj(&mut self) -> REVMAJ_W {
        REVMAJ_W { w: self }
    }
    #[doc = "Bits 0:3 - Minor Revision ID."]
    #[inline(always)]
    pub fn revmin(&mut self) -> REVMIN_W {
        REVMIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Revision\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chiprev](index.html) module"]
pub struct CHIPREV_SPEC;
impl crate::RegisterSpec for CHIPREV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chiprev::R](R) reader structure"]
impl crate::Readable for CHIPREV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chiprev::W](W) writer structure"]
impl crate::Writable for CHIPREV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHIPREV to value 0x01"]
impl crate::Resettable for CHIPREV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
