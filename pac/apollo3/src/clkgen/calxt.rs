#[doc = "Register `CALXT` reader"]
pub struct R(crate::R<CALXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALXT` writer"]
pub struct W(crate::W<CALXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALXT_SPEC>;
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
impl From<crate::W<CALXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALXT` reader - XT Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 16KHz clock derived from the original 32KHz version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The maximum value that is effective is from -1024 to 1023."]
pub struct CALXT_R(crate::FieldReader<u16, u16>);
impl CALXT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CALXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALXT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALXT` writer - XT Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 16KHz clock derived from the original 32KHz version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The maximum value that is effective is from -1024 to 1023."]
pub struct CALXT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - XT Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 16KHz clock derived from the original 32KHz version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The maximum value that is effective is from -1024 to 1023."]
    #[inline(always)]
    pub fn calxt(&self) -> CALXT_R {
        CALXT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - XT Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 16KHz clock derived from the original 32KHz version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The maximum value that is effective is from -1024 to 1023."]
    #[inline(always)]
    pub fn calxt(&mut self) -> CALXT_W {
        CALXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XT Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calxt](index.html) module"]
pub struct CALXT_SPEC;
impl crate::RegisterSpec for CALXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calxt::R](R) reader structure"]
impl crate::Readable for CALXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calxt::W](W) writer structure"]
impl crate::Writable for CALXT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALXT to value 0"]
impl crate::Resettable for CALXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
