#[doc = "Register `IEC` reader"]
pub struct R(crate::R<IEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEC` writer"]
pub struct W(crate::W<IEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEC_SPEC>;
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
impl From<crate::W<IEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OEIC` reader - This bit holds the overflow interrupt clear."]
pub struct OEIC_R(crate::FieldReader<bool, bool>);
impl OEIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEIC` writer - This bit holds the overflow interrupt clear."]
pub struct OEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIC_W<'a> {
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
#[doc = "Field `BEIC` reader - This bit holds the break error interrupt clear."]
pub struct BEIC_R(crate::FieldReader<bool, bool>);
impl BEIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        BEIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEIC` writer - This bit holds the break error interrupt clear."]
pub struct BEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIC_W<'a> {
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
#[doc = "Field `PEIC` reader - This bit holds the parity error interrupt clear."]
pub struct PEIC_R(crate::FieldReader<bool, bool>);
impl PEIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEIC` writer - This bit holds the parity error interrupt clear."]
pub struct PEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIC_W<'a> {
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
#[doc = "Field `FEIC` reader - This bit holds the framing error interrupt clear."]
pub struct FEIC_R(crate::FieldReader<bool, bool>);
impl FEIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIC` writer - This bit holds the framing error interrupt clear."]
pub struct FEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RTIC` reader - This bit holds the receive timeout interrupt clear."]
pub struct RTIC_R(crate::FieldReader<bool, bool>);
impl RTIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTIC` writer - This bit holds the receive timeout interrupt clear."]
pub struct RTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIC_W<'a> {
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
#[doc = "Field `TXIC` reader - This bit holds the transmit interrupt clear."]
pub struct TXIC_R(crate::FieldReader<bool, bool>);
impl TXIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIC` writer - This bit holds the transmit interrupt clear."]
pub struct TXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIC_W<'a> {
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
#[doc = "Field `RXIC` reader - This bit holds the receive interrupt clear."]
pub struct RXIC_R(crate::FieldReader<bool, bool>);
impl RXIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIC` writer - This bit holds the receive interrupt clear."]
pub struct RXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIC_W<'a> {
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
#[doc = "Field `DSRMIC` reader - This bit holds the modem DSR interrupt clear."]
pub struct DSRMIC_R(crate::FieldReader<bool, bool>);
impl DSRMIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSRMIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRMIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRMIC` writer - This bit holds the modem DSR interrupt clear."]
pub struct DSRMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMIC_W<'a> {
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
#[doc = "Field `DCDMIC` reader - This bit holds the modem DCD interrupt clear."]
pub struct DCDMIC_R(crate::FieldReader<bool, bool>);
impl DCDMIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDMIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDMIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDMIC` writer - This bit holds the modem DCD interrupt clear."]
pub struct DCDMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMIC_W<'a> {
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
#[doc = "Field `CTSMIC` reader - This bit holds the modem CTS interrupt clear."]
pub struct CTSMIC_R(crate::FieldReader<bool, bool>);
impl CTSMIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSMIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSMIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSMIC` writer - This bit holds the modem CTS interrupt clear."]
pub struct CTSMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMIC_W<'a> {
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
#[doc = "Field `TXCMPMIC` reader - This bit holds the modem TXCMP interrupt clear."]
pub struct TXCMPMIC_R(crate::FieldReader<bool, bool>);
impl TXCMPMIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXCMPMIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCMPMIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCMPMIC` writer - This bit holds the modem TXCMP interrupt clear."]
pub struct TXCMPMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCMPMIC_W<'a> {
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
    #[doc = "Bit 10 - This bit holds the overflow interrupt clear."]
    #[inline(always)]
    pub fn oeic(&self) -> OEIC_R {
        OEIC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt clear."]
    #[inline(always)]
    pub fn beic(&self) -> BEIC_R {
        BEIC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt clear."]
    #[inline(always)]
    pub fn peic(&self) -> PEIC_R {
        PEIC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt clear."]
    #[inline(always)]
    pub fn feic(&self) -> FEIC_R {
        FEIC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt clear."]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt clear."]
    #[inline(always)]
    pub fn txic(&self) -> TXIC_R {
        TXIC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt clear."]
    #[inline(always)]
    pub fn rxic(&self) -> RXIC_R {
        RXIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit holds the modem DSR interrupt clear."]
    #[inline(always)]
    pub fn dsrmic(&self) -> DSRMIC_R {
        DSRMIC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit holds the modem DCD interrupt clear."]
    #[inline(always)]
    pub fn dcdmic(&self) -> DCDMIC_R {
        DCDMIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit holds the modem CTS interrupt clear."]
    #[inline(always)]
    pub fn ctsmic(&self) -> CTSMIC_R {
        CTSMIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit holds the modem TXCMP interrupt clear."]
    #[inline(always)]
    pub fn txcmpmic(&self) -> TXCMPMIC_R {
        TXCMPMIC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - This bit holds the overflow interrupt clear."]
    #[inline(always)]
    pub fn oeic(&mut self) -> OEIC_W {
        OEIC_W { w: self }
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt clear."]
    #[inline(always)]
    pub fn beic(&mut self) -> BEIC_W {
        BEIC_W { w: self }
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt clear."]
    #[inline(always)]
    pub fn peic(&mut self) -> PEIC_W {
        PEIC_W { w: self }
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt clear."]
    #[inline(always)]
    pub fn feic(&mut self) -> FEIC_W {
        FEIC_W { w: self }
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt clear."]
    #[inline(always)]
    pub fn rtic(&mut self) -> RTIC_W {
        RTIC_W { w: self }
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt clear."]
    #[inline(always)]
    pub fn txic(&mut self) -> TXIC_W {
        TXIC_W { w: self }
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt clear."]
    #[inline(always)]
    pub fn rxic(&mut self) -> RXIC_W {
        RXIC_W { w: self }
    }
    #[doc = "Bit 3 - This bit holds the modem DSR interrupt clear."]
    #[inline(always)]
    pub fn dsrmic(&mut self) -> DSRMIC_W {
        DSRMIC_W { w: self }
    }
    #[doc = "Bit 2 - This bit holds the modem DCD interrupt clear."]
    #[inline(always)]
    pub fn dcdmic(&mut self) -> DCDMIC_W {
        DCDMIC_W { w: self }
    }
    #[doc = "Bit 1 - This bit holds the modem CTS interrupt clear."]
    #[inline(always)]
    pub fn ctsmic(&mut self) -> CTSMIC_W {
        CTSMIC_W { w: self }
    }
    #[doc = "Bit 0 - This bit holds the modem TXCMP interrupt clear."]
    #[inline(always)]
    pub fn txcmpmic(&mut self) -> TXCMPMIC_W {
        TXCMPMIC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec](index.html) module"]
pub struct IEC_SPEC;
impl crate::RegisterSpec for IEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iec::R](R) reader structure"]
impl crate::Readable for IEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iec::W](W) writer structure"]
impl crate::Writable for IEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEC to value 0"]
impl crate::Resettable for IEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
