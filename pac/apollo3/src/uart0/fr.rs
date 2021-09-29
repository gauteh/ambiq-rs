#[doc = "Register `FR` reader"]
pub struct R(crate::R<FR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FR` writer"]
pub struct W(crate::W<FR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FR_SPEC>;
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
impl From<crate::W<FR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXBUSY` reader - This bit holds the transmit BUSY indicator."]
pub struct TXBUSY_R(crate::FieldReader<bool, bool>);
impl TXBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBUSY` writer - This bit holds the transmit BUSY indicator."]
pub struct TXBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBUSY_W<'a> {
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
#[doc = "This bit holds the transmit FIFO empty indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFE_A {
    #[doc = "1: Transmit fifo is empty. value."]
    XMTFIFO_EMPTY = 1,
}
impl From<TXFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFE` reader - This bit holds the transmit FIFO empty indicator."]
pub struct TXFE_R(crate::FieldReader<bool, TXFE_A>);
impl TXFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXFE_A> {
        match self.bits {
            true => Some(TXFE_A::XMTFIFO_EMPTY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XMTFIFO_EMPTY`"]
    #[inline(always)]
    pub fn is_xmtfifo_empty(&self) -> bool {
        **self == TXFE_A::XMTFIFO_EMPTY
    }
}
impl core::ops::Deref for TXFE_R {
    type Target = crate::FieldReader<bool, TXFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFE` writer - This bit holds the transmit FIFO empty indicator."]
pub struct TXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit fifo is empty. value."]
    #[inline(always)]
    pub fn xmtfifo_empty(self) -> &'a mut W {
        self.variant(TXFE_A::XMTFIFO_EMPTY)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "This bit holds the receive FIFO full indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFF_A {
    #[doc = "1: Receive fifo is full. value."]
    RCVFIFO_FULL = 1,
}
impl From<RXFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFF` reader - This bit holds the receive FIFO full indicator."]
pub struct RXFF_R(crate::FieldReader<bool, RXFF_A>);
impl RXFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXFF_A> {
        match self.bits {
            true => Some(RXFF_A::RCVFIFO_FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RCVFIFO_FULL`"]
    #[inline(always)]
    pub fn is_rcvfifo_full(&self) -> bool {
        **self == RXFF_A::RCVFIFO_FULL
    }
}
impl core::ops::Deref for RXFF_R {
    type Target = crate::FieldReader<bool, RXFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFF` writer - This bit holds the receive FIFO full indicator."]
pub struct RXFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive fifo is full. value."]
    #[inline(always)]
    pub fn rcvfifo_full(self) -> &'a mut W {
        self.variant(RXFF_A::RCVFIFO_FULL)
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
#[doc = "This bit holds the transmit FIFO full indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFF_A {
    #[doc = "1: Transmit fifo is full. value."]
    XMTFIFO_FULL = 1,
}
impl From<TXFF_A> for bool {
    #[inline(always)]
    fn from(variant: TXFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFF` reader - This bit holds the transmit FIFO full indicator."]
pub struct TXFF_R(crate::FieldReader<bool, TXFF_A>);
impl TXFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXFF_A> {
        match self.bits {
            true => Some(TXFF_A::XMTFIFO_FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XMTFIFO_FULL`"]
    #[inline(always)]
    pub fn is_xmtfifo_full(&self) -> bool {
        **self == TXFF_A::XMTFIFO_FULL
    }
}
impl core::ops::Deref for TXFF_R {
    type Target = crate::FieldReader<bool, TXFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFF` writer - This bit holds the transmit FIFO full indicator."]
pub struct TXFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit fifo is full. value."]
    #[inline(always)]
    pub fn xmtfifo_full(self) -> &'a mut W {
        self.variant(TXFF_A::XMTFIFO_FULL)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "This bit holds the receive FIFO empty indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFE_A {
    #[doc = "1: Receive fifo is empty. value."]
    RCVFIFO_EMPTY = 1,
}
impl From<RXFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFE` reader - This bit holds the receive FIFO empty indicator."]
pub struct RXFE_R(crate::FieldReader<bool, RXFE_A>);
impl RXFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXFE_A> {
        match self.bits {
            true => Some(RXFE_A::RCVFIFO_EMPTY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RCVFIFO_EMPTY`"]
    #[inline(always)]
    pub fn is_rcvfifo_empty(&self) -> bool {
        **self == RXFE_A::RCVFIFO_EMPTY
    }
}
impl core::ops::Deref for RXFE_R {
    type Target = crate::FieldReader<bool, RXFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFE` writer - This bit holds the receive FIFO empty indicator."]
pub struct RXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive fifo is empty. value."]
    #[inline(always)]
    pub fn rcvfifo_empty(self) -> &'a mut W {
        self.variant(RXFE_A::RCVFIFO_EMPTY)
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
#[doc = "This bit holds the busy indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "1: UART busy indicator. value."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - This bit holds the busy indicator."]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUSY_A> {
        match self.bits {
            true => Some(BUSY_A::BUSY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == BUSY_A::BUSY
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` writer - This bit holds the busy indicator."]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART busy indicator. value."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(BUSY_A::BUSY)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "This bit holds the data carrier detect indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCD_A {
    #[doc = "1: Data carrier detect detected. value."]
    DETECTED = 1,
}
impl From<DCD_A> for bool {
    #[inline(always)]
    fn from(variant: DCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCD` reader - This bit holds the data carrier detect indicator."]
pub struct DCD_R(crate::FieldReader<bool, DCD_A>);
impl DCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCD_A> {
        match self.bits {
            true => Some(DCD_A::DETECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == DCD_A::DETECTED
    }
}
impl core::ops::Deref for DCD_R {
    type Target = crate::FieldReader<bool, DCD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCD` writer - This bit holds the data carrier detect indicator."]
pub struct DCD_W<'a> {
    w: &'a mut W,
}
impl<'a> DCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data carrier detect detected. value."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DCD_A::DETECTED)
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
#[doc = "This bit holds the data set ready indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSR_A {
    #[doc = "1: Data set ready. value."]
    READY = 1,
}
impl From<DSR_A> for bool {
    #[inline(always)]
    fn from(variant: DSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSR` reader - This bit holds the data set ready indicator."]
pub struct DSR_R(crate::FieldReader<bool, DSR_A>);
impl DSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSR_A> {
        match self.bits {
            true => Some(DSR_A::READY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == DSR_A::READY
    }
}
impl core::ops::Deref for DSR_R {
    type Target = crate::FieldReader<bool, DSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSR` writer - This bit holds the data set ready indicator."]
pub struct DSR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data set ready. value."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(DSR_A::READY)
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
#[doc = "This bit holds the clear to send indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_A {
    #[doc = "1: Clear to send is indicated. value."]
    CLEARTOSEND = 1,
}
impl From<CTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - This bit holds the clear to send indicator."]
pub struct CTS_R(crate::FieldReader<bool, CTS_A>);
impl CTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTS_A> {
        match self.bits {
            true => Some(CTS_A::CLEARTOSEND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEARTOSEND`"]
    #[inline(always)]
    pub fn is_cleartosend(&self) -> bool {
        **self == CTS_A::CLEARTOSEND
    }
}
impl core::ops::Deref for CTS_R {
    type Target = crate::FieldReader<bool, CTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTS` writer - This bit holds the clear to send indicator."]
pub struct CTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear to send is indicated. value."]
    #[inline(always)]
    pub fn cleartosend(self) -> &'a mut W {
        self.variant(CTS_A::CLEARTOSEND)
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
    #[doc = "Bit 8 - This bit holds the transmit BUSY indicator."]
    #[inline(always)]
    pub fn txbusy(&self) -> TXBUSY_R {
        TXBUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit holds the transmit FIFO empty indicator."]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit holds the receive FIFO full indicator."]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit holds the transmit FIFO full indicator."]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit holds the receive FIFO empty indicator."]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit holds the busy indicator."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit holds the data carrier detect indicator."]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit holds the data set ready indicator."]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit holds the clear to send indicator."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - This bit holds the transmit BUSY indicator."]
    #[inline(always)]
    pub fn txbusy(&mut self) -> TXBUSY_W {
        TXBUSY_W { w: self }
    }
    #[doc = "Bit 7 - This bit holds the transmit FIFO empty indicator."]
    #[inline(always)]
    pub fn txfe(&mut self) -> TXFE_W {
        TXFE_W { w: self }
    }
    #[doc = "Bit 6 - This bit holds the receive FIFO full indicator."]
    #[inline(always)]
    pub fn rxff(&mut self) -> RXFF_W {
        RXFF_W { w: self }
    }
    #[doc = "Bit 5 - This bit holds the transmit FIFO full indicator."]
    #[inline(always)]
    pub fn txff(&mut self) -> TXFF_W {
        TXFF_W { w: self }
    }
    #[doc = "Bit 4 - This bit holds the receive FIFO empty indicator."]
    #[inline(always)]
    pub fn rxfe(&mut self) -> RXFE_W {
        RXFE_W { w: self }
    }
    #[doc = "Bit 3 - This bit holds the busy indicator."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bit 2 - This bit holds the data carrier detect indicator."]
    #[inline(always)]
    pub fn dcd(&mut self) -> DCD_W {
        DCD_W { w: self }
    }
    #[doc = "Bit 1 - This bit holds the data set ready indicator."]
    #[inline(always)]
    pub fn dsr(&mut self) -> DSR_W {
        DSR_W { w: self }
    }
    #[doc = "Bit 0 - This bit holds the clear to send indicator."]
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W {
        CTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](index.html) module"]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fr::R](R) reader structure"]
impl crate::Readable for FR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fr::W](W) writer structure"]
impl crate::Writable for FR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
