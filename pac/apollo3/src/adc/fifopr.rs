#[doc = "Register `FIFOPR` reader"]
pub struct R(crate::R<FIFOPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOPR` writer"]
pub struct W(crate::W<FIFOPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOPR_SPEC>;
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
impl From<crate::W<FIFOPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSVDPR` reader - RESERVED."]
pub struct RSVDPR_R(crate::FieldReader<bool, bool>);
impl RSVDPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSVDPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVDPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSVDPR` writer - RESERVED."]
pub struct RSVDPR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVDPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `SLOTNUMPR` reader - Slot number associated with this FIFO data."]
pub struct SLOTNUMPR_R(crate::FieldReader<u8, u8>);
impl SLOTNUMPR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLOTNUMPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOTNUMPR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTNUMPR` writer - Slot number associated with this FIFO data."]
pub struct SLOTNUMPR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTNUMPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `COUNT` reader - Number of valid entries in the ADC FIFO."]
pub struct COUNT_R(crate::FieldReader<u8, u8>);
impl COUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT` writer - Number of valid entries in the ADC FIFO."]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | ((value as u32 & 0xff) << 20);
        self.w
    }
}
#[doc = "Field `DATA` reader - Oldest data in the FIFO."]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - Oldest data in the FIFO."]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - RESERVED."]
    #[inline(always)]
    pub fn rsvdpr(&self) -> RSVDPR_R {
        RSVDPR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Slot number associated with this FIFO data."]
    #[inline(always)]
    pub fn slotnumpr(&self) -> SLOTNUMPR_R {
        SLOTNUMPR_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 20:27 - Number of valid entries in the ADC FIFO."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 0:19 - Oldest data in the FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - RESERVED."]
    #[inline(always)]
    pub fn rsvdpr(&mut self) -> RSVDPR_W {
        RSVDPR_W { w: self }
    }
    #[doc = "Bits 28:30 - Slot number associated with this FIFO data."]
    #[inline(always)]
    pub fn slotnumpr(&mut self) -> SLOTNUMPR_W {
        SLOTNUMPR_W { w: self }
    }
    #[doc = "Bits 20:27 - Number of valid entries in the ADC FIFO."]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Bits 0:19 - Oldest data in the FIFO."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Data and Valid Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifopr](index.html) module"]
pub struct FIFOPR_SPEC;
impl crate::RegisterSpec for FIFOPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifopr::R](R) reader structure"]
impl crate::Readable for FIFOPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifopr::W](W) writer structure"]
impl crate::Writable for FIFOPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOPR to value 0"]
impl crate::Resettable for FIFOPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
