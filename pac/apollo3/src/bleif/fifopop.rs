#[doc = "Register `FIFOPOP` reader"]
pub struct R(crate::R<FIFOPOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOPOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOPOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOPOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOPOP` writer"]
pub struct W(crate::W<FIFOPOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOPOP_SPEC>;
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
impl From<crate::W<FIFOPOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOPOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFODOUT` reader - This register will return the read data indicated by the current read pointer on reads. If the POPWR control bit in the FIFOCTRL register is reset (0), the fifo read pointer will be advanced by one word as a result of the read. If the POPWR bit is set (1), the fifo read pointer will only be advanced after a write operation to this register. The write data is ignored for this register. If less than a even word multiple is available, and the command is completed, the module will return the word containing these bytes and undetermined data in the unused fields of the word."]
pub struct FIFODOUT_R(crate::FieldReader<u32, u32>);
impl FIFODOUT_R {
    pub(crate) fn new(bits: u32) -> Self {
        FIFODOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFODOUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFODOUT` writer - This register will return the read data indicated by the current read pointer on reads. If the POPWR control bit in the FIFOCTRL register is reset (0), the fifo read pointer will be advanced by one word as a result of the read. If the POPWR bit is set (1), the fifo read pointer will only be advanced after a write operation to this register. The write data is ignored for this register. If less than a even word multiple is available, and the command is completed, the module will return the word containing these bytes and undetermined data in the unused fields of the word."]
pub struct FIFODOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFODOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register will return the read data indicated by the current read pointer on reads. If the POPWR control bit in the FIFOCTRL register is reset (0), the fifo read pointer will be advanced by one word as a result of the read. If the POPWR bit is set (1), the fifo read pointer will only be advanced after a write operation to this register. The write data is ignored for this register. If less than a even word multiple is available, and the command is completed, the module will return the word containing these bytes and undetermined data in the unused fields of the word."]
    #[inline(always)]
    pub fn fifodout(&self) -> FIFODOUT_R {
        FIFODOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register will return the read data indicated by the current read pointer on reads. If the POPWR control bit in the FIFOCTRL register is reset (0), the fifo read pointer will be advanced by one word as a result of the read. If the POPWR bit is set (1), the fifo read pointer will only be advanced after a write operation to this register. The write data is ignored for this register. If less than a even word multiple is available, and the command is completed, the module will return the word containing these bytes and undetermined data in the unused fields of the word."]
    #[inline(always)]
    pub fn fifodout(&mut self) -> FIFODOUT_W {
        FIFODOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO POP register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifopop](index.html) module"]
pub struct FIFOPOP_SPEC;
impl crate::RegisterSpec for FIFOPOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifopop::R](R) reader structure"]
impl crate::Readable for FIFOPOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifopop::W](W) writer structure"]
impl crate::Writable for FIFOPOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOPOP to value 0"]
impl crate::Resettable for FIFOPOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
