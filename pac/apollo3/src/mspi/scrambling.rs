#[doc = "Register `SCRAMBLING` reader"]
pub struct R(crate::R<SCRAMBLING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCRAMBLING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCRAMBLING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCRAMBLING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCRAMBLING` writer"]
pub struct W(crate::W<SCRAMBLING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCRAMBLING_SPEC>;
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
impl From<crate::W<SCRAMBLING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCRAMBLING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRENABLE` reader - Enables Data Scrambling Region. When 1 reads and writes to the range will be scrambled. When 0, data will be read/written unmodified. Address range is specified in 64K granularity and the START/END ranges are included within the range."]
pub struct SCRENABLE_R(crate::FieldReader<bool, bool>);
impl SCRENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCRENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRENABLE` writer - Enables Data Scrambling Region. When 1 reads and writes to the range will be scrambled. When 0, data will be read/written unmodified. Address range is specified in 64K granularity and the START/END ranges are included within the range."]
pub struct SCRENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `SCREND` reader - Scrambling region end address \\[25:16\\]
(64K block granularity). The END block is the LAST block included in the scrambled address range."]
pub struct SCREND_R(crate::FieldReader<u16, u16>);
impl SCREND_R {
    pub(crate) fn new(bits: u16) -> Self {
        SCREND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCREND_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCREND` writer - Scrambling region end address \\[25:16\\]
(64K block granularity). The END block is the LAST block included in the scrambled address range."]
pub struct SCREND_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `SCRSTART` reader - Scrambling region start address \\[25:16\\]
(64K block granularity). The START block is the FIRST block included in the scrambled address range."]
pub struct SCRSTART_R(crate::FieldReader<u16, u16>);
impl SCRSTART_R {
    pub(crate) fn new(bits: u16) -> Self {
        SCRSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRSTART_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRSTART` writer - Scrambling region start address \\[25:16\\]
(64K block granularity). The START block is the FIRST block included in the scrambled address range."]
pub struct SCRSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRSTART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Enables Data Scrambling Region. When 1 reads and writes to the range will be scrambled. When 0, data will be read/written unmodified. Address range is specified in 64K granularity and the START/END ranges are included within the range."]
    #[inline(always)]
    pub fn screnable(&self) -> SCRENABLE_R {
        SCRENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - Scrambling region end address \\[25:16\\]
(64K block granularity). The END block is the LAST block included in the scrambled address range."]
    #[inline(always)]
    pub fn scrend(&self) -> SCREND_R {
        SCREND_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - Scrambling region start address \\[25:16\\]
(64K block granularity). The START block is the FIRST block included in the scrambled address range."]
    #[inline(always)]
    pub fn scrstart(&self) -> SCRSTART_R {
        SCRSTART_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Enables Data Scrambling Region. When 1 reads and writes to the range will be scrambled. When 0, data will be read/written unmodified. Address range is specified in 64K granularity and the START/END ranges are included within the range."]
    #[inline(always)]
    pub fn screnable(&mut self) -> SCRENABLE_W {
        SCRENABLE_W { w: self }
    }
    #[doc = "Bits 16:25 - Scrambling region end address \\[25:16\\]
(64K block granularity). The END block is the LAST block included in the scrambled address range."]
    #[inline(always)]
    pub fn scrend(&mut self) -> SCREND_W {
        SCREND_W { w: self }
    }
    #[doc = "Bits 0:9 - Scrambling region start address \\[25:16\\]
(64K block granularity). The START block is the FIRST block included in the scrambled address range."]
    #[inline(always)]
    pub fn scrstart(&mut self) -> SCRSTART_W {
        SCRSTART_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Flash Scrambling Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scrambling](index.html) module"]
pub struct SCRAMBLING_SPEC;
impl crate::RegisterSpec for SCRAMBLING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scrambling::R](R) reader structure"]
impl crate::Readable for SCRAMBLING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scrambling::W](W) writer structure"]
impl crate::Writable for SCRAMBLING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCRAMBLING to value 0"]
impl crate::Resettable for SCRAMBLING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
