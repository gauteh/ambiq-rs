#[doc = "Register `SHADOWVALID` reader"]
pub struct R(crate::R<SHADOWVALID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHADOWVALID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHADOWVALID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHADOWVALID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHADOWVALID` writer"]
pub struct W(crate::W<SHADOWVALID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHADOWVALID_SPEC>;
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
impl From<crate::W<SHADOWVALID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHADOWVALID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Indicates whether info0 contains valid data\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INFO0_VALID_A {
    #[doc = "1: Flash info0 (customer) space contains valid data. value."]
    VALID = 1,
}
impl From<INFO0_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: INFO0_VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INFO0_VALID` reader - Indicates whether info0 contains valid data"]
pub struct INFO0_VALID_R(crate::FieldReader<bool, INFO0_VALID_A>);
impl INFO0_VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        INFO0_VALID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INFO0_VALID_A> {
        match self.bits {
            true => Some(INFO0_VALID_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == INFO0_VALID_A::VALID
    }
}
impl core::ops::Deref for INFO0_VALID_R {
    type Target = crate::FieldReader<bool, INFO0_VALID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFO0_VALID` writer - Indicates whether info0 contains valid data"]
pub struct INFO0_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> INFO0_VALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INFO0_VALID_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash info0 (customer) space contains valid data. value."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(INFO0_VALID_A::VALID)
    }
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
#[doc = "Indicates whether the bootloader should sleep or deep sleep if no image loaded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLDSLEEP_A {
    #[doc = "1: Bootloader will go to deep sleep if no flash image loaded value."]
    DEEPSLEEP = 1,
}
impl From<BLDSLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: BLDSLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLDSLEEP` reader - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
pub struct BLDSLEEP_R(crate::FieldReader<bool, BLDSLEEP_A>);
impl BLDSLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLDSLEEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLDSLEEP_A> {
        match self.bits {
            true => Some(BLDSLEEP_A::DEEPSLEEP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        **self == BLDSLEEP_A::DEEPSLEEP
    }
}
impl core::ops::Deref for BLDSLEEP_R {
    type Target = crate::FieldReader<bool, BLDSLEEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLDSLEEP` writer - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
pub struct BLDSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> BLDSLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLDSLEEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bootloader will go to deep sleep if no flash image loaded value."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(BLDSLEEP_A::DEEPSLEEP)
    }
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
#[doc = "Indicates whether the shadow registers contain valid data from the Flash Information Space.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALID_A {
    #[doc = "1: Flash information space contains valid data. value."]
    VALID = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALID` reader - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
pub struct VALID_R(crate::FieldReader<bool, VALID_A>);
impl VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VALID_A> {
        match self.bits {
            true => Some(VALID_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == VALID_A::VALID
    }
}
impl core::ops::Deref for VALID_R {
    type Target = crate::FieldReader<bool, VALID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALID` writer - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VALID_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash information space contains valid data. value."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VALID_A::VALID)
    }
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
    #[doc = "Bit 2 - Indicates whether info0 contains valid data"]
    #[inline(always)]
    pub fn info0_valid(&self) -> INFO0_VALID_R {
        INFO0_VALID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[inline(always)]
    pub fn bldsleep(&self) -> BLDSLEEP_R {
        BLDSLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Indicates whether info0 contains valid data"]
    #[inline(always)]
    pub fn info0_valid(&mut self) -> INFO0_VALID_W {
        INFO0_VALID_W { w: self }
    }
    #[doc = "Bit 1 - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[inline(always)]
    pub fn bldsleep(&mut self) -> BLDSLEEP_W {
        BLDSLEEP_W { w: self }
    }
    #[doc = "Bit 0 - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shadowvalid](index.html) module"]
pub struct SHADOWVALID_SPEC;
impl crate::RegisterSpec for SHADOWVALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shadowvalid::R](R) reader structure"]
impl crate::Readable for SHADOWVALID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shadowvalid::W](W) writer structure"]
impl crate::Writable for SHADOWVALID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHADOWVALID to value 0x07"]
impl crate::Resettable for SHADOWVALID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
