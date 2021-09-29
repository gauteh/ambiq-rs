#[doc = "Register `DEVCFG` reader"]
pub struct R(crate::R<DEVCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVCFG` writer"]
pub struct W(crate::W<DEVCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVCFG_SPEC>;
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
impl From<crate::W<DEVCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVADDR` reader - I2C address of the device that the Master will use to target for read/write operations. This can be either a 7b or 10b address."]
pub struct DEVADDR_R(crate::FieldReader<u16, u16>);
impl DEVADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        DEVADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVADDR` writer - I2C address of the device that the Master will use to target for read/write operations. This can be either a 7b or 10b address."]
pub struct DEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C address of the device that the Master will use to target for read/write operations. This can be either a 7b or 10b address."]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C address of the device that the Master will use to target for read/write operations. This can be either a 7b or 10b address."]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Device Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devcfg](index.html) module"]
pub struct DEVCFG_SPEC;
impl crate::RegisterSpec for DEVCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devcfg::R](R) reader structure"]
impl crate::Readable for DEVCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devcfg::W](W) writer structure"]
impl crate::Writable for DEVCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVCFG to value 0"]
impl crate::Resettable for DEVCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
