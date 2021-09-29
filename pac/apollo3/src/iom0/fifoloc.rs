#[doc = "Register `FIFOLOC` reader"]
pub struct R(crate::R<FIFOLOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOLOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOLOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOLOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOLOC` writer"]
pub struct W(crate::W<FIFOLOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOLOC_SPEC>;
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
impl From<crate::W<FIFOLOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOLOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFORPTR` reader - Current FIFO read pointer. Used to index into the incoming FIFO (FIFO1), which is used to store read data returned from external devices during a read operation."]
pub struct FIFORPTR_R(crate::FieldReader<u8, u8>);
impl FIFORPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFORPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFORPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFORPTR` writer - Current FIFO read pointer. Used to index into the incoming FIFO (FIFO1), which is used to store read data returned from external devices during a read operation."]
pub struct FIFORPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFORPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `FIFOWPTR` reader - Current FIFO write pointer. Value is the index into the outgoing FIFO (FIFO0), which is used during write operations to external devices."]
pub struct FIFOWPTR_R(crate::FieldReader<u8, u8>);
impl FIFOWPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOWPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOWPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOWPTR` writer - Current FIFO write pointer. Value is the index into the outgoing FIFO (FIFO0), which is used during write operations to external devices."]
pub struct FIFOWPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOWPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - Current FIFO read pointer. Used to index into the incoming FIFO (FIFO1), which is used to store read data returned from external devices during a read operation."]
    #[inline(always)]
    pub fn fiforptr(&self) -> FIFORPTR_R {
        FIFORPTR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Current FIFO write pointer. Value is the index into the outgoing FIFO (FIFO0), which is used during write operations to external devices."]
    #[inline(always)]
    pub fn fifowptr(&self) -> FIFOWPTR_R {
        FIFOWPTR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Current FIFO read pointer. Used to index into the incoming FIFO (FIFO1), which is used to store read data returned from external devices during a read operation."]
    #[inline(always)]
    pub fn fiforptr(&mut self) -> FIFORPTR_W {
        FIFORPTR_W { w: self }
    }
    #[doc = "Bits 0:3 - Current FIFO write pointer. Value is the index into the outgoing FIFO (FIFO0), which is used during write operations to external devices."]
    #[inline(always)]
    pub fn fifowptr(&mut self) -> FIFOWPTR_W {
        FIFOWPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Pointers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoloc](index.html) module"]
pub struct FIFOLOC_SPEC;
impl crate::RegisterSpec for FIFOLOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoloc::R](R) reader structure"]
impl crate::Readable for FIFOLOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoloc::W](W) writer structure"]
impl crate::Writable for FIFOLOC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOLOC to value 0"]
impl crate::Resettable for FIFOLOC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
