#[doc = "Register `SRAMMODE` reader"]
pub struct R(crate::R<SRAMMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMMODE` writer"]
pub struct W(crate::W<SRAMMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMMODE_SPEC>;
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
impl From<crate::W<SRAMMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPREFETCH_CACHE` reader - Secondary prefetch feature that will cache prefetched data across bus waitstates (requires DPREFETCH to be set)."]
pub struct DPREFETCH_CACHE_R(crate::FieldReader<bool, bool>);
impl DPREFETCH_CACHE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPREFETCH_CACHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPREFETCH_CACHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPREFETCH_CACHE` writer - Secondary prefetch feature that will cache prefetched data across bus waitstates (requires DPREFETCH to be set)."]
pub struct DPREFETCH_CACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPREFETCH_CACHE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DPREFETCH` reader - When set, data bus accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Use of this mode bit is only recommended if the work flow has a large number of sequential accesses."]
pub struct DPREFETCH_R(crate::FieldReader<bool, bool>);
impl DPREFETCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPREFETCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPREFETCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPREFETCH` writer - When set, data bus accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Use of this mode bit is only recommended if the work flow has a large number of sequential accesses."]
pub struct DPREFETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> DPREFETCH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `IPREFETCH_CACHE` reader - Secondary prefetch feature that will cache prefetched data across bus waitstates (requires IPREFETCH to be set)."]
pub struct IPREFETCH_CACHE_R(crate::FieldReader<bool, bool>);
impl IPREFETCH_CACHE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPREFETCH_CACHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPREFETCH_CACHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPREFETCH_CACHE` writer - Secondary prefetch feature that will cache prefetched data across bus waitstates (requires IPREFETCH to be set)."]
pub struct IPREFETCH_CACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREFETCH_CACHE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `IPREFETCH` reader - When set, instruction accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Generally, this mode bit should be set for improved performance when executing instructions from SRAM."]
pub struct IPREFETCH_R(crate::FieldReader<bool, bool>);
impl IPREFETCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPREFETCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPREFETCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPREFETCH` writer - When set, instruction accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Generally, this mode bit should be set for improved performance when executing instructions from SRAM."]
pub struct IPREFETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREFETCH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Secondary prefetch feature that will cache prefetched data across bus waitstates (requires DPREFETCH to be set)."]
    #[inline(always)]
    pub fn dprefetch_cache(&self) -> DPREFETCH_CACHE_R {
        DPREFETCH_CACHE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When set, data bus accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Use of this mode bit is only recommended if the work flow has a large number of sequential accesses."]
    #[inline(always)]
    pub fn dprefetch(&self) -> DPREFETCH_R {
        DPREFETCH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Secondary prefetch feature that will cache prefetched data across bus waitstates (requires IPREFETCH to be set)."]
    #[inline(always)]
    pub fn iprefetch_cache(&self) -> IPREFETCH_CACHE_R {
        IPREFETCH_CACHE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - When set, instruction accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Generally, this mode bit should be set for improved performance when executing instructions from SRAM."]
    #[inline(always)]
    pub fn iprefetch(&self) -> IPREFETCH_R {
        IPREFETCH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Secondary prefetch feature that will cache prefetched data across bus waitstates (requires DPREFETCH to be set)."]
    #[inline(always)]
    pub fn dprefetch_cache(&mut self) -> DPREFETCH_CACHE_W {
        DPREFETCH_CACHE_W { w: self }
    }
    #[doc = "Bit 4 - When set, data bus accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Use of this mode bit is only recommended if the work flow has a large number of sequential accesses."]
    #[inline(always)]
    pub fn dprefetch(&mut self) -> DPREFETCH_W {
        DPREFETCH_W { w: self }
    }
    #[doc = "Bit 1 - Secondary prefetch feature that will cache prefetched data across bus waitstates (requires IPREFETCH to be set)."]
    #[inline(always)]
    pub fn iprefetch_cache(&mut self) -> IPREFETCH_CACHE_W {
        IPREFETCH_CACHE_W { w: self }
    }
    #[doc = "Bit 0 - When set, instruction accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Generally, this mode bit should be set for improved performance when executing instructions from SRAM."]
    #[inline(always)]
    pub fn iprefetch(&mut self) -> IPREFETCH_W {
        IPREFETCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Controller mode bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srammode](index.html) module"]
pub struct SRAMMODE_SPEC;
impl crate::RegisterSpec for SRAMMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srammode::R](R) reader structure"]
impl crate::Readable for SRAMMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srammode::W](W) writer structure"]
impl crate::Writable for SRAMMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAMMODE to value 0"]
impl crate::Resettable for SRAMMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
