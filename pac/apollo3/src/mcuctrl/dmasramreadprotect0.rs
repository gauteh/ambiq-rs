#[doc = "Register `DMASRAMREADPROTECT0` reader"]
pub struct R(crate::R<DMASRAMREADPROTECT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASRAMREADPROTECT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASRAMREADPROTECT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASRAMREADPROTECT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASRAMREADPROTECT0` writer"]
pub struct W(crate::W<DMASRAMREADPROTECT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASRAMREADPROTECT0_SPEC>;
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
impl From<crate::W<DMASRAMREADPROTECT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASRAMREADPROTECT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_RPROT0` reader - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
pub struct DMA_RPROT0_R(crate::FieldReader<u32, u32>);
impl DMA_RPROT0_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMA_RPROT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_RPROT0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_RPROT0` writer - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
pub struct DMA_RPROT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RPROT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    pub fn dma_rprot0(&self) -> DMA_RPROT0_R {
        DMA_RPROT0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    pub fn dma_rprot0(&mut self) -> DMA_RPROT0_W {
        DMA_RPROT0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM read-protection bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasramreadprotect0](index.html) module"]
pub struct DMASRAMREADPROTECT0_SPEC;
impl crate::RegisterSpec for DMASRAMREADPROTECT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasramreadprotect0::R](R) reader structure"]
impl crate::Readable for DMASRAMREADPROTECT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmasramreadprotect0::W](W) writer structure"]
impl crate::Writable for DMASRAMREADPROTECT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMASRAMREADPROTECT0 to value 0"]
impl crate::Resettable for DMASRAMREADPROTECT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
