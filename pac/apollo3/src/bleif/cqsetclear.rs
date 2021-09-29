#[doc = "Register `CQSETCLEAR` reader"]
pub struct R(crate::R<CQSETCLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQSETCLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQSETCLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQSETCLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQSETCLEAR` writer"]
pub struct W(crate::W<CQSETCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQSETCLEAR_SPEC>;
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
impl From<crate::W<CQSETCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQSETCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CQFCLR` reader - Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field"]
pub struct CQFCLR_R(crate::FieldReader<u8, u8>);
impl CQFCLR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CQFCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQFCLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQFCLR` writer - Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field"]
pub struct CQFCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CQFCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CQFTGL` reader - Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
pub struct CQFTGL_R(crate::FieldReader<u8, u8>);
impl CQFTGL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CQFTGL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQFTGL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQFTGL` writer - Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
pub struct CQFTGL_W<'a> {
    w: &'a mut W,
}
impl<'a> CQFTGL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CQFSET` reader - Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
pub struct CQFSET_R(crate::FieldReader<u8, u8>);
impl CQFSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        CQFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQFSET` writer - Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
pub struct CQFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> CQFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqfclr(&self) -> CQFCLR_R {
        CQFCLR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqftgl(&self) -> CQFTGL_R {
        CQFTGL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqfset(&self) -> CQFSET_R {
        CQFSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqfclr(&mut self) -> CQFCLR_W {
        CQFCLR_W { w: self }
    }
    #[doc = "Bits 8:15 - Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqftgl(&mut self) -> CQFTGL_W {
        CQFTGL_W { w: self }
    }
    #[doc = "Bits 0:7 - Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqfset(&mut self) -> CQFSET_W {
        CQFSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Queue Flag Set/Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqsetclear](index.html) module"]
pub struct CQSETCLEAR_SPEC;
impl crate::RegisterSpec for CQSETCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqsetclear::R](R) reader structure"]
impl crate::Readable for CQSETCLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqsetclear::W](W) writer structure"]
impl crate::Writable for CQSETCLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQSETCLEAR to value 0"]
impl crate::Resettable for CQSETCLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
