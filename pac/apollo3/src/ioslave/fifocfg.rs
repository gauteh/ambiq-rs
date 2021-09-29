#[doc = "Register `FIFOCFG` reader"]
pub struct R(crate::R<FIFOCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOCFG` writer"]
pub struct W(crate::W<FIFOCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOCFG_SPEC>;
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
impl From<crate::W<FIFOCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROBASE` reader - Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOBASE*8-1)"]
pub struct ROBASE_R(crate::FieldReader<u8, u8>);
impl ROBASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ROBASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROBASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROBASE` writer - Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOBASE*8-1)"]
pub struct ROBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `FIFOMAX` reader - These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
pub struct FIFOMAX_R(crate::FieldReader<u8, u8>);
impl FIFOMAX_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOMAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOMAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOMAX` writer - These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
pub struct FIFOMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `FIFOBASE` reader - These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
pub struct FIFOBASE_R(crate::FieldReader<u8, u8>);
impl FIFOBASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOBASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOBASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOBASE` writer - These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
pub struct FIFOBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOBASE*8-1)"]
    #[inline(always)]
    pub fn robase(&self) -> ROBASE_R {
        ROBASE_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
    #[inline(always)]
    pub fn fifomax(&self) -> FIFOMAX_R {
        FIFOMAX_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:4 - These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
    #[inline(always)]
    pub fn fifobase(&self) -> FIFOBASE_R {
        FIFOBASE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOBASE*8-1)"]
    #[inline(always)]
    pub fn robase(&mut self) -> ROBASE_W {
        ROBASE_W { w: self }
    }
    #[doc = "Bits 8:13 - These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
    #[inline(always)]
    pub fn fifomax(&mut self) -> FIFOMAX_W {
        FIFOMAX_W { w: self }
    }
    #[doc = "Bits 0:4 - These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
    #[inline(always)]
    pub fn fifobase(&mut self) -> FIFOBASE_W {
        FIFOBASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocfg](index.html) module"]
pub struct FIFOCFG_SPEC;
impl crate::RegisterSpec for FIFOCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifocfg::R](R) reader structure"]
impl crate::Readable for FIFOCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifocfg::W](W) writer structure"]
impl crate::Writable for FIFOCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOCFG to value 0x2000_0000"]
impl crate::Resettable for FIFOCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0000
    }
}
