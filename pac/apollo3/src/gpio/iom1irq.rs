#[doc = "Register `IOM1IRQ` reader"]
pub struct R(crate::R<IOM1IRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOM1IRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOM1IRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOM1IRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOM1IRQ` writer"]
pub struct W(crate::W<IOM1IRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOM1IRQ_SPEC>;
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
impl From<crate::W<IOM1IRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOM1IRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOM1IRQ` reader - IOMSTR1 IRQ pad select."]
pub struct IOM1IRQ_R(crate::FieldReader<u8, u8>);
impl IOM1IRQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOM1IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOM1IRQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOM1IRQ` writer - IOMSTR1 IRQ pad select."]
pub struct IOM1IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IOM1IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - IOMSTR1 IRQ pad select."]
    #[inline(always)]
    pub fn iom1irq(&self) -> IOM1IRQ_R {
        IOM1IRQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - IOMSTR1 IRQ pad select."]
    #[inline(always)]
    pub fn iom1irq(&mut self) -> IOM1IRQ_W {
        IOM1IRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOM1 Flow Control IRQ Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iom1irq](index.html) module"]
pub struct IOM1IRQ_SPEC;
impl crate::RegisterSpec for IOM1IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iom1irq::R](R) reader structure"]
impl crate::Readable for IOM1IRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iom1irq::W](W) writer structure"]
impl crate::Writable for IOM1IRQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOM1IRQ to value 0x3f"]
impl crate::Resettable for IOM1IRQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
