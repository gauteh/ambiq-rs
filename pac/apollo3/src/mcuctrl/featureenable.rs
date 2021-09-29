#[doc = "Register `FEATUREENABLE` reader"]
pub struct R(crate::R<FEATUREENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEATUREENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEATUREENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEATUREENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEATUREENABLE` writer"]
pub struct W(crate::W<FEATUREENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEATUREENABLE_SPEC>;
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
impl From<crate::W<FEATUREENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEATUREENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Availability of Burst functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTAVAIL_A {
    #[doc = "1: Burst functionality available value."]
    AVAIL = 1,
    #[doc = "0: Burst functionality not available value."]
    NOTAVAIL = 0,
}
impl From<BURSTAVAIL_A> for bool {
    #[inline(always)]
    fn from(variant: BURSTAVAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURSTAVAIL` reader - Availability of Burst functionality"]
pub struct BURSTAVAIL_R(crate::FieldReader<bool, BURSTAVAIL_A>);
impl BURSTAVAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BURSTAVAIL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTAVAIL_A {
        match self.bits {
            true => BURSTAVAIL_A::AVAIL,
            false => BURSTAVAIL_A::NOTAVAIL,
        }
    }
    #[doc = "Checks if the value of the field is `AVAIL`"]
    #[inline(always)]
    pub fn is_avail(&self) -> bool {
        **self == BURSTAVAIL_A::AVAIL
    }
    #[doc = "Checks if the value of the field is `NOTAVAIL`"]
    #[inline(always)]
    pub fn is_notavail(&self) -> bool {
        **self == BURSTAVAIL_A::NOTAVAIL
    }
}
impl core::ops::Deref for BURSTAVAIL_R {
    type Target = crate::FieldReader<bool, BURSTAVAIL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURSTAVAIL` writer - Availability of Burst functionality"]
pub struct BURSTAVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTAVAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTAVAIL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Burst functionality available value."]
    #[inline(always)]
    pub fn avail(self) -> &'a mut W {
        self.variant(BURSTAVAIL_A::AVAIL)
    }
    #[doc = "Burst functionality not available value."]
    #[inline(always)]
    pub fn notavail(self) -> &'a mut W {
        self.variant(BURSTAVAIL_A::NOTAVAIL)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `BURSTACK` reader - ACK for BURSTREQ"]
pub struct BURSTACK_R(crate::FieldReader<bool, bool>);
impl BURSTACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BURSTACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BURSTACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURSTACK` writer - ACK for BURSTREQ"]
pub struct BURSTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Controls the Burst functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTREQ_A {
    #[doc = "1: Enable the Burst functionality value."]
    EN = 1,
    #[doc = "0: Disable the Burst functionality value."]
    DIS = 0,
}
impl From<BURSTREQ_A> for bool {
    #[inline(always)]
    fn from(variant: BURSTREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURSTREQ` reader - Controls the Burst functionality"]
pub struct BURSTREQ_R(crate::FieldReader<bool, BURSTREQ_A>);
impl BURSTREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        BURSTREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTREQ_A {
        match self.bits {
            true => BURSTREQ_A::EN,
            false => BURSTREQ_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == BURSTREQ_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == BURSTREQ_A::DIS
    }
}
impl core::ops::Deref for BURSTREQ_R {
    type Target = crate::FieldReader<bool, BURSTREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURSTREQ` writer - Controls the Burst functionality"]
pub struct BURSTREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTREQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the Burst functionality value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BURSTREQ_A::EN)
    }
    #[doc = "Disable the Burst functionality value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BURSTREQ_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "AVAILABILITY of the BLE functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEAVAIL_A {
    #[doc = "1: BLE functionality available value."]
    AVAIL = 1,
    #[doc = "0: BLE functionality not available value."]
    NOTAVAIL = 0,
}
impl From<BLEAVAIL_A> for bool {
    #[inline(always)]
    fn from(variant: BLEAVAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLEAVAIL` reader - AVAILABILITY of the BLE functionality"]
pub struct BLEAVAIL_R(crate::FieldReader<bool, BLEAVAIL_A>);
impl BLEAVAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEAVAIL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLEAVAIL_A {
        match self.bits {
            true => BLEAVAIL_A::AVAIL,
            false => BLEAVAIL_A::NOTAVAIL,
        }
    }
    #[doc = "Checks if the value of the field is `AVAIL`"]
    #[inline(always)]
    pub fn is_avail(&self) -> bool {
        **self == BLEAVAIL_A::AVAIL
    }
    #[doc = "Checks if the value of the field is `NOTAVAIL`"]
    #[inline(always)]
    pub fn is_notavail(&self) -> bool {
        **self == BLEAVAIL_A::NOTAVAIL
    }
}
impl core::ops::Deref for BLEAVAIL_R {
    type Target = crate::FieldReader<bool, BLEAVAIL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEAVAIL` writer - AVAILABILITY of the BLE functionality"]
pub struct BLEAVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEAVAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEAVAIL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BLE functionality available value."]
    #[inline(always)]
    pub fn avail(self) -> &'a mut W {
        self.variant(BLEAVAIL_A::AVAIL)
    }
    #[doc = "BLE functionality not available value."]
    #[inline(always)]
    pub fn notavail(self) -> &'a mut W {
        self.variant(BLEAVAIL_A::NOTAVAIL)
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
#[doc = "Field `BLEACK` reader - ACK for BLEREQ"]
pub struct BLEACK_R(crate::FieldReader<bool, bool>);
impl BLEACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEACK` writer - ACK for BLEREQ"]
pub struct BLEACK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEACK_W<'a> {
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
#[doc = "Controls the BLE functionality\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEREQ_A {
    #[doc = "1: Enable the BLE functionality value."]
    EN = 1,
    #[doc = "0: Disable the BLE functionality value."]
    DIS = 0,
}
impl From<BLEREQ_A> for bool {
    #[inline(always)]
    fn from(variant: BLEREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLEREQ` reader - Controls the BLE functionality"]
pub struct BLEREQ_R(crate::FieldReader<bool, BLEREQ_A>);
impl BLEREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLEREQ_A {
        match self.bits {
            true => BLEREQ_A::EN,
            false => BLEREQ_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == BLEREQ_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == BLEREQ_A::DIS
    }
}
impl core::ops::Deref for BLEREQ_R {
    type Target = crate::FieldReader<bool, BLEREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEREQ` writer - Controls the BLE functionality"]
pub struct BLEREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEREQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the BLE functionality value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BLEREQ_A::EN)
    }
    #[doc = "Disable the BLE functionality value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BLEREQ_A::DIS)
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
    #[doc = "Bit 6 - Availability of Burst functionality"]
    #[inline(always)]
    pub fn burstavail(&self) -> BURSTAVAIL_R {
        BURSTAVAIL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ACK for BURSTREQ"]
    #[inline(always)]
    pub fn burstack(&self) -> BURSTACK_R {
        BURSTACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Controls the Burst functionality"]
    #[inline(always)]
    pub fn burstreq(&self) -> BURSTREQ_R {
        BURSTREQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AVAILABILITY of the BLE functionality"]
    #[inline(always)]
    pub fn bleavail(&self) -> BLEAVAIL_R {
        BLEAVAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ACK for BLEREQ"]
    #[inline(always)]
    pub fn bleack(&self) -> BLEACK_R {
        BLEACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Controls the BLE functionality"]
    #[inline(always)]
    pub fn blereq(&self) -> BLEREQ_R {
        BLEREQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Availability of Burst functionality"]
    #[inline(always)]
    pub fn burstavail(&mut self) -> BURSTAVAIL_W {
        BURSTAVAIL_W { w: self }
    }
    #[doc = "Bit 5 - ACK for BURSTREQ"]
    #[inline(always)]
    pub fn burstack(&mut self) -> BURSTACK_W {
        BURSTACK_W { w: self }
    }
    #[doc = "Bit 4 - Controls the Burst functionality"]
    #[inline(always)]
    pub fn burstreq(&mut self) -> BURSTREQ_W {
        BURSTREQ_W { w: self }
    }
    #[doc = "Bit 2 - AVAILABILITY of the BLE functionality"]
    #[inline(always)]
    pub fn bleavail(&mut self) -> BLEAVAIL_W {
        BLEAVAIL_W { w: self }
    }
    #[doc = "Bit 1 - ACK for BLEREQ"]
    #[inline(always)]
    pub fn bleack(&mut self) -> BLEACK_W {
        BLEACK_W { w: self }
    }
    #[doc = "Bit 0 - Controls the BLE functionality"]
    #[inline(always)]
    pub fn blereq(&mut self) -> BLEREQ_W {
        BLEREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Feature Enable on Burst and BLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [featureenable](index.html) module"]
pub struct FEATUREENABLE_SPEC;
impl crate::RegisterSpec for FEATUREENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [featureenable::R](R) reader structure"]
impl crate::Readable for FEATUREENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [featureenable::W](W) writer structure"]
impl crate::Writable for FEATUREENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FEATUREENABLE to value 0x01"]
impl crate::Resettable for FEATUREENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
