#[doc = "Register `BLEDBG` reader"]
pub struct R(crate::R<BLEDBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLEDBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLEDBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLEDBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLEDBG` writer"]
pub struct W(crate::W<BLEDBG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLEDBG_SPEC>;
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
impl From<crate::W<BLEDBG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLEDBG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGDATA` reader - Debug data"]
pub struct DBGDATA_R(crate::FieldReader<u32, u32>);
impl DBGDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DBGDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGDATA` writer - Debug data"]
pub struct DBGDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Field `APBCLKON` reader - APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
pub struct APBCLKON_R(crate::FieldReader<bool, bool>);
impl APBCLKON_R {
    pub(crate) fn new(bits: bool) -> Self {
        APBCLKON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APBCLKON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APBCLKON` writer - APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
pub struct APBCLKON_W<'a> {
    w: &'a mut W,
}
impl<'a> APBCLKON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `IOCLKON` reader - IOCLK debug clock control. Enable IO_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
pub struct IOCLKON_R(crate::FieldReader<bool, bool>);
impl IOCLKON_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOCLKON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOCLKON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOCLKON` writer - IOCLK debug clock control. Enable IO_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
pub struct IOCLKON_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCLKON_W<'a> {
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
#[doc = "Field `DBGEN` reader - Debug Enable. Setting this bit will enable the update of data within this register, otherwise it is clock gated for power savings"]
pub struct DBGEN_R(crate::FieldReader<bool, bool>);
impl DBGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGEN` writer - Debug Enable. Setting this bit will enable the update of data within this register, otherwise it is clock gated for power savings"]
pub struct DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_W<'a> {
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
    #[doc = "Bits 3:31 - Debug data"]
    #[inline(always)]
    pub fn dbgdata(&self) -> DBGDATA_R {
        DBGDATA_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub fn apbclkon(&self) -> APBCLKON_R {
        APBCLKON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - IOCLK debug clock control. Enable IO_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub fn ioclkon(&self) -> IOCLKON_R {
        IOCLKON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Debug Enable. Setting this bit will enable the update of data within this register, otherwise it is clock gated for power savings"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - Debug data"]
    #[inline(always)]
    pub fn dbgdata(&mut self) -> DBGDATA_W {
        DBGDATA_W { w: self }
    }
    #[doc = "Bit 2 - APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub fn apbclkon(&mut self) -> APBCLKON_W {
        APBCLKON_W { w: self }
    }
    #[doc = "Bit 1 - IOCLK debug clock control. Enable IO_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub fn ioclkon(&mut self) -> IOCLKON_W {
        IOCLKON_W { w: self }
    }
    #[doc = "Bit 0 - Debug Enable. Setting this bit will enable the update of data within this register, otherwise it is clock gated for power savings"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLEIF Master Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bledbg](index.html) module"]
pub struct BLEDBG_SPEC;
impl crate::RegisterSpec for BLEDBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bledbg::R](R) reader structure"]
impl crate::Readable for BLEDBG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bledbg::W](W) writer structure"]
impl crate::Writable for BLEDBG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLEDBG to value 0"]
impl crate::Resettable for BLEDBG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
