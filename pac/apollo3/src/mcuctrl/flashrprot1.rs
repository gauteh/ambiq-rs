#[doc = "Register `FLASHRPROT1` reader"]
pub struct R(crate::R<FLASHRPROT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHRPROT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHRPROT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHRPROT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHRPROT1` writer"]
pub struct W(crate::W<FLASHRPROT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHRPROT1_SPEC>;
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
impl From<crate::W<FLASHRPROT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHRPROT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FR1BITS` reader - Copy (read) protect flash 0x00080000 - 0x000FFFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub struct FR1BITS_R(crate::FieldReader<u32, u32>);
impl FR1BITS_R {
    pub(crate) fn new(bits: u32) -> Self {
        FR1BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FR1BITS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FR1BITS` writer - Copy (read) protect flash 0x00080000 - 0x000FFFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub struct FR1BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> FR1BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Copy (read) protect flash 0x00080000 - 0x000FFFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fr1bits(&self) -> FR1BITS_R {
        FR1BITS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Copy (read) protect flash 0x00080000 - 0x000FFFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fr1bits(&mut self) -> FR1BITS_W {
        FR1BITS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Read Protection Bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashrprot1](index.html) module"]
pub struct FLASHRPROT1_SPEC;
impl crate::RegisterSpec for FLASHRPROT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashrprot1::R](R) reader structure"]
impl crate::Readable for FLASHRPROT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashrprot1::W](W) writer structure"]
impl crate::Writable for FLASHRPROT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHRPROT1 to value 0"]
impl crate::Resettable for FLASHRPROT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
