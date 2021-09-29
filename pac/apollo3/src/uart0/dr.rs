#[doc = "Register `DR` reader"]
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR` writer"]
pub struct W(crate::W<DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SPEC>;
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
impl From<crate::W<DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "This is the overrun error indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OEDATA_A {
    #[doc = "0: No error on UART OEDATA, overrun error indicator. value."]
    NOERR = 0,
    #[doc = "1: Error on UART OEDATA, overrun error indicator. value."]
    ERR = 1,
}
impl From<OEDATA_A> for bool {
    #[inline(always)]
    fn from(variant: OEDATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OEDATA` reader - This is the overrun error indicator."]
pub struct OEDATA_R(crate::FieldReader<bool, OEDATA_A>);
impl OEDATA_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEDATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OEDATA_A {
        match self.bits {
            false => OEDATA_A::NOERR,
            true => OEDATA_A::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        **self == OEDATA_A::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        **self == OEDATA_A::ERR
    }
}
impl core::ops::Deref for OEDATA_R {
    type Target = crate::FieldReader<bool, OEDATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEDATA` writer - This is the overrun error indicator."]
pub struct OEDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> OEDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OEDATA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No error on UART OEDATA, overrun error indicator. value."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut W {
        self.variant(OEDATA_A::NOERR)
    }
    #[doc = "Error on UART OEDATA, overrun error indicator. value."]
    #[inline(always)]
    pub fn err(self) -> &'a mut W {
        self.variant(OEDATA_A::ERR)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "This is the break error indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEDATA_A {
    #[doc = "0: No error on UART BEDATA, break error indicator. value."]
    NOERR = 0,
    #[doc = "1: Error on UART BEDATA, break error indicator. value."]
    ERR = 1,
}
impl From<BEDATA_A> for bool {
    #[inline(always)]
    fn from(variant: BEDATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEDATA` reader - This is the break error indicator."]
pub struct BEDATA_R(crate::FieldReader<bool, BEDATA_A>);
impl BEDATA_R {
    pub(crate) fn new(bits: bool) -> Self {
        BEDATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEDATA_A {
        match self.bits {
            false => BEDATA_A::NOERR,
            true => BEDATA_A::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        **self == BEDATA_A::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        **self == BEDATA_A::ERR
    }
}
impl core::ops::Deref for BEDATA_R {
    type Target = crate::FieldReader<bool, BEDATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEDATA` writer - This is the break error indicator."]
pub struct BEDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> BEDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BEDATA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No error on UART BEDATA, break error indicator. value."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut W {
        self.variant(BEDATA_A::NOERR)
    }
    #[doc = "Error on UART BEDATA, break error indicator. value."]
    #[inline(always)]
    pub fn err(self) -> &'a mut W {
        self.variant(BEDATA_A::ERR)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "This is the parity error indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEDATA_A {
    #[doc = "0: No error on UART PEDATA, parity error indicator. value."]
    NOERR = 0,
    #[doc = "1: Error on UART PEDATA, parity error indicator. value."]
    ERR = 1,
}
impl From<PEDATA_A> for bool {
    #[inline(always)]
    fn from(variant: PEDATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEDATA` reader - This is the parity error indicator."]
pub struct PEDATA_R(crate::FieldReader<bool, PEDATA_A>);
impl PEDATA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEDATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEDATA_A {
        match self.bits {
            false => PEDATA_A::NOERR,
            true => PEDATA_A::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        **self == PEDATA_A::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        **self == PEDATA_A::ERR
    }
}
impl core::ops::Deref for PEDATA_R {
    type Target = crate::FieldReader<bool, PEDATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEDATA` writer - This is the parity error indicator."]
pub struct PEDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PEDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEDATA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No error on UART PEDATA, parity error indicator. value."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut W {
        self.variant(PEDATA_A::NOERR)
    }
    #[doc = "Error on UART PEDATA, parity error indicator. value."]
    #[inline(always)]
    pub fn err(self) -> &'a mut W {
        self.variant(PEDATA_A::ERR)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "This is the framing error indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEDATA_A {
    #[doc = "0: No error on UART FEDATA, framing error indicator. value."]
    NOERR = 0,
    #[doc = "1: Error on UART FEDATA, framing error indicator. value."]
    ERR = 1,
}
impl From<FEDATA_A> for bool {
    #[inline(always)]
    fn from(variant: FEDATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEDATA` reader - This is the framing error indicator."]
pub struct FEDATA_R(crate::FieldReader<bool, FEDATA_A>);
impl FEDATA_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEDATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEDATA_A {
        match self.bits {
            false => FEDATA_A::NOERR,
            true => FEDATA_A::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        **self == FEDATA_A::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        **self == FEDATA_A::ERR
    }
}
impl core::ops::Deref for FEDATA_R {
    type Target = crate::FieldReader<bool, FEDATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEDATA` writer - This is the framing error indicator."]
pub struct FEDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> FEDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEDATA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No error on UART FEDATA, framing error indicator. value."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut W {
        self.variant(FEDATA_A::NOERR)
    }
    #[doc = "Error on UART FEDATA, framing error indicator. value."]
    #[inline(always)]
    pub fn err(self) -> &'a mut W {
        self.variant(FEDATA_A::ERR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DATA` reader - This is the UART data port."]
pub struct DATA_R(crate::FieldReader<u8, u8>);
impl DATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - This is the UART data port."]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - This is the overrun error indicator."]
    #[inline(always)]
    pub fn oedata(&self) -> OEDATA_R {
        OEDATA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This is the break error indicator."]
    #[inline(always)]
    pub fn bedata(&self) -> BEDATA_R {
        BEDATA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This is the parity error indicator."]
    #[inline(always)]
    pub fn pedata(&self) -> PEDATA_R {
        PEDATA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This is the framing error indicator."]
    #[inline(always)]
    pub fn fedata(&self) -> FEDATA_R {
        FEDATA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - This is the UART data port."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - This is the overrun error indicator."]
    #[inline(always)]
    pub fn oedata(&mut self) -> OEDATA_W {
        OEDATA_W { w: self }
    }
    #[doc = "Bit 10 - This is the break error indicator."]
    #[inline(always)]
    pub fn bedata(&mut self) -> BEDATA_W {
        BEDATA_W { w: self }
    }
    #[doc = "Bit 9 - This is the parity error indicator."]
    #[inline(always)]
    pub fn pedata(&mut self) -> PEDATA_W {
        PEDATA_W { w: self }
    }
    #[doc = "Bit 8 - This is the framing error indicator."]
    #[inline(always)]
    pub fn fedata(&mut self) -> FEDATA_W {
        FEDATA_W { w: self }
    }
    #[doc = "Bits 0:7 - This is the UART data port."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](index.html) module"]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr::R](R) reader structure"]
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr::W](W) writer structure"]
impl crate::Writable for DR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
