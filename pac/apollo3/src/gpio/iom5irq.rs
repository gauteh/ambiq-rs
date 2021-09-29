#[doc = "Register `IOM5IRQ` reader"]
pub struct R(crate::R<IOM5IRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOM5IRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOM5IRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOM5IRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOM5IRQ` writer"]
pub struct W(crate::W<IOM5IRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOM5IRQ_SPEC>;
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
impl From<crate::W<IOM5IRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOM5IRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOM5IRQ` reader - IOMSTR5 IRQ pad select."]
pub struct IOM5IRQ_R(crate::FieldReader<u8, u8>);
impl IOM5IRQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOM5IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOM5IRQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOM5IRQ` writer - IOMSTR5 IRQ pad select."]
pub struct IOM5IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IOM5IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - IOMSTR5 IRQ pad select."]
    #[inline(always)]
    pub fn iom5irq(&self) -> IOM5IRQ_R {
        IOM5IRQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - IOMSTR5 IRQ pad select."]
    #[inline(always)]
    pub fn iom5irq(&mut self) -> IOM5IRQ_W {
        IOM5IRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOM5 Flow Control IRQ Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iom5irq](index.html) module"]
pub struct IOM5IRQ_SPEC;
impl crate::RegisterSpec for IOM5IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iom5irq::R](R) reader structure"]
impl crate::Readable for IOM5IRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iom5irq::W](W) writer structure"]
impl crate::Writable for IOM5IRQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOM5IRQ to value 0x3f"]
impl crate::Resettable for IOM5IRQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
