#[doc = "Register `DMASRAMWRITEPROTECT0` reader"]
pub struct R(crate::R<DMASRAMWRITEPROTECT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASRAMWRITEPROTECT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASRAMWRITEPROTECT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASRAMWRITEPROTECT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASRAMWRITEPROTECT0` writer"]
pub struct W(crate::W<DMASRAMWRITEPROTECT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASRAMWRITEPROTECT0_SPEC>;
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
impl From<crate::W<DMASRAMWRITEPROTECT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASRAMWRITEPROTECT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_WPROT0` reader - Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
pub struct DMA_WPROT0_R(crate::FieldReader<u32, u32>);
impl DMA_WPROT0_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMA_WPROT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_WPROT0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_WPROT0` writer - Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
pub struct DMA_WPROT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_WPROT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
    #[inline(always)]
    pub fn dma_wprot0(&self) -> DMA_WPROT0_R {
        DMA_WPROT0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
    #[inline(always)]
    pub fn dma_wprot0(&mut self) -> DMA_WPROT0_W {
        DMA_WPROT0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM write-protection bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasramwriteprotect0](index.html) module"]
pub struct DMASRAMWRITEPROTECT0_SPEC;
impl crate::RegisterSpec for DMASRAMWRITEPROTECT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasramwriteprotect0::R](R) reader structure"]
impl crate::Readable for DMASRAMWRITEPROTECT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmasramwriteprotect0::W](W) writer structure"]
impl crate::Writable for DMASRAMWRITEPROTECT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMASRAMWRITEPROTECT0 to value 0"]
impl crate::Resettable for DMASRAMWRITEPROTECT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
