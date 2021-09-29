#[doc = "Register `DMASRAMREADPROTECT1` reader"]
pub struct R(crate::R<DMASRAMREADPROTECT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASRAMREADPROTECT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASRAMREADPROTECT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASRAMREADPROTECT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASRAMREADPROTECT1` writer"]
pub struct W(crate::W<DMASRAMREADPROTECT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASRAMREADPROTECT1_SPEC>;
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
impl From<crate::W<DMASRAMREADPROTECT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASRAMREADPROTECT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_RPROT1` reader - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
pub struct DMA_RPROT1_R(crate::FieldReader<u16, u16>);
impl DMA_RPROT1_R {
    pub(crate) fn new(bits: u16) -> Self {
        DMA_RPROT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_RPROT1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_RPROT1` writer - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
pub struct DMA_RPROT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RPROT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    pub fn dma_rprot1(&self) -> DMA_RPROT1_R {
        DMA_RPROT1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    pub fn dma_rprot1(&mut self) -> DMA_RPROT1_W {
        DMA_RPROT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM read-protection bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasramreadprotect1](index.html) module"]
pub struct DMASRAMREADPROTECT1_SPEC;
impl crate::RegisterSpec for DMASRAMREADPROTECT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasramreadprotect1::R](R) reader structure"]
impl crate::Readable for DMASRAMREADPROTECT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmasramreadprotect1::W](W) writer structure"]
impl crate::Writable for DMASRAMREADPROTECT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMASRAMREADPROTECT1 to value 0"]
impl crate::Resettable for DMASRAMREADPROTECT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
