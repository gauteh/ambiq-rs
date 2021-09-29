#[doc = "Register `CALRC` reader"]
pub struct R(crate::R<CALRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALRC` writer"]
pub struct W(crate::W<CALRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALRC_SPEC>;
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
impl From<crate::W<CALRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALRC` reader - LFRC Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 512 Hz clock derived from the original 1024 version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The range is from -131072 (decimal) to 131071 (decimal). This register is normally used in conjuction with ACALCTR register. The CALRC register will load the ACALCTR register (bits 17:0) if the ACALCTR register is set to measure the LFRC with the XT clock."]
pub struct CALRC_R(crate::FieldReader<u32, u32>);
impl CALRC_R {
    pub(crate) fn new(bits: u32) -> Self {
        CALRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALRC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALRC` writer - LFRC Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 512 Hz clock derived from the original 1024 version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The range is from -131072 (decimal) to 131071 (decimal). This register is normally used in conjuction with ACALCTR register. The CALRC register will load the ACALCTR register (bits 17:0) if the ACALCTR register is set to measure the LFRC with the XT clock."]
pub struct CALRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - LFRC Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 512 Hz clock derived from the original 1024 version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The range is from -131072 (decimal) to 131071 (decimal). This register is normally used in conjuction with ACALCTR register. The CALRC register will load the ACALCTR register (bits 17:0) if the ACALCTR register is set to measure the LFRC with the XT clock."]
    #[inline(always)]
    pub fn calrc(&self) -> CALRC_R {
        CALRC_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - LFRC Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 512 Hz clock derived from the original 1024 version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The range is from -131072 (decimal) to 131071 (decimal). This register is normally used in conjuction with ACALCTR register. The CALRC register will load the ACALCTR register (bits 17:0) if the ACALCTR register is set to measure the LFRC with the XT clock."]
    #[inline(always)]
    pub fn calrc(&mut self) -> CALRC_W {
        CALRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RC Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calrc](index.html) module"]
pub struct CALRC_SPEC;
impl crate::RegisterSpec for CALRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calrc::R](R) reader structure"]
impl crate::Readable for CALRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calrc::W](W) writer structure"]
impl crate::Writable for CALRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALRC to value 0"]
impl crate::Resettable for CALRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
