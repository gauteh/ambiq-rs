#[doc = "Register `INTRPTCLR` reader"]
pub struct R(crate::R<INTRPTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTRPTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTRPTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTRPTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTRPTCLR` writer"]
pub struct W(crate::W<INTRPTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTRPTCLR_SPEC>;
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
impl From<crate::W<INTRPTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTRPTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OF` reader - XT Oscillator Fail interrupt"]
pub struct OF_R(crate::FieldReader<bool, bool>);
impl OF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OF` writer - XT Oscillator Fail interrupt"]
pub struct OF_W<'a> {
    w: &'a mut W,
}
impl<'a> OF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ACC` reader - Autocalibration Complete interrupt"]
pub struct ACC_R(crate::FieldReader<bool, bool>);
impl ACC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACC` writer - Autocalibration Complete interrupt"]
pub struct ACC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACC_W<'a> {
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
#[doc = "Field `ACF` reader - Autocalibration Fail interrupt"]
pub struct ACF_R(crate::FieldReader<bool, bool>);
impl ACF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACF` writer - Autocalibration Fail interrupt"]
pub struct ACF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACF_W<'a> {
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
    #[doc = "Bit 2 - XT Oscillator Fail interrupt"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Autocalibration Complete interrupt"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Autocalibration Fail interrupt"]
    #[inline(always)]
    pub fn acf(&self) -> ACF_R {
        ACF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - XT Oscillator Fail interrupt"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W { w: self }
    }
    #[doc = "Bit 1 - Autocalibration Complete interrupt"]
    #[inline(always)]
    pub fn acc(&mut self) -> ACC_W {
        ACC_W { w: self }
    }
    #[doc = "Bit 0 - Autocalibration Fail interrupt"]
    #[inline(always)]
    pub fn acf(&mut self) -> ACF_W {
        ACF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLKGEN Interrupt Register: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrptclr](index.html) module"]
pub struct INTRPTCLR_SPEC;
impl crate::RegisterSpec for INTRPTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intrptclr::R](R) reader structure"]
impl crate::Readable for INTRPTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intrptclr::W](W) writer structure"]
impl crate::Writable for INTRPTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTRPTCLR to value 0"]
impl crate::Resettable for INTRPTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
