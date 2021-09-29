#[doc = "Register `FLASHWPROT1` reader"]
pub struct R(crate::R<FLASHWPROT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHWPROT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHWPROT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHWPROT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHWPROT1` writer"]
pub struct W(crate::W<FLASHWPROT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHWPROT1_SPEC>;
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
impl From<crate::W<FLASHWPROT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHWPROT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FW1BITS` reader - Write protect flash 0x00080000 - 0x000FFFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub struct FW1BITS_R(crate::FieldReader<u32, u32>);
impl FW1BITS_R {
    pub(crate) fn new(bits: u32) -> Self {
        FW1BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FW1BITS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FW1BITS` writer - Write protect flash 0x00080000 - 0x000FFFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub struct FW1BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> FW1BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write protect flash 0x00080000 - 0x000FFFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fw1bits(&self) -> FW1BITS_R {
        FW1BITS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write protect flash 0x00080000 - 0x000FFFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fw1bits(&mut self) -> FW1BITS_W {
        FW1BITS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Write Protection Bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashwprot1](index.html) module"]
pub struct FLASHWPROT1_SPEC;
impl crate::RegisterSpec for FLASHWPROT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashwprot1::R](R) reader structure"]
impl crate::Readable for FLASHWPROT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashwprot1::W](W) writer structure"]
impl crate::Writable for FLASHWPROT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHWPROT1 to value 0"]
impl crate::Resettable for FLASHWPROT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
