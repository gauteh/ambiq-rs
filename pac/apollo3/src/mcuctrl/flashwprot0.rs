#[doc = "Register `FLASHWPROT0` reader"]
pub struct R(crate::R<FLASHWPROT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHWPROT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHWPROT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHWPROT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHWPROT0` writer"]
pub struct W(crate::W<FLASHWPROT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHWPROT0_SPEC>;
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
impl From<crate::W<FLASHWPROT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHWPROT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FW0BITS` reader - Write protect flash 0x00000000 - 0x0007FFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub struct FW0BITS_R(crate::FieldReader<u32, u32>);
impl FW0BITS_R {
    pub(crate) fn new(bits: u32) -> Self {
        FW0BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FW0BITS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FW0BITS` writer - Write protect flash 0x00000000 - 0x0007FFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub struct FW0BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> FW0BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write protect flash 0x00000000 - 0x0007FFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fw0bits(&self) -> FW0BITS_R {
        FW0BITS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write protect flash 0x00000000 - 0x0007FFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fw0bits(&mut self) -> FW0BITS_W {
        FW0BITS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Write Protection Bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashwprot0](index.html) module"]
pub struct FLASHWPROT0_SPEC;
impl crate::RegisterSpec for FLASHWPROT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashwprot0::R](R) reader structure"]
impl crate::Readable for FLASHWPROT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashwprot0::W](W) writer structure"]
impl crate::Writable for FLASHWPROT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHWPROT0 to value 0"]
impl crate::Resettable for FLASHWPROT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
