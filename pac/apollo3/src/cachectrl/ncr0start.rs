#[doc = "Register `NCR0START` reader"]
pub struct R(crate::R<NCR0START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NCR0START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NCR0START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NCR0START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NCR0START` writer"]
pub struct W(crate::W<NCR0START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NCR0START_SPEC>;
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
impl From<crate::W<NCR0START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NCR0START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Start address for non-cacheable region 0"]
pub struct ADDR_R(crate::FieldReader<u32, u32>);
impl ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - Start address for non-cacheable region 0"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 4)) | ((value as u32 & 0x007f_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:26 - Start address for non-cacheable region 0"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 4) & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:26 - Start address for non-cacheable region 0"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Cache Noncachable Region 0 Start\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncr0start](index.html) module"]
pub struct NCR0START_SPEC;
impl crate::RegisterSpec for NCR0START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ncr0start::R](R) reader structure"]
impl crate::Readable for NCR0START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ncr0start::W](W) writer structure"]
impl crate::Writable for NCR0START_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NCR0START to value 0"]
impl crate::Resettable for NCR0START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
