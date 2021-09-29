#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XCMPWR` reader - Transfer complete interrupt, write to register space."]
pub struct XCMPWR_R(crate::FieldReader<bool, bool>);
impl XCMPWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        XCMPWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XCMPWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XCMPWR` writer - Transfer complete interrupt, write to register space."]
pub struct XCMPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> XCMPWR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `XCMPWF` reader - Transfer complete interrupt, write to FIFO space."]
pub struct XCMPWF_R(crate::FieldReader<bool, bool>);
impl XCMPWF_R {
    pub(crate) fn new(bits: bool) -> Self {
        XCMPWF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XCMPWF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XCMPWF` writer - Transfer complete interrupt, write to FIFO space."]
pub struct XCMPWF_W<'a> {
    w: &'a mut W,
}
impl<'a> XCMPWF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `XCMPRR` reader - Transfer complete interrupt, read from register space."]
pub struct XCMPRR_R(crate::FieldReader<bool, bool>);
impl XCMPRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        XCMPRR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XCMPRR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XCMPRR` writer - Transfer complete interrupt, read from register space."]
pub struct XCMPRR_W<'a> {
    w: &'a mut W,
}
impl<'a> XCMPRR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `XCMPRF` reader - Transfer complete interrupt, read from FIFO space."]
pub struct XCMPRF_R(crate::FieldReader<bool, bool>);
impl XCMPRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        XCMPRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XCMPRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XCMPRF` writer - Transfer complete interrupt, read from FIFO space."]
pub struct XCMPRF_W<'a> {
    w: &'a mut W,
}
impl<'a> XCMPRF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `IOINTW` reader - IO Write interrupt."]
pub struct IOINTW_R(crate::FieldReader<bool, bool>);
impl IOINTW_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOINTW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOINTW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOINTW` writer - IO Write interrupt."]
pub struct IOINTW_W<'a> {
    w: &'a mut W,
}
impl<'a> IOINTW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `GENAD` reader - I2C General Address interrupt."]
pub struct GENAD_R(crate::FieldReader<bool, bool>);
impl GENAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENAD` writer - I2C General Address interrupt."]
pub struct GENAD_W<'a> {
    w: &'a mut W,
}
impl<'a> GENAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `FRDERR` reader - FIFO Read Error interrupt."]
pub struct FRDERR_R(crate::FieldReader<bool, bool>);
impl FRDERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRDERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRDERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRDERR` writer - FIFO Read Error interrupt."]
pub struct FRDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRDERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `FUNDFL` reader - FIFO Underflow interrupt."]
pub struct FUNDFL_R(crate::FieldReader<bool, bool>);
impl FUNDFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUNDFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUNDFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNDFL` writer - FIFO Underflow interrupt."]
pub struct FUNDFL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNDFL_W<'a> {
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
#[doc = "Field `FOVFL` reader - FIFO Overflow interrupt."]
pub struct FOVFL_R(crate::FieldReader<bool, bool>);
impl FOVFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOVFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FOVFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOVFL` writer - FIFO Overflow interrupt."]
pub struct FOVFL_W<'a> {
    w: &'a mut W,
}
impl<'a> FOVFL_W<'a> {
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
#[doc = "Field `FSIZE` reader - FIFO Size interrupt."]
pub struct FSIZE_R(crate::FieldReader<bool, bool>);
impl FSIZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSIZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSIZE` writer - FIFO Size interrupt."]
pub struct FSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSIZE_W<'a> {
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
    #[doc = "Bit 9 - Transfer complete interrupt, write to register space."]
    #[inline(always)]
    pub fn xcmpwr(&self) -> XCMPWR_R {
        XCMPWR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transfer complete interrupt, write to FIFO space."]
    #[inline(always)]
    pub fn xcmpwf(&self) -> XCMPWF_R {
        XCMPWF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transfer complete interrupt, read from register space."]
    #[inline(always)]
    pub fn xcmprr(&self) -> XCMPRR_R {
        XCMPRR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transfer complete interrupt, read from FIFO space."]
    #[inline(always)]
    pub fn xcmprf(&self) -> XCMPRF_R {
        XCMPRF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IO Write interrupt."]
    #[inline(always)]
    pub fn iointw(&self) -> IOINTW_R {
        IOINTW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C General Address interrupt."]
    #[inline(always)]
    pub fn genad(&self) -> GENAD_R {
        GENAD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO Read Error interrupt."]
    #[inline(always)]
    pub fn frderr(&self) -> FRDERR_R {
        FRDERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO Underflow interrupt."]
    #[inline(always)]
    pub fn fundfl(&self) -> FUNDFL_R {
        FUNDFL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO Overflow interrupt."]
    #[inline(always)]
    pub fn fovfl(&self) -> FOVFL_R {
        FOVFL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - FIFO Size interrupt."]
    #[inline(always)]
    pub fn fsize(&self) -> FSIZE_R {
        FSIZE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Transfer complete interrupt, write to register space."]
    #[inline(always)]
    pub fn xcmpwr(&mut self) -> XCMPWR_W {
        XCMPWR_W { w: self }
    }
    #[doc = "Bit 8 - Transfer complete interrupt, write to FIFO space."]
    #[inline(always)]
    pub fn xcmpwf(&mut self) -> XCMPWF_W {
        XCMPWF_W { w: self }
    }
    #[doc = "Bit 7 - Transfer complete interrupt, read from register space."]
    #[inline(always)]
    pub fn xcmprr(&mut self) -> XCMPRR_W {
        XCMPRR_W { w: self }
    }
    #[doc = "Bit 6 - Transfer complete interrupt, read from FIFO space."]
    #[inline(always)]
    pub fn xcmprf(&mut self) -> XCMPRF_W {
        XCMPRF_W { w: self }
    }
    #[doc = "Bit 5 - IO Write interrupt."]
    #[inline(always)]
    pub fn iointw(&mut self) -> IOINTW_W {
        IOINTW_W { w: self }
    }
    #[doc = "Bit 4 - I2C General Address interrupt."]
    #[inline(always)]
    pub fn genad(&mut self) -> GENAD_W {
        GENAD_W { w: self }
    }
    #[doc = "Bit 3 - FIFO Read Error interrupt."]
    #[inline(always)]
    pub fn frderr(&mut self) -> FRDERR_W {
        FRDERR_W { w: self }
    }
    #[doc = "Bit 2 - FIFO Underflow interrupt."]
    #[inline(always)]
    pub fn fundfl(&mut self) -> FUNDFL_W {
        FUNDFL_W { w: self }
    }
    #[doc = "Bit 1 - FIFO Overflow interrupt."]
    #[inline(always)]
    pub fn fovfl(&mut self) -> FOVFL_W {
        FOVFL_W { w: self }
    }
    #[doc = "Bit 0 - FIFO Size interrupt."]
    #[inline(always)]
    pub fn fsize(&mut self) -> FSIZE_W {
        FSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO Slave Interrupts: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
