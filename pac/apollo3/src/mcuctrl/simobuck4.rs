#[doc = "Register `SIMOBUCK4` reader"]
pub struct R(crate::R<SIMOBUCK4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIMOBUCK4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIMOBUCK4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIMOBUCK4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIMOBUCK4` writer"]
pub struct W(crate::W<SIMOBUCK4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIMOBUCK4_SPEC>;
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
impl From<crate::W<SIMOBUCK4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIMOBUCK4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIMOBUCKCLKDIVSEL` reader - simobuck_clkdiv_sel"]
pub struct SIMOBUCKCLKDIVSEL_R(crate::FieldReader<u8, u8>);
impl SIMOBUCKCLKDIVSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIMOBUCKCLKDIVSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIMOBUCKCLKDIVSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIMOBUCKCLKDIVSEL` writer - simobuck_clkdiv_sel"]
pub struct SIMOBUCKCLKDIVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKCLKDIVSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 21:22 - simobuck_clkdiv_sel"]
    #[inline(always)]
    pub fn simobuckclkdivsel(&self) -> SIMOBUCKCLKDIVSEL_R {
        SIMOBUCKCLKDIVSEL_R::new(((self.bits >> 21) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 21:22 - simobuck_clkdiv_sel"]
    #[inline(always)]
    pub fn simobuckclkdivsel(&mut self) -> SIMOBUCKCLKDIVSEL_W {
        SIMOBUCKCLKDIVSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SIMO Buck Control Reg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [simobuck4](index.html) module"]
pub struct SIMOBUCK4_SPEC;
impl crate::RegisterSpec for SIMOBUCK4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [simobuck4::R](R) reader structure"]
impl crate::Readable for SIMOBUCK4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [simobuck4::W](W) writer structure"]
impl crate::Writable for SIMOBUCK4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIMOBUCK4 to value 0x3c8d_80aa"]
impl crate::Resettable for SIMOBUCK4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3c8d_80aa
    }
}
