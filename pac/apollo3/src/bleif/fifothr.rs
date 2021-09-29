#[doc = "Register `FIFOTHR` reader"]
pub struct R(crate::R<FIFOTHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOTHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOTHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOTHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOTHR` writer"]
pub struct W(crate::W<FIFOTHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOTHR_SPEC>;
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
impl From<crate::W<FIFOTHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOTHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOWTHR` reader - FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
pub struct FIFOWTHR_R(crate::FieldReader<u8, u8>);
impl FIFOWTHR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOWTHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOWTHR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOWTHR` writer - FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
pub struct FIFOWTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOWTHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `FIFORTHR` reader - FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
pub struct FIFORTHR_R(crate::FieldReader<u8, u8>);
impl FIFORTHR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFORTHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFORTHR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFORTHR` writer - FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
pub struct FIFORTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFORTHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13 - FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
    #[inline(always)]
    pub fn fifowthr(&self) -> FIFOWTHR_R {
        FIFOWTHR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
    #[inline(always)]
    pub fn fiforthr(&self) -> FIFORTHR_R {
        FIFORTHR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:13 - FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
    #[inline(always)]
    pub fn fifowthr(&mut self) -> FIFOWTHR_W {
        FIFOWTHR_W { w: self }
    }
    #[doc = "Bits 0:5 - FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
    #[inline(always)]
    pub fn fiforthr(&mut self) -> FIFORTHR_W {
        FIFORTHR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Threshold Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifothr](index.html) module"]
pub struct FIFOTHR_SPEC;
impl crate::RegisterSpec for FIFOTHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifothr::R](R) reader structure"]
impl crate::Readable for FIFOTHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifothr::W](W) writer structure"]
impl crate::Writable for FIFOTHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOTHR to value 0"]
impl crate::Resettable for FIFOTHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
