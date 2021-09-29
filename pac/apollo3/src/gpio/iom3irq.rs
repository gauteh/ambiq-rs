#[doc = "Register `IOM3IRQ` reader"]
pub struct R(crate::R<IOM3IRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOM3IRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOM3IRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOM3IRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOM3IRQ` writer"]
pub struct W(crate::W<IOM3IRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOM3IRQ_SPEC>;
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
impl From<crate::W<IOM3IRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOM3IRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOM3IRQ` reader - IOMSTR3 IRQ pad select."]
pub struct IOM3IRQ_R(crate::FieldReader<u8, u8>);
impl IOM3IRQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOM3IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOM3IRQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOM3IRQ` writer - IOMSTR3 IRQ pad select."]
pub struct IOM3IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IOM3IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - IOMSTR3 IRQ pad select."]
    #[inline(always)]
    pub fn iom3irq(&self) -> IOM3IRQ_R {
        IOM3IRQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - IOMSTR3 IRQ pad select."]
    #[inline(always)]
    pub fn iom3irq(&mut self) -> IOM3IRQ_W {
        IOM3IRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOM3 Flow Control IRQ Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iom3irq](index.html) module"]
pub struct IOM3IRQ_SPEC;
impl crate::RegisterSpec for IOM3IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iom3irq::R](R) reader structure"]
impl crate::Readable for IOM3IRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iom3irq::W](W) writer structure"]
impl crate::Writable for IOM3IRQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOM3IRQ to value 0x3f"]
impl crate::Resettable for IOM3IRQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
