#[doc = "Register `KEY1` reader"]
pub struct R(crate::R<KEY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY1` writer"]
pub struct W(crate::W<KEY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY1_SPEC>;
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
impl From<crate::W<KEY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY1` reader - Bits \\[63:32\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub struct KEY1_R(crate::FieldReader<u32, u32>);
impl KEY1_R {
    pub(crate) fn new(bits: u32) -> Self {
        KEY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY1` writer - Bits \\[63:32\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub struct KEY1_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bits \\[63:32\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    pub fn key1(&self) -> KEY1_R {
        KEY1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[63:32\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    pub fn key1(&mut self) -> KEY1_W {
        KEY1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1](index.html) module"]
pub struct KEY1_SPEC;
impl crate::RegisterSpec for KEY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key1::R](R) reader structure"]
impl crate::Readable for KEY1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key1::W](W) writer structure"]
impl crate::Writable for KEY1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY1 to value 0"]
impl crate::Resettable for KEY1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
