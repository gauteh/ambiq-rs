#[doc = "Register `SKU` reader"]
pub struct R(crate::R<SKU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SKU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SKU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SKU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SKU` writer"]
pub struct W(crate::W<SKU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SKU_SPEC>;
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
impl From<crate::W<SKU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SKU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECBOOT` reader - Secure boot feature allowed"]
pub struct SECBOOT_R(crate::FieldReader<bool, bool>);
impl SECBOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECBOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECBOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECBOOT` writer - Secure boot feature allowed"]
pub struct SECBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> SECBOOT_W<'a> {
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
#[doc = "Field `ALLOWBLE` reader - Allow BLE feature"]
pub struct ALLOWBLE_R(crate::FieldReader<bool, bool>);
impl ALLOWBLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALLOWBLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALLOWBLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALLOWBLE` writer - Allow BLE feature"]
pub struct ALLOWBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLOWBLE_W<'a> {
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
#[doc = "Field `ALLOWBURST` reader - Allow Burst feature"]
pub struct ALLOWBURST_R(crate::FieldReader<bool, bool>);
impl ALLOWBURST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALLOWBURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALLOWBURST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALLOWBURST` writer - Allow Burst feature"]
pub struct ALLOWBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLOWBURST_W<'a> {
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
    #[doc = "Bit 2 - Secure boot feature allowed"]
    #[inline(always)]
    pub fn secboot(&self) -> SECBOOT_R {
        SECBOOT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Allow BLE feature"]
    #[inline(always)]
    pub fn allowble(&self) -> ALLOWBLE_R {
        ALLOWBLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Allow Burst feature"]
    #[inline(always)]
    pub fn allowburst(&self) -> ALLOWBURST_R {
        ALLOWBURST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Secure boot feature allowed"]
    #[inline(always)]
    pub fn secboot(&mut self) -> SECBOOT_W {
        SECBOOT_W { w: self }
    }
    #[doc = "Bit 1 - Allow BLE feature"]
    #[inline(always)]
    pub fn allowble(&mut self) -> ALLOWBLE_W {
        ALLOWBLE_W { w: self }
    }
    #[doc = "Bit 0 - Allow Burst feature"]
    #[inline(always)]
    pub fn allowburst(&mut self) -> ALLOWBURST_W {
        ALLOWBURST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Unique Chip SKU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sku](index.html) module"]
pub struct SKU_SPEC;
impl crate::RegisterSpec for SKU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sku::R](R) reader structure"]
impl crate::Readable for SKU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sku::W](W) writer structure"]
impl crate::Writable for SKU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SKU to value 0"]
impl crate::Resettable for SKU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
