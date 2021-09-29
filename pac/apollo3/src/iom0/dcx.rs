#[doc = "Register `DCX` reader"]
pub struct R(crate::R<DCX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCX` writer"]
pub struct W(crate::W<DCX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCX_SPEC>;
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
impl From<crate::W<DCX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Revision A: MUST NOT be programmed! Revision B: Bit 4: DCX Signaling Enable via other CE signals. The selected DCX signal (unused CE pin) will be driven low during write of offset byte, and high during transmission of data bytes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCXEN_A {
    #[doc = "1: Enable DCX. value."]
    EN = 1,
    #[doc = "0: Disable DCX. value."]
    DIS = 0,
}
impl From<DCXEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCXEN` reader - Revision A: MUST NOT be programmed! Revision B: Bit 4: DCX Signaling Enable via other CE signals. The selected DCX signal (unused CE pin) will be driven low during write of offset byte, and high during transmission of data bytes."]
pub struct DCXEN_R(crate::FieldReader<bool, DCXEN_A>);
impl DCXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCXEN_A {
        match self.bits {
            true => DCXEN_A::EN,
            false => DCXEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == DCXEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == DCXEN_A::DIS
    }
}
impl core::ops::Deref for DCXEN_R {
    type Target = crate::FieldReader<bool, DCXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCXEN` writer - Revision A: MUST NOT be programmed! Revision B: Bit 4: DCX Signaling Enable via other CE signals. The selected DCX signal (unused CE pin) will be driven low during write of offset byte, and high during transmission of data bytes."]
pub struct DCXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable DCX. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DCXEN_A::EN)
    }
    #[doc = "Disable DCX. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DCXEN_A::DIS)
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
#[doc = "Field `CE3OUT` reader - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE3 output."]
pub struct CE3OUT_R(crate::FieldReader<bool, bool>);
impl CE3OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CE3OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CE3OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CE3OUT` writer - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE3 output."]
pub struct CE3OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CE3OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CE2OUT` reader - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE2 output."]
pub struct CE2OUT_R(crate::FieldReader<bool, bool>);
impl CE2OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CE2OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CE2OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CE2OUT` writer - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE2 output."]
pub struct CE2OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CE2OUT_W<'a> {
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
#[doc = "Field `CE1OUT` reader - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE1 output."]
pub struct CE1OUT_R(crate::FieldReader<bool, bool>);
impl CE1OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CE1OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CE1OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CE1OUT` writer - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE1 output."]
pub struct CE1OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CE1OUT_W<'a> {
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
#[doc = "Field `CE0OUT` reader - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE0 output."]
pub struct CE0OUT_R(crate::FieldReader<bool, bool>);
impl CE0OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CE0OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CE0OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CE0OUT` writer - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE0 output."]
pub struct CE0OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CE0OUT_W<'a> {
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
    #[doc = "Bit 4 - Revision A: MUST NOT be programmed! Revision B: Bit 4: DCX Signaling Enable via other CE signals. The selected DCX signal (unused CE pin) will be driven low during write of offset byte, and high during transmission of data bytes."]
    #[inline(always)]
    pub fn dcxen(&self) -> DCXEN_R {
        DCXEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE3 output."]
    #[inline(always)]
    pub fn ce3out(&self) -> CE3OUT_R {
        CE3OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE2 output."]
    #[inline(always)]
    pub fn ce2out(&self) -> CE2OUT_R {
        CE2OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE1 output."]
    #[inline(always)]
    pub fn ce1out(&self) -> CE1OUT_R {
        CE1OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE0 output."]
    #[inline(always)]
    pub fn ce0out(&self) -> CE0OUT_R {
        CE0OUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Revision A: MUST NOT be programmed! Revision B: Bit 4: DCX Signaling Enable via other CE signals. The selected DCX signal (unused CE pin) will be driven low during write of offset byte, and high during transmission of data bytes."]
    #[inline(always)]
    pub fn dcxen(&mut self) -> DCXEN_W {
        DCXEN_W { w: self }
    }
    #[doc = "Bit 3 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE3 output."]
    #[inline(always)]
    pub fn ce3out(&mut self) -> CE3OUT_W {
        CE3OUT_W { w: self }
    }
    #[doc = "Bit 2 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE2 output."]
    #[inline(always)]
    pub fn ce2out(&mut self) -> CE2OUT_W {
        CE2OUT_W { w: self }
    }
    #[doc = "Bit 1 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE1 output."]
    #[inline(always)]
    pub fn ce1out(&mut self) -> CE1OUT_W {
        CE1OUT_W { w: self }
    }
    #[doc = "Bit 0 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE0 output."]
    #[inline(always)]
    pub fn ce0out(&mut self) -> CE0OUT_W {
        CE0OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcx](index.html) module"]
pub struct DCX_SPEC;
impl crate::RegisterSpec for DCX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcx::R](R) reader structure"]
impl crate::Readable for DCX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcx::W](W) writer structure"]
impl crate::Writable for DCX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCX to value 0"]
impl crate::Resettable for DCX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
