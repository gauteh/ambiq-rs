#[doc = "Register `KEY0` reader"]
pub struct R(crate::R<KEY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY0` writer"]
pub struct W(crate::W<KEY0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY0_SPEC>;
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
impl From<crate::W<KEY0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0` reader - Bits \\[31:0\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub struct KEY0_R(crate::FieldReader<u32, u32>);
impl KEY0_R {
    pub(crate) fn new(bits: u32) -> Self {
        KEY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY0` writer - Bits \\[31:0\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub struct KEY0_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bits \\[31:0\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    pub fn key0(&self) -> KEY0_R {
        KEY0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[31:0\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    pub fn key0(&mut self) -> KEY0_W {
        KEY0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key0](index.html) module"]
pub struct KEY0_SPEC;
impl crate::RegisterSpec for KEY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key0::R](R) reader structure"]
impl crate::Readable for KEY0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key0::W](W) writer structure"]
impl crate::Writable for KEY0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY0 to value 0"]
impl crate::Resettable for KEY0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
