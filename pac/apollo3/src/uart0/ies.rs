#[doc = "Register `IES` reader"]
pub struct R(crate::R<IES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IES` writer"]
pub struct W(crate::W<IES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IES_SPEC>;
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
impl From<crate::W<IES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OERIS` reader - This bit holds the overflow interrupt status."]
pub struct OERIS_R(crate::FieldReader<bool, bool>);
impl OERIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OERIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OERIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OERIS` writer - This bit holds the overflow interrupt status."]
pub struct OERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OERIS_W<'a> {
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
#[doc = "Field `BERIS` reader - This bit holds the break error interrupt status."]
pub struct BERIS_R(crate::FieldReader<bool, bool>);
impl BERIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BERIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BERIS` writer - This bit holds the break error interrupt status."]
pub struct BERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BERIS_W<'a> {
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
#[doc = "Field `PERIS` reader - This bit holds the parity error interrupt status."]
pub struct PERIS_R(crate::FieldReader<bool, bool>);
impl PERIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIS` writer - This bit holds the parity error interrupt status."]
pub struct PERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIS_W<'a> {
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
#[doc = "Field `FERIS` reader - This bit holds the framing error interrupt status."]
pub struct FERIS_R(crate::FieldReader<bool, bool>);
impl FERIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FERIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FERIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FERIS` writer - This bit holds the framing error interrupt status."]
pub struct FERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FERIS_W<'a> {
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
#[doc = "Field `RTRIS` reader - This bit holds the receive timeout interrupt status."]
pub struct RTRIS_R(crate::FieldReader<bool, bool>);
impl RTRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTRIS` writer - This bit holds the receive timeout interrupt status."]
pub struct RTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTRIS_W<'a> {
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
#[doc = "Field `TXRIS` reader - This bit holds the transmit interrupt status."]
pub struct TXRIS_R(crate::FieldReader<bool, bool>);
impl TXRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRIS` writer - This bit holds the transmit interrupt status."]
pub struct TXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRIS_W<'a> {
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
#[doc = "Field `RXRIS` reader - This bit holds the receive interrupt status."]
pub struct RXRIS_R(crate::FieldReader<bool, bool>);
impl RXRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRIS` writer - This bit holds the receive interrupt status."]
pub struct RXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRIS_W<'a> {
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
#[doc = "Field `DSRMRIS` reader - This bit holds the modem DSR interrupt status."]
pub struct DSRMRIS_R(crate::FieldReader<bool, bool>);
impl DSRMRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSRMRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRMRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRMRIS` writer - This bit holds the modem DSR interrupt status."]
pub struct DSRMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMRIS_W<'a> {
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
#[doc = "Field `DCDMRIS` reader - This bit holds the modem DCD interrupt status."]
pub struct DCDMRIS_R(crate::FieldReader<bool, bool>);
impl DCDMRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDMRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDMRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDMRIS` writer - This bit holds the modem DCD interrupt status."]
pub struct DCDMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMRIS_W<'a> {
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
#[doc = "Field `CTSMRIS` reader - This bit holds the modem CTS interrupt status."]
pub struct CTSMRIS_R(crate::FieldReader<bool, bool>);
impl CTSMRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSMRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSMRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSMRIS` writer - This bit holds the modem CTS interrupt status."]
pub struct CTSMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMRIS_W<'a> {
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
#[doc = "Field `TXCMPMRIS` reader - This bit holds the modem TXCMP interrupt status."]
pub struct TXCMPMRIS_R(crate::FieldReader<bool, bool>);
impl TXCMPMRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXCMPMRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCMPMRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCMPMRIS` writer - This bit holds the modem TXCMP interrupt status."]
pub struct TXCMPMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCMPMRIS_W<'a> {
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
    #[doc = "Bit 10 - This bit holds the overflow interrupt status."]
    #[inline(always)]
    pub fn oeris(&self) -> OERIS_R {
        OERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt status."]
    #[inline(always)]
    pub fn beris(&self) -> BERIS_R {
        BERIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt status."]
    #[inline(always)]
    pub fn peris(&self) -> PERIS_R {
        PERIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt status."]
    #[inline(always)]
    pub fn feris(&self) -> FERIS_R {
        FERIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt status."]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt status."]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt status."]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit holds the modem DSR interrupt status."]
    #[inline(always)]
    pub fn dsrmris(&self) -> DSRMRIS_R {
        DSRMRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit holds the modem DCD interrupt status."]
    #[inline(always)]
    pub fn dcdmris(&self) -> DCDMRIS_R {
        DCDMRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit holds the modem CTS interrupt status."]
    #[inline(always)]
    pub fn ctsmris(&self) -> CTSMRIS_R {
        CTSMRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit holds the modem TXCMP interrupt status."]
    #[inline(always)]
    pub fn txcmpmris(&self) -> TXCMPMRIS_R {
        TXCMPMRIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - This bit holds the overflow interrupt status."]
    #[inline(always)]
    pub fn oeris(&mut self) -> OERIS_W {
        OERIS_W { w: self }
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt status."]
    #[inline(always)]
    pub fn beris(&mut self) -> BERIS_W {
        BERIS_W { w: self }
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt status."]
    #[inline(always)]
    pub fn peris(&mut self) -> PERIS_W {
        PERIS_W { w: self }
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt status."]
    #[inline(always)]
    pub fn feris(&mut self) -> FERIS_W {
        FERIS_W { w: self }
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt status."]
    #[inline(always)]
    pub fn rtris(&mut self) -> RTRIS_W {
        RTRIS_W { w: self }
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt status."]
    #[inline(always)]
    pub fn txris(&mut self) -> TXRIS_W {
        TXRIS_W { w: self }
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt status."]
    #[inline(always)]
    pub fn rxris(&mut self) -> RXRIS_W {
        RXRIS_W { w: self }
    }
    #[doc = "Bit 3 - This bit holds the modem DSR interrupt status."]
    #[inline(always)]
    pub fn dsrmris(&mut self) -> DSRMRIS_W {
        DSRMRIS_W { w: self }
    }
    #[doc = "Bit 2 - This bit holds the modem DCD interrupt status."]
    #[inline(always)]
    pub fn dcdmris(&mut self) -> DCDMRIS_W {
        DCDMRIS_W { w: self }
    }
    #[doc = "Bit 1 - This bit holds the modem CTS interrupt status."]
    #[inline(always)]
    pub fn ctsmris(&mut self) -> CTSMRIS_W {
        CTSMRIS_W { w: self }
    }
    #[doc = "Bit 0 - This bit holds the modem TXCMP interrupt status."]
    #[inline(always)]
    pub fn txcmpmris(&mut self) -> TXCMPMRIS_W {
        TXCMPMRIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ies](index.html) module"]
pub struct IES_SPEC;
impl crate::RegisterSpec for IES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ies::R](R) reader structure"]
impl crate::Readable for IES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ies::W](W) writer structure"]
impl crate::Writable for IES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IES to value 0"]
impl crate::Resettable for IES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
